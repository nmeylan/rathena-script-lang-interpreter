use std::{io};
use std::sync::{Arc};
use std::io::{Stdout, Write};
use std::rc::Rc;

use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Native, ValueRef, ValueType, Variable};
use crate::lang::vm::{Hashcode, HeapEntry, NATIVE_FUNCTIONS, Vm};
use crate::lang::value::Value;
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{*};
use crate::lang::class::{Class, Function, Instance};
use crate::lang::stack::StackEntry::ConstantPoolReference;
use crate::lang::vm::RuntimeError;

pub enum CallFrameBreak {
    Return(bool),
    Goto(usize),
    End,
}

pub struct Thread {
    pub vm: Arc<Vm>,
    stack: Stack,
}

impl Thread {
    pub fn new(vm: Arc<Vm>) -> Self {
        let stack = Stack::new();
        Self {
            vm,
            stack,
        }
    }

    pub fn run_main(&mut self, instance: Instance) -> Result<(), RuntimeError> {
        let class = self.vm.get_class(&instance.class_name);
        let function = class.functions_pool.get(&Vm::calculate_hash(&String::from("_main"))).unwrap();
        let call_frame = CallFrame::new(function, 1, 0);
        self.run(call_frame, 0, class.as_ref(), Some(&instance)).map(|_| ())
    }

    pub fn run_function(&mut self, class: Rc<Class>, instance: Option<&Instance>, function: &Function) -> Result<(), RuntimeError> {
        let call_frame = CallFrame::new(function, 1, 0);
        self.run(call_frame, 0, class.as_ref(), instance).map(|_| ())
    }

    pub fn run(&mut self, call_frame: CallFrame, depth: usize, class: &Class, instance: Option<&Instance>) -> Result<CallFrameBreak, RuntimeError> {
        let mut stdout = io::stdout();
        writeln!(stdout, "=========   VM    ========").unwrap();
        self.vm.dump(&mut stdout);
        writeln!(stdout, "=========   Thread    ========").unwrap();
        self.dump(class, &mut stdout);
        writeln!(stdout).unwrap();
        writeln!(stdout, "========= Call frame {} ========", call_frame.name).unwrap();
        call_frame.dump(&mut stdout);
        stdout.flush().unwrap();
        let mut op_index = 0;
        loop {
            if op_index >= call_frame.code.len() {
                break;
            }
            let next_op_code = call_frame.code.get(op_index).unwrap();
            writeln!(stdout, "=========   Executing    ========").unwrap();
            writeln!(stdout, "[{}] {:?}", op_index, next_op_code).unwrap();
            writeln!(stdout, "=========   Stack    ========").unwrap();
            self.dump_stack(&mut stdout, &call_frame, class, instance);
            stdout.flush().unwrap();
            match next_op_code {
                OpCode::LoadConstant(reference) => {
                    self.stack.push(StackEntry::ConstantPoolReference(*reference));
                }
                OpCode::StoreGlobal(_) => {}
                OpCode::LoadGlobal(_) => {}
                OpCode::ArrayStore(arr_index) => {
                    let array_ref_stack_entry = self.stack.pop()?;
                    let value_ref_stack_entry = self.stack.pop()?;
                    if let StackEntry::HeadReference((owner_reference, reference)) = array_ref_stack_entry {
                        let array = self.vm.array_from_heap_reference(owner_reference, reference)?;
                        array.assign(*arr_index, self.constant_ref_from_stack_entry(value_ref_stack_entry, &call_frame, class, instance)?)
                    } else {
                        return Err(RuntimeError::new("OpCode::ArrayStore - Expected stack entry to be a heap reference."));
                    }
                }
                OpCode::ArrayLoad(arr_index) => {
                    let array_ref_stack_entry = self.stack.pop()?;
                    if let StackEntry::HeadReference((owner_reference, reference)) = array_ref_stack_entry {
                        let array = self.vm.array_from_heap_reference(owner_reference, reference)?;
                        let result = array.get(*arr_index)?;
                        if result.is_none() {
                            return Err(RuntimeError::new_string(format!("OpCode::ArrayLoad - Array value as not been initialized at index {}.", arr_index)));
                        }
                        self.stack.push(StackEntry::ConstantPoolReference(result.unwrap()));
                    } else {
                        return Err(RuntimeError::new("OpCode::ArrayLoad - Expected stack entry to be a heap reference."));
                    }
                }
                OpCode::StoreLocal(reference) => {
                    let variable = call_frame.get_local(*reference).ok_or_else(|| RuntimeError::new(format!("Variable with reference {} is not declared in local scope", reference).as_str()))?;
                    let owner_reference = call_frame.hash_code();
                    self.set_variable(&call_frame, class, instance, variable, owner_reference);
                }
                OpCode::LoadLocal(reference) => {
                    let variable = call_frame.get_local(*reference).ok_or_else(|| RuntimeError::new(format!("Variable with reference {} is not declared in local scope", reference).as_str()))?;
                    let owner_reference = call_frame.hash_code();
                    self.load_variable(variable, owner_reference, || StackEntry::LocalVariableReference(*reference))
                }
                OpCode::StoreInstance(reference) => {
                    if instance.is_none() {
                        return Err(RuntimeError::new("Can't store instance variable in a static(non-instance) context"));
                    }
                    let variable = instance.unwrap().get_variable(*reference).ok_or_else(|| RuntimeError::new(format!("Variable with reference {} is not declared in instance scope", reference).as_str()))?;
                    let owner_reference = instance.unwrap().hash_code();
                    self.set_variable(&call_frame, class, instance, variable, owner_reference);
                }
                OpCode::LoadInstance(reference) => {
                    if instance.is_none() {
                        return Err(RuntimeError::new("Can't load instance variable in a static(non-instance) context"));
                    }
                    let variable = instance.unwrap().get_variable(*reference).ok_or_else(|| RuntimeError::new(format!("Variable with reference {} is not declared in local scope", reference).as_str()))?;
                    let owner_reference = instance.unwrap().hash_code();
                    self.load_variable(variable, owner_reference, || StackEntry::InstanceVariableReference(*reference));
                }
                OpCode::StoreStatic(reference) => {
                    let variable = class.get_variable(*reference).ok_or_else(|| RuntimeError::new(format!("Variable with reference {} is not declared in class scope", reference).as_str()))?;
                    let owner_reference = class.hash_code();
                    self.set_variable(&call_frame, class, instance, variable, owner_reference);
                }
                OpCode::LoadStatic(reference) => {
                    let variable = class.get_variable(*reference).ok_or_else(|| RuntimeError::new(format!("Variable with reference {} is not declared in local scope", reference).as_str()))?;
                    let owner_reference = class.hash_code();
                    self.load_variable(variable, owner_reference, || StackEntry::StaticVariableReference(*reference));
                }
                OpCode::DefineFunction(_) => {}
                OpCode::Equal => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v1.is_string() && v2.is_string() {
                        v1.string_value() == v2.string_value()
                    } else if v2.is_number() && v2.is_number() {
                        v1.number_value() == v2.number_value()
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::NotEqual => {
                    // TODO refactor, same op code can be used with a variant
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v1.is_string() && v2.is_string() {
                        v1.string_value() != v2.string_value()
                    } else if v2.is_number() && v1.is_number() {
                        v1.number_value() != v2.number_value()
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::LogicalAnd => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v2.is_number() && v1.is_number() {
                        v1.number_value() == 1 && v2.number_value() == 1
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::LogicalOr => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v2.is_number() && v1.is_number() {
                        v1.number_value() == 1 || v2.number_value() == 1
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Relational(relational) => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let comparison_result = if v2.is_number() && v2.is_number() {
                        match relational {
                            Relational::GT => v1.number_value() > v2.number_value(),
                            Relational::GTE => v1.number_value() >= v2.number_value(),
                            Relational::LT => v1.number_value() < v2.number_value(),
                            Relational::LTE => v1.number_value() <= v2.number_value(),
                        }
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Add => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        Value::String(Some(format!("{}{}",
                                                   if v1.is_string() { v1.string_value().clone() } else { v1.number_value().to_string() },
                                                   if v2.is_string() { v2.string_value().clone() } else { v2.number_value().to_string() })))
                    } else {
                        Value::Number(Some(v1.number_value() + v2.number_value()))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::NumericOperation(operation) => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame, class, instance)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame, class, instance)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        let operation_name = match operation {
                            NumericOperation::Subtract => "subtract",
                            NumericOperation::Multiply => "multiply",
                            NumericOperation::Divide => "divide",
                            NumericOperation::Modulo => "modulo"
                        };
                        return Err(RuntimeError::new(format!("Attempt to {} strings: {} - {}", operation_name, v1, v2).as_str()));
                    } else {
                        let res = match operation {
                            NumericOperation::Subtract => v1.number_value() - v2.number_value(),
                            NumericOperation::Multiply => v1.number_value() * v2.number_value(),
                            NumericOperation::Divide => v1.number_value() / v2.number_value(),
                            NumericOperation::Modulo => v1.number_value() % v2.number_value(),
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
                        op_index = *index - 1;
                    }
                }
                OpCode::CallNative { argument_count, reference } => {
                    let mut arguments: Vec<Value> = vec![];
                    //TODO allow heap reference access
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop()?;
                        arguments.push(self.value_from_stack_entry(&stack_entry, &call_frame, class, instance)?);
                    }
                    arguments.reverse();
                    let native_method = self.native_from_stack_entry(StackEntry::NativeReference(*reference))?;
                    if NATIVE_FUNCTIONS.iter().find(|(native, _)| native == &native_method.name.as_str()).is_some() {
                        self.handle_native_method(native_method, &call_frame, arguments)?;
                    } else {
                        self.vm.native_method_handler().handle(native_method, arguments, self, &call_frame);
                    }
                }
                OpCode::CallFunction { argument_count, reference } => {
                    let function = class.functions_pool.get(reference).unwrap();
                    let stack_pointer = self.stack.len() - argument_count;
                    // Convert function argument to heap ref. If we use current frame variable as argument, we need to retrieve value in sub callframe.
                    // We can do that by putting retrieving constant reference and put them in the stack.
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop()?;
                        arguments.push(self.value_from_stack_entry(&stack_entry, &call_frame, class, instance)?);
                    }
                    for value in arguments {
                        let reference = self.vm.add_in_constant_pool(value);
                        self.stack.push(ConstantPoolReference(reference));
                    }
                    let new_function_call_frame = CallFrame::new(function, stack_pointer, *argument_count);
                    let break_type = self.run(new_function_call_frame, depth + 1, class, instance)?;
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
                OpCode::Return(not_empty_return) => {
                    return if *not_empty_return {
                        let returned_stack_entry = self.stack.pop()?;
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
                    let stack_entry = self.stack.pop()?;
                    let value = self.value_from_stack_entry(&stack_entry, &call_frame, class, instance)?;
                    if value.number_value() != 1 {
                        op_index = *jump_to_index - 1;
                    }
                }
                OpCode::Else => {
                    // does nothing
                }
                OpCode::SkipOp => {
                    op_index += 1;
                }
                OpCode::End => {
                    return Ok(CallFrameBreak::End);
                }
            }
            op_index += 1;
        }
        writeln!(stdout, "*********   Final status ********").unwrap();
        writeln!(stdout, "=========   Thread   ========").unwrap();
        self.dump(class, &mut stdout);
        writeln!(stdout, "========= Call frame ========").unwrap();
        call_frame.dump(&mut stdout);
        stdout.flush().unwrap();
        Ok(CallFrameBreak::Return(false))
    }

    fn load_variable<F>(&mut self, variable: &Variable, owner_reference: u64, apply_when_not_array: F)
        where F: FnOnce() -> StackEntry {
        if variable.value_ref.borrow().is_array() {
            let array_ref = Vm::calculate_hash(variable);
            self.stack.push(StackEntry::HeadReference((owner_reference, array_ref)));
        } else {
            self.stack.push(apply_when_not_array());
        }
    }

    fn set_variable(&mut self, call_frame: &CallFrame, class: &Class, instance: Option<&Instance>, variable: &Variable, owner_reference: u64) -> Result<(), RuntimeError> {
        let reference = if variable.value_ref.borrow().is_array() {
            let array_ref = Vm::calculate_hash(variable);
            self.vm.allocate_array_if_needed(owner_reference, array_ref, variable.value_ref.borrow().value_type.clone());
            self.stack.push(StackEntry::HeadReference((owner_reference, array_ref)));
            array_ref
        } else {
            let stack_entry = self.stack.pop()?;
            self.constant_ref_from_stack_entry(stack_entry, &call_frame, class, instance)?
        };
        Ok(variable.set_value_ref(reference))
    }

    fn dump(&self, class: &Class, out: &mut Stdout) {
        writeln!(out, "========= Functions =========").unwrap();
        for (reference, func) in class.functions_pool.iter() {
            writeln!(out, "({}) {}", reference, func).unwrap();
        }
        writeln!(out, "========= VM =========").unwrap();
        self.vm.dump(out);
    }

    fn dump_stack(&self, out: &mut Stdout, call_frame: &CallFrame, class: &Class, instance: Option<&Instance>) {
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

    fn value_from_stack_entry(&self, stack_entry: &StackEntry, call_frame: &CallFrame, class: &Class, instance: Option<&Instance>) -> Result<Value, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                let constant = self.vm.get_from_constant_pool(*reference).ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant.value())
            }
            StackEntry::LocalVariableReference(reference) => {
                let variable = call_frame.get_local(*reference).ok_or_else(|| RuntimeError::new(format!("Can't find local variable in CAllFRAME local variable pool for given reference ({})", reference).as_str()))?;
                Ok(self.value_from_value_ref(&variable.value_ref.borrow())?)
            }
            StackEntry::NativeReference(reference) => {
                let native = self.vm.get_from_native_pool(*reference).ok_or_else(|| RuntimeError::new(format!("Can't find native in VM native pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(native.name.clone())))
            }
            StackEntry::FunctionReference(reference) => {
                let function = class.functions_pool.get(reference).ok_or_else(|| RuntimeError::new(format!("Can't find function in CLASS function pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(function.name.clone())))
            }
            StackEntry::InstanceVariableReference(reference) => {
                if instance.is_none() {
                    return Err(RuntimeError::new("Can't find instance variable in a static (non-instance) context."));
                }
                let variable = instance.unwrap().variables.get(reference).ok_or_else(|| RuntimeError::new(format!("Can't find instance variable in INSTANCE variable pool for given reference ({})", reference).as_str()))?;
                Ok(self.value_from_value_ref(&variable.value_ref.borrow())?)
            }
            StackEntry::StaticVariableReference(reference) => {
                let variable = class.static_variables.get(reference).ok_or_else(|| RuntimeError::new(format!("Can't find instance variable in CLASS static variable pool for given reference ({})", reference).as_str()))?;
                Ok(self.value_from_value_ref(&variable.value_ref.borrow())?)
            }
            StackEntry::HeadReference((owner_reference, reference)) => {
                Ok(self.value_from_value_ref(&self.vm.get_value_ref_from_heap_entry(*owner_reference, *reference)?)?)
            }
        }
    }

    fn value_from_value_ref(&self, value_ref: &ValueRef) -> Result<Value, RuntimeError> {
        match value_ref.value_type {
            ValueType::String => {
                let option = self.vm.get_from_constant_pool(value_ref.reference.ok_or_else(|| RuntimeError::new("String ValueRef does not contains reference. Variable has been not initialized.".to_string().as_str()))?);
                let constant = option.ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", value_ref.reference.unwrap()).as_str()))?;
                Ok(constant.value())
            }
            ValueType::Number => {
                let option = self.vm.get_from_constant_pool(value_ref.reference.ok_or_else(|| RuntimeError::new("Number ValueRef does not contains reference. Variable has been not initialized.".to_string().as_str()))?);
                let constant = option.ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", value_ref.reference.unwrap()).as_str()))?;
                Ok(constant.value())
            }
            _ => Err(RuntimeError::new("value_from_value_ref - Array not implemented yet"))
        }
    }

    fn native_from_stack_entry(&self, stack_entry: StackEntry) -> Result<&Native, RuntimeError> {
        match stack_entry {
            StackEntry::NativeReference(reference) => {
                let native = self.vm.get_from_native_pool(reference).ok_or_else(|| RuntimeError::new(format!("Can't find native in VM native pool for given reference ({})", reference).as_str()))?;
                Ok(native)
            }
            x => Err(RuntimeError::new_string(format!("Expected stack entry to be a reference to Native method but was {:?}", x)))
        }
    }

    fn constant_ref_from_stack_entry(&self, stack_entry: StackEntry, call_frame: &CallFrame, class: &Class, instance: Option<&Instance>) -> Result<u64, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                self.vm.get_from_constant_pool(reference).ok_or_else(|| RuntimeError::new(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(reference)
            }
            StackEntry::LocalVariableReference(reference) => {
                let variable = call_frame.get_local(reference).ok_or_else(|| RuntimeError::new(format!("Can't find local variable in CAllFRAME local variable pool for given reference ({})", reference).as_str()))?;
                let constant_ref = variable.value_ref.borrow().get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| RuntimeError::new(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant_ref)
            }
            StackEntry::InstanceVariableReference(reference) => {
                if instance.is_none() {
                    return Err(RuntimeError::new("constant_ref_from_stack_entry - Can't retrieve constant reference for instance variable in a static(non-instance) context"));
                }
                let variable = instance.unwrap().get_variable(reference).ok_or_else(|| RuntimeError::new(format!("Can't instance variable in INSTANCE variable pool for given reference ({})", reference).as_str()))?;
                let constant_ref = variable.value_ref.borrow().get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| RuntimeError::new(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant_ref)
            }
            StackEntry::StaticVariableReference(reference) => {
                let variable = class.get_variable(reference).ok_or_else(|| RuntimeError::new(format!("Can't find static variable in CLASS static variable pool for given reference ({})", reference).as_str()))?;
                let constant_ref = variable.value_ref.borrow().get_ref();
                self.vm.get_from_constant_pool(constant_ref).ok_or_else(|| RuntimeError::new(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant_ref)
            }
            x => Err(RuntimeError::new_string(format!("Expected stack entry to be a reference to Constant but was {:?}", x)))
        }
    }

    fn handle_native_method(&self, native: &Native, call_frame: &CallFrame, arguments: Vec<Value>) -> Result<(), RuntimeError> {
        match native.name.as_str() {
            "getarg" => {
                let index = arguments[0].number_value() as usize;
                if arguments.len() == 1 && index > (call_frame.arguments_count - 1) {
                    return Err(RuntimeError::Other(format!("Can't call getarg({}) which is greater than number of arguments provided: {}. Maximum allow index is {}. Consider calling getarg with a default value: getarg({}, DEFAULT_VALUE)", index, call_frame.arguments_count, call_frame.arguments_count - 1, index)));
                } else if arguments.len() == 2 && index > (call_frame.arguments_count - 1) {
                    let value = arguments[1].clone();
                    let reference = self.vm.add_in_constant_pool(value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                } else if arguments.len() > 2 {
                    return Err(RuntimeError::Other(String::from("Can't call getarg with more than 2 arguments")));
                } else {
                    let stack_entry = self.stack.peek(call_frame.stack_pointer + index)?;
                    self.stack.push(stack_entry);
                }
            }
            "getarraysize" => {
                println!("getarraysize")
            }
            _ => {
                return Err(RuntimeError::new_string(format!("Native function {} is not handled yet!", native.name)));
            }
        }
        Ok(())
    }
}