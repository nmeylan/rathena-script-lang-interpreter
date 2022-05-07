use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::{io, mem};
use std::io::Write;
use std::rc::Rc;
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Constant, Function, Identifier, Native, Scope, Value};

pub struct Vm {
    stack: Stack,
    global_identifiers_pool: HashMap<u64, Identifier, NoopHasher>,
    constants_pool: HashMap<u64, Constant, NoopHasher>,
    native_method_handler: Box<dyn NativeMethodHandler>,
}

#[derive(Debug)]
pub enum RuntimeError {
    NoMoreOperations(usize),
    Other(String),
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self::Other(message.to_string())
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::NoMoreOperations(ip) => f.write_str(&format!(
                "The VM was halted because there were no more operations at the ip {}",
                ip
            )),
            RuntimeError::Other(msg) => f.write_str(msg),
        }
    }
}

pub trait NativeMethodHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, identifiers_pool: &HashMap<u64, Identifier, NoopHasher>, constant_pool: &HashMap<u64, Constant, NoopHasher>) {
        panic!("Native function {}", native.name);
    }
}

impl Vm {
    pub fn new(native_method_handler: Box<dyn NativeMethodHandler>) -> Vm {
        let mut identifiers_pool: HashMap<u64, Identifier, NoopHasher> = Default::default();
        identifiers_pool.insert(Self::calculate_hash(&"print".to_string()),
                                Identifier::Native(Native { arity: 255, name: "println".to_string() }));
        Self {
            stack: Stack::new(),
            global_identifiers_pool: identifiers_pool,
            native_method_handler,
            constants_pool: Default::default(),
        }
    }

    pub fn run_main(&mut self, mut function: Function) -> Result<(), RuntimeError> {
        let mut chunk = &mut function.chunk;
        self.constants_pool.extend(mem::replace(&mut chunk.constants_storage, HashMap::default()));
        let function_name = function.name.clone();
        let mut call_frame = CallFrame::new(chunk, self.stack.len() + 1, function_name);
        self.stack.push(StackEntry::Identifier(Identifier::Function(function)));
        self.run(&mut call_frame)
    }

    pub fn run(&mut self, call_frame: &mut CallFrame) -> Result<(), RuntimeError> {
        println!("=========   VM    ========");
        self.dump();
        println!();
        println!("========= Call frame ========");
        call_frame.dump();
        loop {
            println!("=========   Stack    ========");
            println!("{}", self.stack);
            let maybe_next_op_code = call_frame.next_op_code();
            if maybe_next_op_code.is_none() {
                break;
            }
            let next_op_code = maybe_next_op_code.unwrap();

            match next_op_code {
                OpCode::LoadConstant => {
                    let hash = call_frame.next_reference().unwrap();
                    self.stack.push(StackEntry::ConstantPoolReference(hash));
                }
                OpCode::Pop => {}
                OpCode::StoreGlobal => {
                    let hash = call_frame.next_reference().unwrap();
                    call_frame.get_identifier(hash);
                }
                OpCode::LoadGlobal => {
                    let hash = call_frame.next_reference().unwrap();
                    // println!("identifier {}", hash);
                    // self.global_identifiers_pool.iter().for_each(|(k, v)| println!("{}, {}", k, v));
                    let maybe_global_identifier = self.global_identifiers_pool.get(&hash);
                    let identifier = if maybe_global_identifier.is_some() {
                        maybe_global_identifier.unwrap()
                    } else {
                        let stack_entry = self.stack.get(hash  as usize + call_frame.stack_pointer)
                            .or(Err(RuntimeError::new("Can't find identifier from global identifier pool nor in the stack")))?;
                        match stack_entry {
                            StackEntry::Identifier(identifier) => identifier,
                            _ => return Err(RuntimeError::new("LoadIdentifier - Expected to find an Identifier from stack but was a reference")),
                        }
                    };
                    match identifier {
                        Identifier::Variable(var) => {
                            match var.scope {
                                Scope::Server => {

                                }
                                Scope::Account => {}
                                Scope::Character => {}
                                Scope::Npc => {}
                                Scope::Instance => {}
                                Scope::Local => {
                                    // self.stack.push(StackEntry::Identifier(identifier));
                                }
                            }
                        }
                        Identifier::Function(_) => {

                        }
                        Identifier::Native(_) => {
                            self.stack.push(StackEntry::GlobalIdentifierPoolReference(hash));
                        }
                        Identifier::String(_) => {
                            self.stack.push(StackEntry::GlobalIdentifierPoolReference(hash));
                        }
                    }
                }
                OpCode::StoreLocal => {

                }
                OpCode::LoadLocal => {}
                OpCode::StoreInstance => {}
                OpCode::LoadInstance => {}
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
                OpCode::Call => {
                    let argument_count = call_frame.next_reference().ok_or(RuntimeError::new("Function arguments count"))?;
                    let mut arguments: Vec<Value> = vec![];
                    for _ in 0..argument_count {
                        let instruction = self.stack.pop()?;
                        match instruction {
                            StackEntry::Identifier(_) => {
                                panic!("don't know how to handle identifier in function arguments, yet");
                            }
                            StackEntry::ConstantPoolReference(reference) => {
                                arguments.push((self.constants_pool.get(&reference).ok_or(RuntimeError::new("Can't find constant in constant pool"))?).value());
                            }
                            StackEntry::GlobalIdentifierPoolReference(_) => {
                                panic!("don't know how to handle identifier pool reference in function arguments, yet");
                            }
                        }
                    }
                    arguments.reverse();
                   match self.stack.pop()? {
                        StackEntry::Identifier(identifier) => {
                            self.execute_call(arguments, &identifier);
                        },
                        StackEntry::ConstantPoolReference(_) => return Err(RuntimeError::new("Call - Expected call to received an Identifier but was a ConstantPoolReference")),
                        StackEntry::GlobalIdentifierPoolReference(reference) => {
                            let identifier = self.global_identifiers_pool.get(&reference).ok_or(RuntimeError::new(format!("Call - cannot find global identifiers with reference {}", reference).as_str()))?;
                            self.execute_call(arguments, identifier);
                        }
                    };

                }
                OpCode::Return => {}
                OpCode::Command => {}
            }
        }
        Ok(())
    }

    fn execute_call(&self, mut arguments: Vec<Value>, identifier: &Identifier) -> Result<(), RuntimeError> {
        match identifier {
            Identifier::Function(_) => {}
            Identifier::Native(native) => {
                self.native_method_handler.handle(native, arguments, &self.global_identifiers_pool, &self.constants_pool);
            }
            _ => return Err(RuntimeError::new(format!("Expected either a Function or a Native to be called, but it was: {:?}", identifier).as_str()))
        }
        Ok(())
    }

    fn dump(&self) {
        let mut out = io::stdout();
        writeln!(out, "========= Constants =========");
        for (reference, constant) in self.constants_pool.iter() {
            writeln!(out, "({}) {}", reference, constant);
        }
        writeln!(out, "========= Globals =========");
        for (reference, constant) in self.global_identifiers_pool.iter() {
            writeln!(out, "({}) {}", reference, constant);
        }
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}