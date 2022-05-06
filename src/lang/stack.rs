use std::fmt::{Display, Formatter};
use crate::lang::chunk::Value;
use crate::lang::vm::RuntimeError;

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.values
            .pop()
            .ok_or(RuntimeError::new("Tried to pop an empty stack."))
    }

    pub fn pop_number(&mut self) -> Result<u32, RuntimeError> {
        match self.pop()? {
            Value::Number(n) => Ok(n),
            v => Err(RuntimeError::new(&format!(
                "Expected to pop a number but found '{}'.\n{}",
                v, self
            ))),
        }
    }

    pub fn pop_string(&mut self) -> Result<String, RuntimeError> {
        match self.pop()? {
            Value::String(s) => Ok(s),
            v => Err(RuntimeError::new(&format!(
                "Expected to pop a string but found '{}'.",
                v
            ))),
        }
    }

    pub fn get(&self, index: usize) -> Result<&Value, RuntimeError> {
        self.values.get(index).ok_or(RuntimeError::new(&format!(
            "No value found at index {}.",
            index
        )))
    }

    pub fn set(&mut self, index: usize, value: Value) {
        self.values[index] = value;
    }

    pub fn peek(&self) -> Result<&Value, RuntimeError> {
        self.values
            .last()
            .ok_or(RuntimeError::new("Tried to peek an empty stack"))
    }

    pub fn peek_many(&self, distance: usize) -> Result<&Value, RuntimeError> {
        self.get(self.values.len() - 1 - distance)
    }

    pub fn contents(&self) -> &Vec<Value> {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.contents().len() == 0 {
            f.write_str("         <empty stack>\n")?;
        } else {
            for (i, val) in self.values.iter().enumerate() {
                f.write_str(&format!("    [{}]  {}\n", i, val))?;
            }
        }
        Ok(())
    }
}
