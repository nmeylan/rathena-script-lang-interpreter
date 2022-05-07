use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::lang::value::{Constant, Identifier};
use crate::lang::vm::RuntimeError;

#[derive(Debug)]
pub enum StackEntry {
    Identifier(Identifier),
    ConstantPoolReference(u64),
    GlobalIdentifierPoolReference(u64),
}

#[derive(Debug)]
pub struct Stack {
    values: Vec<StackEntry>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: vec![] }
    }

    pub fn push(&mut self, value: StackEntry) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Result<StackEntry, RuntimeError> {
        self.values
            .pop()
            .ok_or(RuntimeError::new("Tried to pop an empty stack."))
    }

    pub fn get(&self, index: usize) -> Result<&StackEntry, RuntimeError> {
        self.values.get(index).ok_or(RuntimeError::new(&format!(
            "No value found at index {}.",
            index
        )))
    }

    pub fn set(&mut self, index: usize, value: StackEntry) {
        self.values[index] = value;
    }

    pub fn peek(&self) -> Result<&StackEntry, RuntimeError> {
        self.values
            .last()
            .ok_or(RuntimeError::new("Tried to peek an empty stack"))
    }

    pub fn peek_many(&self, distance: usize) -> Result<&StackEntry, RuntimeError> {
        self.get(self.values.len() - 1 - distance)
    }

    pub fn contents(&self) -> &Vec<StackEntry> {
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
                f.write_str(&format!("    [{}]  {:?}\n", i, val))?;
            }
        }
        Ok(())
    }
}
