use std::collections::HashMap;
use std::{io};
use std::sync::{Arc};
use std::io::{Stdout, Write};
use std::default::Default;
use std::fmt::format;
use std::fs::read;

use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Function, Native, ValueRef, Variable};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::vm::Vm;
use crate::lang::value::Value;
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{*};
use crate::lang::stack::StackEntry::ConstantPoolReference;
use crate::lang::vm::RuntimeError;

const NATIVE_METHODS_INTERNAL_IMPLEMENTATION: &'static [&'static str] = &[
    "getarg"
];

pub enum CallFrameBreak {
    Return(bool),
    Goto(usize),
}

pub struct Program {
    pub vm: Arc<Vm>,
    stack: Stack,
    functions_pool: HashMap<u64, Function, NoopHasher>,
    instances_variable_pool: HashMap<u64, Variable, NoopHasher>,
}

impl Program {
    pub fn new(vm: Arc<Vm>) -> Self {
        let stack = Stack::new();
        let functions_pool: HashMap<u64, Function, NoopHasher> = Default::default();
        Self {
            vm,
            stack,
            functions_pool,
            instances_variable_pool: Default::default(),
        }
    }

    pub fn run_main(&mut self, function: &mut Function) -> Result<(), RuntimeError> {
        let chunk = &mut function.chunk;
        let function_name = function.name.clone();
        self.functions_pool = std::mem::take(&mut chunk.functions);
        let call_frame = CallFrame::new(chunk, 1, function_name, 0);
        self.run(call_frame, 0).map(|_| ())
    }

    pub fn run(&mut self, call_frame: CallFrame, depth: usize) -> Result<CallFrameBreak, RuntimeError> {
        let mut stdout = io::stdout();
        writeln!(stdout, "=========   VM    ========").unwrap();
        self.vm.dump(&mut stdout);
        writeln!(stdout, "=========   Thread    ========").unwrap();
        self.dump(&mut stdout);
        writeln!(stdout).unwrap();
        writeln!(stdout, "========= Call frame ========").unwrap();
        call_frame.dump(&mut stdout);
        stdout.flush();
        let mut op_index = 0;
        loop {
            if op_index >= call_frame.code.len() {
                break;
            }
            let next_op_code = call_frame.code.get(op_index).unwrap();
            writeln!(stdout, "=========   Executing    ========").unwrap();
            writeln!(stdout, "[{}] {:?}", op_index, next_op_code).unwrap();
            writeln!(stdout, "=========   Stack    ========").unwrap();
            self.dump_stack(&mut stdout, &call_frame);
            stdout.flush();
            match next_op_code {
                OpCode::LoadConstant(reference) => {
                    self.stack.push(StackEntry::ConstantPoolReference(*reference));
                }
                OpCode::Pop => {}
                OpCode::StoreGlobal(_) => {}
                OpCode::LoadGlobal(_) => {}
                OpCode::StoreLocal(reference) => {
                    let stack_entry = self.stack.pop()?;
                    let constant_reference = self.constant_ref_from_stack_entry(stack_entry)?;
                    let variable = call_frame.get_local(*reference).ok_or_else(|| RuntimeError::new(format!("Can't find local variable with reference {}", reference).as_str()))?;
                    variable.set_value_ref(constant_reference);
                }
                OpCode::LoadLocal(reference) => {
                    self.stack.push(StackEntry::LocalVariableReference(*reference));
                }
                OpCode::StoreInstance(_) => {}
                OpCode::LoadInstance(_) => {}
                OpCode::DefineFunction(_) => {}
                OpCode::Equal => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
                    let comparison_result = if v2.is_number() && v1.is_number() {
                        v1.number_value() == 1 || v2.number_value() == 1
                    } else { false };
                    let result_as_number = Value::Number(Some(if comparison_result { 1 } else { 0 }));
                    let reference = self.vm.add_in_constant_pool(result_as_number);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Relational(Relational) => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
                    let comparison_result = if v2.is_number() && v2.is_number() {
                        match Relational {
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
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
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
                OpCode::Subtract => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        return Err(RuntimeError::new(format!("Attempt to substract strings: {} - {}", v1, v2).as_str()));
                    } else {
                        Value::Number(Some(v1.number_value() - v2.number_value()))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Multiply => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        return Err(RuntimeError::new(format!("Attempt to multiply strings: {} - {}", v1, v2).as_str()));
                    } else {
                        Value::Number(Some(v1.number_value() * v2.number_value()))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Divide => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        return Err(RuntimeError::new(format!("Attempt to divide strings: {} - {}", v1, v2).as_str()));
                    } else {
                        Value::Number(Some(v1.number_value() / v2.number_value()))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Modulo => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(&stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(&stack_entry2, &call_frame)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        return Err(RuntimeError::new(format!("Attempt to perform modulo strings: {} - {}", v1, v2).as_str()));
                    } else {
                        Value::Number(Some(v1.number_value() % v2.number_value()))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Not => {}
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
                OpCode::Invoke => {}
                OpCode::CallNative { argument_count, reference } => {
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop()?;
                        arguments.push(self.value_from_stack_entry(&stack_entry, &call_frame)?);
                    }
                    arguments.reverse();
                    let native_method = self.native_from_stack_entry(StackEntry::NativeReference(*reference))?;
                    if NATIVE_METHODS_INTERNAL_IMPLEMENTATION.contains(&native_method.name.as_str()) {
                        self.handle_native_method(native_method, &call_frame, arguments)?;
                    } else {
                        self.vm.native_method_handler().handle(native_method, arguments, &self, &call_frame);
                    }
                }
                OpCode::CallFunction { argument_count, reference } => {
                    let function = self.functions_pool.get(reference).unwrap();
                    let mut chunk = function.chunk.clone();
                    self.vm.extend_constant_pool(std::mem::take(&mut chunk.constants_storage));
                    let stack_pointer = self.stack.len() - argument_count;
                    // Convert function argument to heap ref. If we use current frame variable as argument, we need to retrieve value in sub callframe.
                    // We can do that by putting retrieving constant reference and put them in the stack.
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop()?;
                        arguments.push(self.value_from_stack_entry(&stack_entry, &call_frame)?);
                    }
                    for value in arguments {
                        let reference = self.vm.add_in_constant_pool(value);
                        self.stack.push(ConstantPoolReference(reference));
                    }
                    let new_function_call_frame = CallFrame::new(&mut chunk, stack_pointer, function.name.clone(), *argument_count);
                    let break_type = self.run(new_function_call_frame, depth + 1)?;
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
                    }
                }
                OpCode::Call => {}
                OpCode::Return(not_empty_return) => {
                    return if *not_empty_return {
                        let returned_stack_entry = self.stack.pop()?;
                        self.stack.truncate(call_frame.stack_pointer);
                        let returned_value = self.value_from_stack_entry(&returned_stack_entry, &call_frame)?;
                        let reference = self.vm.add_in_constant_pool(returned_value);
                        self.stack.push(StackEntry::ConstantPoolReference(reference));
                        Ok(CallFrameBreak::Return(true))
                    } else {
                        Ok(CallFrameBreak::Return(false))
                    }
                }
                OpCode::Command => {}
                OpCode::If(jump_to_index) => {
                    let stack_entry = self.stack.pop()?;
                    let value = self.value_from_stack_entry(&stack_entry, &call_frame)?;
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
            }
            op_index += 1;
        }
        writeln!(stdout, "*********   Final status ********").unwrap();
        writeln!(stdout, "=========   Thread   ========").unwrap();
        self.dump(&mut stdout);
        writeln!(stdout, "========= Call frame ========").unwrap();
        call_frame.dump(&mut stdout);
        stdout.flush();
        Ok(CallFrameBreak::Return(false))
    }

    fn dump(&self, out: &mut Stdout) {
        writeln!(out, "========= Functions =========").unwrap();
        for (reference, func) in self.functions_pool.iter() {
            writeln!(out, "({}) {}", reference, func).unwrap();
        }
        writeln!(out, "========= Instance Variables =========").unwrap();
        for (reference, variable) in self.instances_variable_pool.iter() {
            writeln!(out, "({}) {:?}", reference, variable).unwrap();
        }
    }

    fn dump_stack(&self, out: &mut Stdout, call_frame: &CallFrame) {
        if self.stack.contents().is_empty() {
            writeln!(out, "         <empty stack>").unwrap();
        } else {
            for (i, val) in self.stack.contents().iter().enumerate() {
                write!(out, "    [{}]  {:?}", i, val).unwrap();
                let maybe_value = self.value_from_stack_entry(val, call_frame);
                if maybe_value.is_ok() {
                    write!(out, " - {}", maybe_value.unwrap()).unwrap();
                }
                writeln!(out).unwrap();
            }
        }
    }

    fn value_from_stack_entry(&self, stack_entry: &StackEntry, call_frame: &CallFrame) -> Result<Value, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                let constant = self.vm.get_from_constant_pool(*reference).ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant.value())
            }
            StackEntry::HeapReference(reference) => {
                let heap_entry = self.vm.get_from_heap_pool(*reference).ok_or_else(|| RuntimeError::new(format!("Can't find value in VM heap for given reference ({})", reference).as_str()))?;
                let value_ref = heap_entry.value_ref();
                Ok(self.value_from_value_ref(&value_ref)?)
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
                let function = self.functions_pool.get(reference).ok_or_else(|| RuntimeError::new(format!("Can't find function in PROGRAM function pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(function.name.clone())))
            }
            StackEntry::InstanceReference(reference) => {
                let variable = self.instances_variable_pool.get(reference).ok_or_else(|| RuntimeError::new(format!("Can't find instance variable in PROGRAM instance variable pool for given reference ({})", reference).as_str()))?;
                Ok(self.value_from_value_ref(&variable.value_ref.borrow())?)
            }
        }
    }

    fn value_from_value_ref(&self, value_ref: &ValueRef) -> Result<Value, RuntimeError> {
        match value_ref {
            ValueRef::String(reference) => {
                let option = self.vm.get_from_constant_pool(reference.ok_or_else(|| RuntimeError::new("String ValueRef does not contains reference. Variable has been not initialized.".to_string().as_str()))?);
                let constant = option.ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", reference.unwrap()).as_str()))?;
                Ok(constant.value())
            }
            ValueRef::Number(reference) => {
                let option = self.vm.get_from_constant_pool(reference.ok_or_else(|| RuntimeError::new("Number ValueRef does not contains reference. Variable has been not initialized.".to_string().as_str()))?);
                let constant = option.ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", reference.unwrap()).as_str()))?;
                Ok(constant.value())
            }
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

    fn constant_ref_from_stack_entry(&self, stack_entry: StackEntry) -> Result<u64, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                self.vm.get_from_constant_pool(reference).ok_or_else(|| RuntimeError::new(format!("constant_ref_from_stack_entry - Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(reference)
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
                    return Err(RuntimeError::Other(format!("Can't call getarg with more than 2 arguments")));
                } else {
                    let stack_entry = self.stack.peek(call_frame.stack_pointer + index)?;
                    self.stack.push(stack_entry.clone());
                }
            }
            _ => {}
        }
        Ok(())
    }
}