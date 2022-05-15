use std::collections::HashMap;
use std::{io};
use std::sync::{Arc};
use std::io::Write;
use std::default::Default;
use std::fmt::format;

use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Function, Native, ValueRef, Variable};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::vm::Vm;
use crate::lang::value::Value;
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{*};
use crate::lang::vm::RuntimeError;

pub struct Program {
    vm: Arc<Vm>,
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

    pub fn run(&mut self, mut function: Function) -> Result<(), RuntimeError> {
        let chunk = &mut function.chunk;
        let function_name = function.name.clone();
        let locals = std::mem::take(&mut chunk.locals);
        let call_frame = CallFrame::new(chunk, 1, function_name, locals);
        println!("=========   VM    ========");
        self.vm.dump();
        println!("=========   Thread    ========");
        self.dump();
        println!();
        println!("========= Call frame ========");
        call_frame.dump();
        for next_op_code in call_frame.code.iter() {
            println!("=========   Stack    ========");
            println!("{}", self.stack);
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
                    println!("locals size: {}", call_frame.locals.len());
                }
                OpCode::LoadLocal(reference) => {
                    self.stack.push(StackEntry::LocalVariableReference(*reference));
                }
                OpCode::StoreInstance(_) => {}
                OpCode::LoadInstance(_) => {}
                OpCode::DefineFunction(_) => {}
                OpCode::Equal => {}
                OpCode::Greater => {}
                OpCode::Less => {}
                OpCode::Add => {
                    let stack_entry1 = self.stack.pop()?;
                    let stack_entry2 = self.stack.pop()?;
                    let v1 = self.value_from_stack_entry(stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(stack_entry2, &call_frame)?;
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
                    let v1 = self.value_from_stack_entry(stack_entry1, &call_frame)?;
                    let v2 = self.value_from_stack_entry(stack_entry2, &call_frame)?;
                    let new_value = if v1.is_string() || v2.is_string() {
                        return Err(RuntimeError::new(format!("Attempt to perform modulo strings: {} - {}", v1, v2).as_str()));
                    } else {
                        Value::Number(Some(v1.number_value() % v2.number_value()))
                    };
                    let reference = self.vm.add_in_constant_pool(new_value);
                    self.stack.push(StackEntry::ConstantPoolReference(reference));
                }
                OpCode::Not => {}
                OpCode::Jump => {}
                OpCode::Invoke => {}
                OpCode::CallNative { argument_count, reference } => {
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop()?;
                        arguments.push(self.value_from_stack_entry(stack_entry, &call_frame)?);
                    }
                    arguments.reverse();
                    let native_method_name = self.native_from_stack_entry(StackEntry::NativeReference(*reference))?;
                    self.vm.native_method_handler().handle(native_method_name, arguments);
                }
                OpCode::Call => {}
                OpCode::Return => {}
                OpCode::Command => {}
            }
        }
        Ok(())
    }

    fn dump(&self) {
        let mut out = io::stdout();
        writeln!(out, "========= Functions =========").unwrap();
        for (reference, func) in self.functions_pool.iter() {
            writeln!(out, "({}) {}", reference, func).unwrap();
        }
        writeln!(out, "========= Instance Variables =========").unwrap();
        for (reference, variable) in self.instances_variable_pool.iter() {
            writeln!(out, "({}) {:?}", reference, variable).unwrap();
        }
    }

    fn value_from_stack_entry(&self, stack_entry: StackEntry, call_frame: &CallFrame) -> Result<Value, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                let constant = self.vm.get_from_constant_pool(reference).ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant.value())
            }
            StackEntry::HeapReference(reference) => {
                let heap_entry = self.vm.get_from_heap_pool(reference).ok_or_else(|| RuntimeError::new(format!("Can't find value in VM heap for given reference ({})", reference).as_str()))?;
                let value_ref = heap_entry.value_ref();
                Ok(self.value_from_value_ref(&value_ref)?)
            }
            StackEntry::LocalVariableReference(reference) => {
                let variable = call_frame.get_local(reference).ok_or_else(|| RuntimeError::new(format!("Can't find local variable in CAllFRAME local variable pool for given reference ({})", reference).as_str()))?;
                Ok(self.value_from_value_ref(&variable.value_ref.borrow())?)
            }
            StackEntry::NativeReference(reference) => {
                let native = self.vm.get_from_native_pool(reference).ok_or_else(|| RuntimeError::new(format!("Can't find native in VM native pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(native.name.clone())))
            }
            StackEntry::FunctionReference(reference) => {
                let function = self.functions_pool.get(&reference).ok_or_else(|| RuntimeError::new(format!("Can't find function in PROGRAM function pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(function.name.clone())))
            }
            StackEntry::InstanceReference(reference) => {
                let variable = self.instances_variable_pool.get(&reference).ok_or_else(|| RuntimeError::new(format!("Can't find instance variable in PROGRAM instance variable pool for given reference ({})", reference).as_str()))?;
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
                self.vm.get_from_constant_pool(reference).ok_or_else(|| RuntimeError::new(format!("Can't find constant in VM native pool for given reference ({})", reference).as_str()))?;
                Ok(reference)
            }
            x => Err(RuntimeError::new_string(format!("Expected stack entry to be a reference to Constant but was {:?}", x)))
        }
    }
}