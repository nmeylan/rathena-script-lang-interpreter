use std::{io, mem};


use std::sync::{Arc};
use std::io::{Stdout, Write};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use crate::lang::array::Array;


use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Constant, Native, Scope, ValueRef, ValueType, Variable};
use crate::lang::vm::{DebugFlag, Hashcode, NATIVE_FUNCTIONS, NativeMethodHandler, Vm};
use crate::lang::value::Value;
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{*};
use crate::lang::class::{Class, Function, Instance};
use crate::lang::compiler::CompilationDetail;
use crate::lang::error::{RuntimeError, TemporaryRuntimeError};

use crate::lang::native::handle_native_method;
use crate::lang::stack::StackEntry::{ArrayHeapReference, ConstantPoolReference, HeapReference};
use crate::lang::stack_trace::StackTrace;

pub enum CallFrameBreak {
    Return(bool),
    Goto(usize),
    End,
}

pub struct Thread {
    pub(crate) debug_flag: u16,
    pub vm: Arc<Vm>,
    pub(crate) stack: Stack,
    pub(crate) current_source_line: CompilationDetail,
    pub(crate) stack_traces: Vec<StackTrace>,
    pub aborted: AtomicBool,
}

impl Thread {
    pub fn new(vm: Arc<Vm>, debug_flag: u16) -> Self {
        let stack = Stack::new();
        Self {
            debug_flag,
            vm,
            stack,
            current_source_line: CompilationDetail::new_empty(),
            stack_traces: vec![],
            aborted: Default::default(),
        }
    }

    pub fn run_main(&mut self, instance: Arc<Instance>, native_method_handler: Box<&dyn NativeMethodHandler>) -> Result<(), RuntimeError> {
        let class = self.vm.get_class(&instance.class_name);
        let function = class.functions_pool.get(&Vm::calculate_hash(&String::from("_main"))).unwrap();
        let call_frame = CallFrame::new(function, 1, 0, self.debug_flag);
        self.run(call_frame, 0, class.as_ref(), &mut Some(instance), native_method_handler).map(|_| ())
    }

    pub fn run_function(&mut self, class: Arc<Class>, instance: &mut Option<Arc<Instance>>, function: &Function, native_method_handler: Box<&dyn NativeMethodHandler>, constructor_args: Vec<Value>) -> Result<(), RuntimeError> {
        let call_frame = CallFrame::new(function, 0, constructor_args.len(), self.debug_flag);
        for argument in constructor_args {
            match argument {
                Value::String(_) | Value::Number(_) => {
                    let reference = self.vm.add_in_constant_pool(argument);
                    self.stack.push(ConstantPoolReference(reference));
                }
                _ => panic!("Only String or Number argument are allowed in constructor arguments")
            }
        }
        self.run(call_frame, 0, class.as_ref(), instance, native_method_handler).map(|_| ())
    }

    pub fn run(&mut self, mut call_frame: CallFrame, depth: usize, class: &Class, instance: &mut Option<Arc<Instance>>, native_method_handler: Box<&dyn NativeMethodHandler>) -> Result<CallFrameBreak, RuntimeError> {
        let mut stdout = io::stdout();
        self.print_before_current_run(&call_frame, class, &mut stdout);
        let mut op_index = 0;
        loop {
            if self.aborted.load(Relaxed) {
                return Ok(CallFrameBreak::End);
            }
            if op_index >= call_frame.op_codes.len() {
                break;
            }
            class.sources.get(&Vm::calculate_hash(&call_frame.name)).as_ref().unwrap().get(op_index).map(|source| self.current_source_line = source.clone()); // TODO remove clone here
            let next_op_code = call_frame.op_codes.get(op_index).unwrap();
            self.print_dump_current_op_code(&call_frame, class, instance, &mut stdout, &mut op_index, next_op_code);
            match next_op_code {
                OpCode::LoadConstant(reference) => {
                    self.stack.push(StackEntry::ConstantPoolReference(*reference));
                }
                OpCode::AssignVariable(reference) => {
                    let value_from_stack = self.value_from_stack_entry(&StackEntry::ConstantPoolReference(*reference), &call_frame, class, instance)?;
                    let variable_name = value_from_stack.string_value().unwrap();
                    let variable = Variable::from_string(variable_name);
                    let owner_reference = Vm::calculate_hash(&variable);
                    self.variable_assign_reference(class, instance, &mut call_frame, &native_method_handler, variable, owner_reference)?;
                }
                OpCode::StoreGlobal => {
                    let variable_name_stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let value_from_stack = self.value_from_stack_entry(&variable_name_stack_entry, &call_frame, class, instance)?;
                    let variable_name = value_from_stack.string_value().unwrap();
                    let variable = Variable::from_string(variable_name);
                    let owner_reference = Vm::calculate_hash(&variable);
                    self.variable_assign_reference(class, instance, &mut call_frame, &native_method_handler, variable, owner_reference)?;
                }
                OpCode::LoadGlobal => {
                    let variable_name_stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let value_from_stack = self.value_from_stack_entry(&variable_name_stack_entry, &call_frame, class, instance)?;
                    let variable_name = value_from_stack.string_value().unwrap();
                    let variable = Variable::from_string(variable_name);
                    let owner_reference = Vm::calculate_hash(&variable);
                    self.load_global(class, instance, &call_frame, &native_method_handler, &variable, owner_reference)?;
                }
                OpCode::ArrayStore => {
                    let arr_index = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let arr_index = self.value_from_stack_entry(&arr_index, &call_frame, class, instance)?.number_value().map_err(|e| self.new_runtime_from_temporary(e, "Expected array index to be a number"))?;
                    let array_ref_stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    if let StackEntry::HeapReference((owner_reference, reference)) = array_ref_stack_entry {
                        let array = self.vm.array_from_heap_reference(owner_reference, reference)
                            .map_err(|err| self.new_runtime_from_temporary(err, "VM: ArrayStore expected to retrieve array from heap reference"))?;
                        if self.stack.len() > 0 {
                            let value_ref_stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                            let value_constant_ref = self.constant_ref_from_stack_entry(&value_ref_stack_entry, &call_frame, class, instance);
                            let value = value_constant_ref.map_or_else(|err| None, |ok| Some(ok));
                            array.assign(arr_index as usize, value, self.array_update_callback(&call_frame, &native_method_handler, array.clone(), &class));
                        }

                    } else {
                        return Err(self.new_runtime_error("OpCode::ArrayStore - Expected stack entry to be a heap reference.".to_string()));
                    }
                }
                OpCode::ArrayLoad => {
                    let arr_index = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let arr_index = self.value_from_stack_entry(&arr_index, &call_frame, class, instance)?.number_value().map_err(|e| self.new_runtime_from_temporary(e, "Expected array index to be a number"))?;
                    let array_ref_stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    if let StackEntry::HeapReference((owner_reference, reference)) = array_ref_stack_entry {
                        self.stack.push(StackEntry::ArrayHeapReference((owner_reference, reference, arr_index as usize)));
                    } else {
                        return Err(self.new_runtime_error("OpCode::ArrayLoad - Expected stack entry to be a heap reference.".to_string()));
                    }
                }
                OpCode::StoreLocal(reference) => {
                    let variable = call_frame.get_local(*reference).ok_or_else(|| self.new_runtime_error("Variable is not declared in local scope".to_string()))?;
                    let owner_reference = call_frame.hash_code();
                    let variable_cloned = variable.clone();
                    self.variable_assign_reference(class, instance, &mut call_frame, &native_method_handler, variable_cloned, owner_reference)?;
                }
                OpCode::LoadLocal(reference) => {
                    self.load_local_variable(&call_frame, reference)?;
                }
                OpCode::LoadValue => {
                    let stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let constant_reference = self.constant_ref_from_stack_entry(&stack_entry, &call_frame, class, instance)?;
                    self.stack.push(StackEntry::ConstantPoolReference(constant_reference));
                }
                OpCode::StoreInstance(reference) => {
                    if instance.is_none() {
                        return Err(self.new_runtime_error("Can't store instance variable in a static(non-instance) context".to_string()));
                    }
                    let instance_variables_guard = instance.as_ref().unwrap().variables.read().unwrap();
                    let variable = instance_variables_guard.get(reference)
                        .ok_or_else(|| self.new_runtime_error("Variable is not declared in instance scope".to_string()))?.clone();
                    let owner_reference = instance.as_ref().unwrap().hash_code();
                    drop(instance_variables_guard);
                    self.variable_assign_reference(class, instance, &mut call_frame, &native_method_handler, variable, owner_reference)?;
                }
                OpCode::LoadInstance(reference) => {
                    if instance.is_none() {
                        return Err(self.new_runtime_error("Can't load instance variable in a static(non-instance) context".to_string()));
                    }
                    self.load_instance_variable(instance, reference)?;
                }
                OpCode::StoreStatic(reference) => {
                    let variable = class.get_variable(*reference).ok_or_else(|| self.new_runtime_error("Variable is not declared in class scope".to_string()))?;
                    let owner_reference = class.hash_code();
                    self.variable_assign_reference(class, instance, &mut call_frame, &native_method_handler, variable.clone(), owner_reference)?;
                }
                OpCode::LoadStatic(reference) => {
                    self.load_static_variable(class, reference)?;
                }
                OpCode::DefineFunction(_) => {}
                OpCode::Equal => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v1.is_string() && v2.is_string() {
                        v1.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? == v2.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                    } else if v2.is_number() && v2.is_number() {
                        v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? == v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::SwitchCompare => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.peek(self.stack.len() - 1).map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v1.is_string() && v2.is_string() {
                        v1.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? == v2.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                    } else if v2.is_number() && v2.is_number() {
                        v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? == v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::NotEqual => {
                    // TODO refactor, same op code can be used with a variant
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v1.is_string() && v2.is_string() {
                        v1.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? != v2.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                    } else if v2.is_number() && v1.is_number() {
                        v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? != v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::BitAnd => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let result = v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                        & v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?;
                    let result_as_number = Value::Number(Some(result));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::BitOr => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let result = v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?
                        | v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?;
                    let result_as_number = Value::Number(Some(result));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::LogicalAnd => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v2.is_number() && v1.is_number() {
                        v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? > 0 && v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? > 0
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::LogicalOr => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v2.is_number() && v1.is_number() {
                        v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? == 1 || v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? == 1
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Relational(relational) => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v1.is_number() && v2.is_number() {
                        match relational {
                            Relational::GT => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? > v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            Relational::GTE => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? >= v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            Relational::LT => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? < v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            Relational::LTE => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? <= v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                        }
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Add => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        Value::String(Some(format!("{}{}",
                                                   if v1.is_string() { v1.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?.clone() } else { v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?.to_string() },
                                                   if v2.is_string() { v2.string_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?.clone() } else { v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?.to_string() })))
                    } else {
                        Value::Number(Some(v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? + v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::NumericOperation(operation) => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let stack_entry2 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        let operation_name = match operation {
                            NumericOperation::Subtract => "subtract",
                            NumericOperation::Multiply => "multiply",
                            NumericOperation::Divide => "divide",
                            NumericOperation::Modulo => "modulo",
                            NumericOperation::Add => "add",
                            NumericOperation::RightShift => "right shift",
                            NumericOperation::LeftShift => "left shift",
                        };
                        return Err(self.new_runtime_error(format!("Attempt to {} strings: {} - {}", operation_name, v1, v2)));
                    } else {
                        let res = match operation {
                            NumericOperation::Subtract => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? - v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            NumericOperation::Multiply => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? * v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            NumericOperation::Divide => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? / v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            NumericOperation::Modulo => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? % v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            NumericOperation::Add => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? + v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            NumericOperation::RightShift => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? >> v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                            NumericOperation::LeftShift => v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? << v2.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))?,
                        };
                        Value::Number(Some(res))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Jump(jump_to_index) => {
                    op_index = *jump_to_index - 1;
                }
                OpCode::Goto(index) => {
                    if depth > 0 {
                        return Ok(CallFrameBreak::Goto(*index));
                    } else {
                        op_index = if *index > 0 {
                            *index - 1
                        } else {
                            *index
                        }
                    }
                }
                OpCode::CallNative { argument_count, reference } => {
                    self.add_stack_trace(StackTrace::from_compilation_detail(&self.current_source_line, call_frame.name.clone(), class.name.clone()));
                    let mut arguments: Vec<Value> = vec![];
                    let mut arguments_ref: Vec<Option<u64>> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                        arguments.push(self.value_from_stack_entry(&stack_entry, &call_frame, class, instance)?);
                        arguments_ref.push(self.constant_ref_from_stack_entry(&stack_entry, &call_frame, class, instance).ok());
                    }
                    arguments.reverse();
                    arguments_ref.reverse();
                    let native_method = self.native_from_stack_entry(StackEntry::NativeReference(*reference))?;
                    if NATIVE_FUNCTIONS.iter().any(|native| native.name == native_method.name.as_str()) {
                        handle_native_method(self, native_method, class, instance, &mut call_frame, arguments, arguments_ref, &native_method_handler)?;
                    } else {
                        native_method_handler.handle(native_method, arguments, self, &call_frame, &self.current_source_line, class.name.clone());
                    }
                    self.stack_traces.pop();
                }
                OpCode::CallFunction { argument_count, reference } => {
                    self.add_stack_trace(StackTrace::from_compilation_detail(&self.current_source_line, call_frame.name.clone(), class.name.clone()));
                    let global_class = self.vm.get_global_class(reference);
                    let mut is_global_function = false;
                    let mut function = class.functions_pool.get(reference);
                    if function.is_none() {
                        function = global_class.functions_pool.get(reference);
                        is_global_function = function.is_some();
                    }
                    if function.is_none() {
                        return Err(self.new_runtime_error("Function is not defined in current script nor in global functions list".to_string()));
                    }
                    let function = function.unwrap();
                    let stack_pointer = self.stack.len() - argument_count;
                    // Convert function argument to constant ref. If we pass current frame's variable in function argument, we need to retrieve value in sub callframe.
                    // We can do that by putting retrieved constant reference and put them in the stack.
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                        arguments.push(self.value_from_stack_entry(&stack_entry, &call_frame, class, instance)?);
                    }
                    arguments.reverse();
                    for value in arguments {
                        match value {
                            Value::String(_) | Value::Number(_) => {
                                let reference = self.vm.add_in_constant_pool(value);
                                self.stack.push(ConstantPoolReference(reference));
                            }
                            Value::Reference(references) => {
                                if let Some((owner_reference, reference)) = references {
                                    self.stack.push(HeapReference((owner_reference, reference)));
                                } else {
                                    panic!("CallFunction Value::Reference")
                                }
                            }
                            Value::ArrayEntry(array_entry) => {
                                if let Some((owner_reference, reference, _, index)) = array_entry {
                                    self.stack.push(ArrayHeapReference((owner_reference, reference, index)));
                                } else {
                                    panic!("CallFunction Value::ArrayEntry")
                                }
                            }
                        }
                    }
                    let new_function_call_frame = CallFrame::new(function, stack_pointer, *argument_count, self.debug_flag);
                    let class = if is_global_function {
                        global_class.as_ref()
                    } else {
                        class
                    };
                    let break_type = self.run(new_function_call_frame, depth + 1, class, instance, native_method_handler.clone())?;
                    self.stack_traces.pop();
                    match break_type {
                        CallFrameBreak::Return(has_returned) => {
                            if !has_returned {
                                self.stack.truncate(stack_pointer);
                            }
                        }
                        CallFrameBreak::Goto(index) => {
                            if depth > 0 {
                                return Ok(CallFrameBreak::Goto(index));
                            }
                            op_index = index - 1
                        }
                        CallFrameBreak::End => {
                            return Ok(CallFrameBreak::End);
                        }
                    }
                }
                OpCode::Call => {}
                OpCode::Return(has_returned) => {
                    return if *has_returned {
                        let returned_stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                        self.stack.truncate(call_frame.stack_pointer);
                        let returned_value = self.value_from_stack_entry(&returned_stack_entry, &call_frame, class, instance)?;
                        let reference = self.vm.add_in_constant_pool(returned_value);
                        self.stack.push(StackEntry::ConstantPoolReference(reference));
                        Ok(CallFrameBreak::Return(true))
                    } else {
                        Ok(CallFrameBreak::Return(false))
                    };
                }
                OpCode::Command => {}
                OpCode::If(jump_to_index) => {
                    let stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let value = self.value_from_stack_entry(&stack_entry, &call_frame, class, instance)?;
                    if value.number_value().map_err(|err| self.new_runtime_from_temporary(err, "VM: If expected expression result to be a number"))? == 0 {
                        op_index = *jump_to_index - 1;
                    }
                }
                OpCode::Else => {
                    // does nothing
                }
                OpCode::End => {
                    return Ok(CallFrameBreak::End);
                }
                OpCode::Noop => {// Does nothing
                }
                OpCode::GlobalArrayStore => {}
                OpCode::GlobalArrayLoad => {}
                OpCode::BitNot => {
                    let stack_entry1 = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let number = v1.number_value().map_err(|err| self.new_runtime_from_temporary(err, ""))? as u32;
                    let result_as_number = Value::Number(Some(((!number) & (!(1 << 31))) as i32)); // set u32 first bit to 0 to be able to convert to i32
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
            }
            op_index += 1;
        }
        self.print_after_current_run(call_frame, class, &mut stdout);
        Ok(CallFrameBreak::Return(false))
    }

    pub(crate) fn load_global(&self, class: &Class, instance: &mut Option<Arc<Instance>>, call_frame: &CallFrame, native_method_handler: &&dyn NativeMethodHandler, variable: &Variable, owner_reference: u64) -> Result<(), RuntimeError> {
        let variable_ref = Vm::calculate_hash(&variable);
        if variable.value_ref.is_array() {
            // When we load a global array we allocated it on the heap
            self.vm.allocate_array_if_needed(owner_reference, variable_ref, variable.value_ref.value_type.clone(), variable);
            let array = self.vm.array_from_heap_reference(owner_reference, variable_ref).unwrap();
            let arguments = vec![Value::String(Some(array.name.clone())), Value::String(Some(array.scope.as_string()))];
            // Calling native getglobalarray which put on stack array: number of stack entreies + (index/value reference)*
            // TODO: doc
            native_method_handler.handle(&Native { name: "getglobalarray".to_string() }, arguments, self, call_frame, &self.current_source_line, class.name.clone());
            let number_of_stack_entries = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
            let number_of_stack_entries = self.value_from_stack_entry(&number_of_stack_entries, call_frame, class, instance)?.number_value().unwrap() as usize;
            let mut i = 0;
            loop {
                if i >= number_of_stack_entries {
                    break;
                }
                let array_index = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                let array_index = self.value_from_stack_entry(&array_index, call_frame, class, instance)?.number_value().unwrap() as usize;
                let array_value = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
                let array_value = self.constant_ref_from_stack_entry(&array_value, call_frame, class, instance)?;
                array.assign::<Box<dyn Fn(Array)>>(array_index, Some(array_value), None);
                i += 2;
            }
            //
            self.stack.push(StackEntry::HeapReference((owner_reference, variable_ref)));
        } else {
            let arguments: Vec<Value> = vec![Value::String(Some(variable.to_script_identifier())), Value::String(Some(variable.scope.as_string()))];
            // TODO: doc
            native_method_handler.handle(&Native { name: "getglobalvariable".to_string() }, arguments, self, call_frame, &self.current_source_line, class.name.clone());
        }
        Ok(())
    }

    pub(crate) fn array_update_callback<'thread, 'program: 'thread>(&'thread self, call_frame: &'thread CallFrame, native_method_handler: &'thread &'program dyn NativeMethodHandler, array: Arc<Array>, class: &'thread Class) -> Option<Box<dyn Fn(Array) + 'thread>> {
        if array.scope.is_global() {
            let closure = move |_updated_array: Array| {
                let mut arguments: Vec<Value> = vec![Value::String(Some(array.name.clone())), Value::String(Some(array.scope.as_string()))];
                // call setglobalarray(array_name, scope, values...)
                for (index, value_reference) in array.values.read().unwrap().iter().enumerate() {
                    if let Some(reference) = value_reference {
                        arguments.push(Value::Number(Some(index as i32)));
                        arguments.push(self.vm.get_from_constant_pool(*reference).unwrap().value());
                    }
                }
                // TODO: doc
                native_method_handler.handle(&Native { name: "setglobalarray".to_string() }, arguments, self, call_frame, &self.current_source_line, class.name.clone());
            };
            Some(Box::new(closure))
        } else {
            None
        }
    }

    pub(crate) fn array_remove_items_callback<'thread, 'program: 'thread>(&'thread self, call_frame: &'thread CallFrame, native_method_handler: &'thread &'program dyn NativeMethodHandler, array: Arc<Array>, class: &'thread Class) -> Option<Box<dyn Fn(Array, usize, usize) + 'thread>> {
        if array.scope.is_global() {
            let closure = move |_updated_array: Array, start_index: usize, end_index: usize| {
                let arguments: Vec<Value> = vec![Value::String(Some(array.name.clone())), Value::String(Some(array.scope.as_string())), Value::new_number(start_index as i32), Value::new_number(end_index as i32)];
                // TODO: doc
                native_method_handler.handle(&Native { name: "removeitemsglobalarray".to_string() }, arguments, self, call_frame, &self.current_source_line, class.name.clone());
            };
            Some(Box::new(closure))
        } else {
            None
        }
    }

    pub fn push_constant_on_stack(&self, value: Value) {
        let reference = self.vm.add_in_constant_pool(value);
        self.stack.push(StackEntry::ConstantPoolReference(reference));
    }

    pub fn new_runtime_error(&self, message: String) -> RuntimeError {
        RuntimeError::new_string(self.current_source_line.clone(), self.stack_traces.clone(), message)
    }

    pub fn new_runtime_from_temporary(&self, err: TemporaryRuntimeError, message: &str) -> RuntimeError {
        RuntimeError::from_temporary_and_message(self.current_source_line.clone(), self.stack_traces.clone(), err, message)
    }

    pub fn load_local_variable(&self, call_frame: &CallFrame, reference: &u64) -> Result<(), RuntimeError> {
        let variable = self.get_local_variable(call_frame, reference)?;
        let owner_reference = call_frame.hash_code();
        self.load_variable(&variable, owner_reference, || StackEntry::LocalVariableReference(*reference));
        Ok(())
    }

    pub fn get_local_variable(&self, call_frame: &CallFrame, reference: &u64) -> Result<Variable, RuntimeError> {
        let variable = call_frame.get_local(*reference).ok_or_else(|| self.new_runtime_error("Variable is not declared in local scope".to_string()))?;
        Ok(variable.clone())
    }

    pub fn load_static_variable(&self, class: &Class, reference: &u64) -> Result<(), RuntimeError> {
        let variable = self.get_static_variable(class, reference)?;
        let owner_reference = class.hash_code();
        self.load_variable(&variable, owner_reference, || StackEntry::StaticVariableReference(*reference));
        Ok(())
    }

    pub fn get_static_variable(&self, class: &Class, reference: &u64) -> Result<Variable, RuntimeError> {
        let variable = class.get_variable(*reference).ok_or_else(|| self.new_runtime_error("Variable is not declared in NPC scope".to_string()))?;
        Ok(variable)
    }

    pub fn load_instance_variable(&self, instance: &mut Option<Arc<Instance>>, reference: &u64) -> Result<(), RuntimeError> {
        let variable = self.get_instance_variable(instance, reference)?;
        let instance = instance.as_ref().unwrap();
        let owner_reference = instance.hash_code();
        self.load_variable(&variable, owner_reference, || StackEntry::InstanceVariableReference(*reference));
        Ok(())
    }

    pub fn get_instance_variable(&self, instance: &Option<Arc<Instance>>, reference: &u64) -> Result<Variable, RuntimeError> {
        let instance = instance.as_ref().unwrap();
        let instance_variables_guard = instance.variables.read().unwrap();
        let variable = instance_variables_guard.get(reference).ok_or_else(|| self.new_runtime_error("Variable is not declared in instance scope".to_string()))?;
        Ok(variable.clone())
    }

    fn load_variable<F>(&self, variable: &Variable, owner_reference: u64, apply_when_not_array: F)
        where F: FnOnce() -> StackEntry {
        if variable.value_ref.is_array() {
            let array_ref = Vm::calculate_hash(variable);
            self.stack.push(StackEntry::HeapReference((owner_reference, array_ref)));
        } else {
            self.stack.push(apply_when_not_array());
        }
    }

    pub(crate) fn variable_assign_reference(&self, class: &Class, instance: &mut Option<Arc<Instance>>, call_frame: &mut CallFrame, native_method_handler: &&dyn NativeMethodHandler, variable: Variable, owner_reference: u64) -> Result<(), RuntimeError> {
        let variable_ref = Vm::calculate_hash(&variable);
        let reference = if variable.value_ref.is_array() {
            self.vm.allocate_array_if_needed(owner_reference, variable_ref, variable.value_ref.value_type.clone(), &variable);
            self.stack.push(StackEntry::HeapReference((owner_reference, variable_ref)));
            variable_ref
        } else {
            let stack_entry = self.stack.pop().map_err(|err| self.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
            let value = self.value_from_stack_entry(&stack_entry, call_frame, class, instance)?;
            if !variable.value_ref.value_type.match_value(&value) {
                return Err(RuntimeError::new_string(self.current_source_line.clone(), self.stack_traces.clone(),
                                                    format!("tried to assign a {} to a variable declared as {}",
                                                            value.display_type(), variable.value_ref.value_type.display_type())));
            }
            self.constant_ref_from_stack_entry(&stack_entry, call_frame, class, instance)?
        };
        variable.set_value_ref(reference);
        if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Instance) {
            if instance.is_none() {
                return Err(self.new_runtime_error("Can't store instance variable in a static(non-instance) context".to_string()));
            }
            let instance = instance.as_ref().unwrap();
            instance.insert_variable(variable_ref, variable);
        } else if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Npc) {
            class.insert_variable(variable_ref, variable);
        } else if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Local) {
            call_frame.locals.insert(variable_ref, variable);
        } else {
            let mut arguments: Vec<Value> = vec![];
            // call setglobalvariable(variable_name, scope, <expression result to assign to variable>)
            arguments.push(Value::String(Some(variable.to_script_identifier())));
            arguments.push(Value::String(Some(variable.scope.as_string())));
            if !variable.value_ref.is_array() { // In case of array, reference is the reference of the array, not of the expression result constant. it will come in ArrayStore instruction
                arguments.push(self.vm.get_from_constant_pool(reference).unwrap().value());
                // TODO: doc
                native_method_handler.handle(&Native { name: "setglobalvariable".to_string() }, arguments, self, call_frame, &self.current_source_line, class.name.clone());
            }
        };
        Ok(())
    }

    fn value_from_stack_entry(&self, stack_entry: &StackEntry, call_frame: &CallFrame, class: &Class, instance: &Option<Arc<Instance>>) -> Result<Value, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                let constant = self.vm.get_from_constant_pool(*reference).ok_or_else(|| self.new_runtime_error(format!("Can't find constant in VM constant pool for given reference ({})", reference)))?;
                Ok(constant.value())
            }
            StackEntry::LocalVariableReference(reference) => {
                let variable = call_frame.get_local(*reference).ok_or_else(|| self.new_runtime_error(format!("Can't find local variable in CAllFRAME local variable pool for given reference ({})", reference)))?;
                Ok(self.value_from_value_ref(&variable.value_ref)?)
            }
            StackEntry::NativeReference(reference) => {
                let native = self.vm.get_from_native_pool(*reference).ok_or_else(|| self.new_runtime_error(format!("Can't find native in VM native pool for given reference ({})", reference)))?;
                Ok(Value::String(Some(native.name.clone())))
            }
            StackEntry::FunctionReference(reference) => {
                let function = class.functions_pool.get(reference).ok_or_else(|| self.new_runtime_error(format!("Can't find function in CLASS function pool for given reference ({})", reference)))?;
                Ok(Value::String(Some(function.name.clone())))
            }
            StackEntry::InstanceVariableReference(reference) => {
                if instance.is_none() {
                    return Err(self.new_runtime_error("Can't find instance variable in a static (non-instance) context.".to_string()));
                }
                let instance = instance.as_ref().unwrap();
                let instance_variables_guard = instance.variables.read().unwrap();
                let variable = instance_variables_guard.get(reference).ok_or_else(|| self.new_runtime_error(format!("Can't find instance variable in INSTANCE variable pool for given reference ({})", reference)))?;
                let value = self.value_from_value_ref(&variable.value_ref)?;
                Ok(value)
            }
            StackEntry::StaticVariableReference(reference) => {
                let variable = class.get_variable(*reference).ok_or_else(|| self.new_runtime_error(format!("Can't find instance variable in CLASS static variable pool for given reference ({})", reference)))?;

                Ok(self.value_from_value_ref(&variable.value_ref)?)
            }
            StackEntry::HeapReference((owner_reference, reference)) => {
                Ok(Value::Reference(Some((*owner_reference, *reference))))
            }
            StackEntry::ArrayHeapReference((owner_reference, reference, index)) => {
                if let Ok(array) = self.vm.array_from_heap_reference(*owner_reference, *reference) {
                    let array_value_ref = array.get(*index);
                    let constant = if array_value_ref.is_err() {
                        // TODO warn error?
                        None
                    } else { array_value_ref.ok().unwrap().map(|array_value_ref| self.vm.get_from_constant_pool(array_value_ref).unwrap()) };
                    Ok(Value::ArrayEntry(Some((*owner_reference, *reference, constant, *index))))
                } else {
                    Err(self.new_runtime_error("value_from_stack_entry ArrayHeapReference - Expected heap entry to contain array".to_string()))
                }
            }
            StackEntry::VariableReference((scope, _owner_reference, reference)) => {
                let variable = self.get_variable_from_scope_and_reference(call_frame, class, instance, scope, *reference)?;
                let value = self.value_from_value_ref(&variable.value_ref)?;
                Ok(value)
            }
        }
    }

    fn get_variable_from_scope_and_reference(&self, call_frame: &CallFrame, class: &Class, instance: &Option<Arc<Instance>>, scope: &Scope, reference: u64) -> Result<Variable, RuntimeError> {
        let variable = if mem::discriminant(scope) == mem::discriminant(&Scope::Instance) {
            if instance.is_none() {
                return Err(RuntimeError::new(self.current_source_line.clone(), self.stack_traces.clone(),
                                             "Can't find instance variable in a static (non-instance) context."));
            }
            self.get_instance_variable(instance, &reference)?
        } else if mem::discriminant(scope) == mem::discriminant(&Scope::Npc) {
            self.get_static_variable(class, &reference)?
        } else if mem::discriminant(scope) == mem::discriminant(&Scope::Local) {
            self.get_local_variable(call_frame, &reference)?
        } else {
            panic!("Get variable value from reference - Not supported yet, only static, instance and local variable scope are supported");
        };
        Ok(variable)
    }

    fn value_from_value_ref(&self, value_ref: &ValueRef) -> Result<Value, RuntimeError> {
        match value_ref.value_type {
            ValueType::String => {
                let option = self.vm.get_from_constant_pool(value_ref.reference().ok_or_else(|| self.new_runtime_error("String ValueRef does not contains reference. Variable has been not initialized.".to_string()))?);
                let constant = option.ok_or_else(|| self.new_runtime_error(format!("Can't find constant in VM constant pool for given reference ({})", value_ref.reference().unwrap())))?;
                Ok(constant.value())
            }
            ValueType::Number => {
                let option = self.vm.get_from_constant_pool(value_ref.reference().ok_or_else(|| self.new_runtime_error("Number ValueRef does not contains reference. Variable has been not initialized.".to_string()))?);
                let constant = option.ok_or_else(|| self.new_runtime_error(format!("Can't find constant in VM constant pool for given reference ({})", value_ref.reference().unwrap())))?;
                Ok(constant.value())
            }
            _ => Err(self.new_runtime_error("value_from_value_ref - Array not implemented yet".to_string()))
        }
    }

    fn native_from_stack_entry(&self, stack_entry: StackEntry) -> Result<&Native, RuntimeError> {
        match stack_entry {
            StackEntry::NativeReference(reference) => {
                let native = self.vm.get_from_native_pool(reference).ok_or_else(|| self.new_runtime_error(format!("Can't find native in VM native pool for given reference ({})", reference)))?;
                Ok(native)
            }
            x => Err(RuntimeError::new_string(self.current_source_line.clone(), self.stack_traces.clone(), format!("Expected stack entry to be a reference to Native method but was {:?}", x)))
        }
    }

    fn constant_ref_from_stack_entry(&self, stack_entry: &StackEntry, call_frame: &CallFrame, class: &Class, instance: &Option<Arc<Instance>>) -> Result<u64, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                self.vm.get_from_constant_pool(*reference).ok_or_else(|| self.new_runtime_error(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference)))?;
                Ok(*reference)
            }
            StackEntry::LocalVariableReference(reference) => {
                let variable = call_frame.get_local(*reference).ok_or_else(|| self.new_runtime_error(format!("Can't find local variable in CAllFRAME local variable pool for given reference ({})", reference)))?;
                let constant_ref = variable.value_ref.get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| self.new_runtime_error(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference)))?;
                Ok(constant_ref)
            }
            StackEntry::InstanceVariableReference(reference) => {
                if instance.is_none() {
                    return Err(self.new_runtime_error("constant_ref_from_stack_entry - Can't retrieve constant reference for instance variable in a static(non-instance) context".to_string()));
                }
                let instance = instance.as_ref().unwrap();
                let instance_variables_guard = instance.variables.read().unwrap();
                let variable = instance_variables_guard.get(reference).ok_or_else(|| self.new_runtime_error(format!("Can't instance variable in INSTANCE variable pool for given reference ({})", reference)))?;
                let constant_ref = variable.value_ref.get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| self.new_runtime_error(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference)))?;
                Ok(constant_ref)
            }
            StackEntry::StaticVariableReference(reference) => {
                let variable = class.get_variable(*reference).ok_or_else(|| self.new_runtime_error(format!("Can't find static variable in CLASS static variable pool for given reference ({})", reference)))?;
                let constant_ref = variable.value_ref.get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| self.new_runtime_error(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference)))?;
                Ok(constant_ref)
            }
            StackEntry::VariableReference((scope, _owner_reference, reference)) => {
                let variable = self.get_variable_from_scope_and_reference(call_frame, class, instance, scope, *reference)?;
                let constant_ref = variable.value_ref.get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| self.new_runtime_error(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference)))?;
                Ok(constant_ref)
            }
            StackEntry::ArrayHeapReference((owner_reference, reference, index)) => {
                if let Ok(array) = self.vm.array_from_heap_reference(*owner_reference, *reference) {
                    let array_value_ref = array.get(*index);
                    let value = array_value_ref.map_err(|e| self.new_runtime_from_temporary(e.clone(), e.message.as_str()))?;
                    if let Some(value) = value {
                        Ok(value)
                    } else {
                        Err(self.new_runtime_error(format!("Value at index {} is not initialized", index)))
                    }
                } else {
                    Err(self.new_runtime_error("value_from_stack_entry ArrayHeapReference - Expected heap entry to contain array".to_string()))
                }
            }
            x => Err(RuntimeError::new_string(self.current_source_line.clone(), self.stack_traces.clone(), format!("Expected stack entry to be a reference to Constant but was {:?}", x)))
        }
    }

    pub fn array_constants(&self, array: Arc<Array>) -> Vec<Constant> {
        let mut i = 0;
        let mut values = vec![];
        while i < array.len() {
            if let Ok(maybe_reference) = array.get(i) {
                if let Some(reference) = maybe_reference {
                    if let Some(constant) = self.vm.get_from_constant_pool(reference) {
                        values.push(constant)
                    }
                }
            }
            i += 1;
        }
        values
    }

    fn add_stack_trace(&mut self, stack_trace: StackTrace) {
        if self.stack_traces.len() > 10 {
            self.stack_traces.remove(0);
        }
        self.stack_traces.push(stack_trace);
    }

    pub fn abort(&self) {
        self.aborted.store(true, SeqCst);
    }


    fn dump(&self, class: &Class, out: &mut Stdout) {
        if self.debug_flag & DebugFlag::Function.value() == DebugFlag::Function.value() {
            writeln!(out, "========= Functions =========").unwrap();
            for (reference, func) in class.functions_pool.iter() {
                writeln!(out, "({}) {}", reference, func).unwrap();
            }
        }
    }

    fn dump_stack(&self, out: &mut Stdout, call_frame: &CallFrame, class: &Class, instance: &mut Option<Arc<Instance>>) {
        if self.stack.contents().is_empty() {
            writeln!(out, "         <empty stack>").unwrap();
        } else {
            for (i, val) in self.stack.contents().iter().enumerate() {
                write!(out, "    [{}]  {:?}", i, val).unwrap();
                let maybe_value = self.value_from_stack_entry(val, call_frame, class, instance);
                if maybe_value.is_ok() {
                    write!(out, " - {}", maybe_value.unwrap()).unwrap();
                }
                writeln!(out).unwrap();
            }
        }
    }


    fn print_dump_current_op_code(&mut self, call_frame: &CallFrame, class: &Class, instance: &mut Option<Arc<Instance>>, stdout: &mut Stdout, op_index: &mut usize, next_op_code: &OpCode) {
        if self.debug_flag & DebugFlag::Execution.value() == DebugFlag::Execution.value() {
            writeln!(stdout, "=========  Thread Executing    ========").unwrap();
            writeln!(stdout, "[{}] {:?} - {}", op_index, next_op_code, self.current_source_line.single_line()).unwrap();
        }
        if self.debug_flag & DebugFlag::Stack.value() == DebugFlag::Stack.value() {
            writeln!(stdout, "=========  Thread Stack    ========").unwrap();
            self.dump_stack(stdout, call_frame, class, instance);
        }
        if self.debug_flag & DebugFlag::LocalsVariable.value() == DebugFlag::LocalsVariable.value() {
            call_frame.dump_locals(stdout, self.vm.clone());
        }
        stdout.flush().unwrap();
    }

    fn print_before_current_run(&mut self, call_frame: &CallFrame, class: &Class, stdout: &mut Stdout) {
        self.vm.dump(stdout);
        call_frame.dump(stdout, self.vm.clone());
        self.dump(class, stdout);
        stdout.flush().unwrap();
    }

    fn print_after_current_run(&mut self, _call_frame: CallFrame, class: &Class, stdout: &mut Stdout) {
        self.dump(class, stdout);
        stdout.flush().unwrap();
    }
}