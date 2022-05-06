use std::collections::HashMap;
use std::fmt::Display;
use crate::lang::chunk::{Chunk, Native, OpCode, Scope, Value};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::stack::Stack;

pub struct Vm {
    stack: Stack,
    identifiers_pool: HashMap<u64, Identifier, NoopHasher>,
    constant_pool: HashMap<u64, Value, NoopHasher>,
}

pub struct Identifier {
    name: String,
    value: Value,
    scope: Scope,
}

impl Identifier {
    pub fn can_access(&self) -> bool {
        true // TODO handle scope, add context argument.
    }
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

impl Vm {
    pub fn new() -> Vm {
        let mut identifiers_pool: HashMap<u64, Identifier, NoopHasher> = Default::default();
        identifiers_pool.insert(Chunk::calculate_hash(&"print".to_string()), Identifier { name: "print".to_string(), value: Value::Native(Native { arity: 255, name: "println".to_string() }), scope: Scope::Server });
        Self {
            stack: Stack::new(),
            identifiers_pool,
            constant_pool: Default::default(),
        }
    }
    pub fn test(&mut self, chunks: &mut Vec<Chunk>) -> Result<(), RuntimeError> {
        for chunk in chunks.iter_mut() {
            loop {
                let maybe_next_op_code = chunk.next_op_code();
                if maybe_next_op_code.is_none() {
                    return Ok(());
                }
                let next_op_code = maybe_next_op_code.unwrap();
                match next_op_code {
                    OpCode::Constant => {
                        let constant = chunk.next_constant()
                            .ok_or(RuntimeError::new("Expected to find constant"))?;
                        self.stack.push(constant.clone());
                    }
                    OpCode::Pop => {}
                    OpCode::GetIdentifier => {
                        let identifier = self.identifiers_pool.get(&chunk.next_bytes().unwrap())
                            .ok_or(RuntimeError::new("Expected to find identifier"))?;
                        self.stack.push(identifier.value.clone());
                    }
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
                        let argument_count = chunk.next_bytes().ok_or(RuntimeError::new("Function arguments count"))?;
                        let mut arguments: Vec<Value> = vec![];
                        for _ in 0..argument_count {
                            arguments.push(self.stack.pop()?)
                        }
                        arguments.reverse();
                        let value = self.stack.pop()?;
                        match value {
                            Value::Function(_) => {}
                            Value::Native(native) => {
                                if native.name.eq("println") {
                                    println!("{}", arguments.iter().fold(String::new(), |acc, arg| acc + format!("{}", arg).as_str()));
                                }
                            }
                            _ => return Err(RuntimeError::new(format!("Expected either a Function or a Native to be called, but it was: {:?}", value).as_str()))
                        }

                    }
                    OpCode::Return => {}
                    OpCode::Command => {}
                    OpCode::DefineIdentifier => {}
                    OpCode::SetIdentifier => {}
                }
            }
        }
        Ok(())
    }
}