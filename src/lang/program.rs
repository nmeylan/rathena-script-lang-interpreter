use std::collections::HashMap;
use std::{io, mem};
use std::fmt::format;
use std::sync::{Arc, RwLock};
use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Function, Native, Scope, Variable};
use crate::{NoopHasher, Value, Vm};
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{*};
use crate::lang::vm::RuntimeError;
use std::io::Write;

pub struct Program {
    vm: Arc<Vm>,
    stack: Stack,
    functions_pool: HashMap<u64, Function, NoopHasher>,
    instances_variable_pool: HashMap<u64, Variable, NoopHasher>,
}

impl Program {
    pub fn new(vm: Arc<Vm>) -> Self {
        let mut stack = Stack::new();
        let mut functions_pool: HashMap<u64, Function, NoopHasher> = Default::default();
        Self {
            vm,
            stack,
            functions_pool,
            instances_variable_pool: Default::default()
        }
    }

    pub fn run(&mut self, call_frame: &CallFrame) -> Result<(), RuntimeError> {
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
                OpCode::StoreLocal(_) => {}
                OpCode::LoadLocal(_) => {}
                OpCode::StoreInstance(_) => {}
                OpCode::LoadInstance(_) => {}
                OpCode::DefineFunction(_) => {}
                OpCode::Equal => {}
                OpCode::Greater => {}
                OpCode::Less => {}
                OpCode::Add => {}
                OpCode::Subtract => {}
                OpCode::Multiply => {}
                OpCode::Divide => {}
                OpCode::Not => {}
                OpCode::Jump => {}
                OpCode::Invoke => {}
                OpCode::CallNative {argument_count, reference} => {
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..*argument_count {
                        let stack_entry = self.stack.pop()?;
                        arguments.push(self.value_from_stack_entry(stack_entry, call_frame)?);
                    }
                    arguments.reverse();
                    let native_method_name = self.native_from_stack_entry(StackEntry::NativeReference(*reference))?;
                    self.vm.native_method_handler().handle(native_method_name, arguments);
                }
                OpCode::Call => {}
                OpCode::Return => {}
                OpCode::Command => {}
            }
            // match next_op_code {
            //     // LoadConstant(reference) => {
            //     //     self.stack.push(StackEntry::ConstantPoolReference(*reference));
            //     // }
            //     Pop => {}
            //     StoreGlobal => {
            //         // let hash = call_frame.next_reference().unwrap();
            //         // call_frame.get_identifier(hash);
            //     }
            //     LoadGlobal => {
            //         // let hash = call_frame.next_reference().unwrap();
            //         // // println!("identifier {}", hash);
            //         // // self.global_identifiers_pool.iter().for_each(|(k, v)| println!("{}, {}", k, v));
            //         // let maybe_global_identifier = self.global_identifiers_pool.get(&hash);
            //         // let identifier = if maybe_global_identifier.is_some() {
            //         //     maybe_global_identifier.unwrap()
            //         // } else {
            //         //     let stack_entry = self.stack.get(hash  as usize + call_frame.stack_pointer)
            //         //         .or(Err(RuntimeError::new("Can't find identifier from global identifier pool nor in the stack")))?;
            //         //     match stack_entry {
            //         //         StackEntry::Identifier(identifier) => identifier,
            //         //         _ => return Err(RuntimeError::new("LoadIdentifier - Expected to find an Identifier from stack but was a reference")),
            //         //     }
            //         // };
            //         // match identifier {
            //         //     Identifier::Variable(var) => {
            //         //         match var.scope {
            //         //             Scope::Server => {
            //         //
            //         //             }
            //         //             Scope::Account => {}
            //         //             Scope::Character => {}
            //         //             Scope::Npc => {}
            //         //             Scope::Instance => {}
            //         //             Scope::Local => {
            //         //                 // self.stack.push(StackEntry::Identifier(identifier));
            //         //             }
            //         //         }
            //         //     }
            //         //     Identifier::Function(_) => {
            //         //
            //         //     }
            //         //     Identifier::Native(_) => {
            //         //         self.stack.push(StackEntry::GlobalIdentifierPoolReference(hash));
            //         //     }
            //         //     Identifier::String(_) => {
            //         //         self.stack.push(StackEntry::GlobalIdentifierPoolReference(hash));
            //         //     }
            //         // }
            //     }
            //     StoreLocal => {
            //
            //     }
            //     LoadLocal => {}
            //     StoreInstance => {}
            //     LoadInstance => {}
            //     Equal => {}
            //     Greater => {}
            //     Less => {}
            //     Add => {}
            //     Subtract => {}
            //     Multiply => {}
            //     Divide => {}
            //     Not => {}
            //     Jump => {}
            //     Invoke => {}
            //     Call => {
            //         // let argument_count = call_frame.next_reference().ok_or(RuntimeError::new("Function arguments count"))?;
            //         // let mut arguments: Vec<Value> = vec![];
            //         // for _ in 0..argument_count {
            //         //     let instruction = self.stack.pop()?;
            //         //     match instruction {
            //         //         StackEntry::Identifier(_) => {
            //         //             panic!("don't know how to handle identifier in function arguments, yet");
            //         //         }
            //         //         StackEntry::ConstantPoolReference(reference) => {
            //         //             arguments.push((self.constants_pool.get(&reference).ok_or(RuntimeError::new("Can't find constant in constant pool"))?).value());
            //         //         }
            //         //         StackEntry::GlobalIdentifierPoolReference(_) => {
            //         //             panic!("don't know how to handle identifier pool reference in function arguments, yet");
            //         //         }
            //         //     }
            //         // }
            //         // arguments.reverse();
            //         // match self.stack.pop()? {
            //         //     StackEntry::Identifier(identifier) => {
            //         //         self.execute_call(arguments, &identifier);
            //         //     },
            //         //     StackEntry::ConstantPoolReference(_) => return Err(RuntimeError::new("Call - Expected call to received an Identifier but was a ConstantPoolReference")),
            //         //     StackEntry::GlobalIdentifierPoolReference(reference) => {
            //         //         let identifier = self.global_identifiers_pool.get(&reference).ok_or(RuntimeError::new(format!("Call - cannot find global identifiers with reference {}", reference).as_str()))?;
            //         //         self.execute_call(arguments, identifier);
            //         //     }
            //         // };
            //
            //     }
            //     Return => {}
            //     Command => {}
            // }
        }
        Ok(())
    }

    // fn execute_call(&self, mut arguments: Vec<Value>, identifier: &Identifier) -> Result<(), RuntimeError> {
    //     match identifier {
    //         Identifier::Function(_) => {}
    //         Identifier::Native(native) => {
    //             self.native_method_handler.handle(native, arguments, &self.global_identifiers_pool, &self.constants_pool);
    //         }
    //         _ => return Err(RuntimeError::new(format!("Expected either a Function or a Native to be called, but it was: {:?}", identifier).as_str()))
    //     }
    //     Ok(())
    // }

    fn dump(&self) {
        // let mut out = io::stdout();
        // writeln!(out, "========= Constants =========");
        // for (reference, constant) in self.constants_pool.iter() {
        //     writeln!(out, "({}) {}", reference, constant);
        // }
        // writeln!(out, "========= Globals =========");
        // for (reference, constant) in self.global_identifiers_pool.iter() {
        //     writeln!(out, "({}) {}", reference, constant);
        // }
    }

    fn value_from_stack_entry(&self, stack_entry: StackEntry, call_frame: &CallFrame) -> Result<Value, RuntimeError> {
        match stack_entry {
            StackEntry::ConstantPoolReference(reference) => {
                let constant = self.vm.get_from_constant_pool(reference).ok_or(RuntimeError::new(format!("Can't find constant in VM constant pool for given reference ({})", reference).as_str()))?;
                Ok(constant.value())
            }
            StackEntry::HeapReference(reference) => {
                let heap_entry = self.vm.get_from_heap_pool(reference).ok_or(RuntimeError::new(format!("Can't find value in VM heap for given reference ({})", reference).as_str()))?;
                Ok(heap_entry.value())
            }
            StackEntry::LocalVariableReference(reference) => {
                let function = call_frame.get_local(reference).ok_or(RuntimeError::new(format!("Can't find local variable in CAllFRAME local variable pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(function.name.clone())))
            }
            StackEntry::NativeReference(reference) => {
                let native =self.vm.get_from_native_pool(reference).ok_or(RuntimeError::new(format!("Can't find native in VM native pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(native.name.clone())))
            }
            StackEntry::FunctionReference(reference) => {
                let function = self.functions_pool.get(&reference).ok_or(RuntimeError::new(format!("Can't find function in PROGRAM function pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(function.name.clone())))
            }
            StackEntry::InstanceReference(reference) => {
                let variable = self.instances_variable_pool.get(&reference).ok_or(RuntimeError::new(format!("Can't find instance variable in PROGRAM instance variable pool for given reference ({})", reference).as_str()))?;
                Ok(Value::String(Some(variable.name.clone())))
            }
        }
    }

    fn native_from_stack_entry(&self, stack_entry: StackEntry) -> Result<&Native, RuntimeError> {
        match stack_entry {
            StackEntry::NativeReference(reference) => {
                let native =self.vm.get_from_native_pool(reference).ok_or(RuntimeError::new(format!("Can't find native in VM native pool for given reference ({})", reference).as_str()))?;
                Ok(native)
            },
            x => Err(RuntimeError::new_string(format!("Expected stack entry to be a reference to Native method but was {:?}", x)))
        }
    }
}