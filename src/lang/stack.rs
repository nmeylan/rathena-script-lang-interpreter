use std::fmt::{Display, Formatter};


use crate::lang::vm::RuntimeError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum StackEntry {
    ConstantPoolReference(u64),
    HeapReference(u64),
    LocalVariableReference(u64),
    NativeReference(u64),
    FunctionReference(u64),
    InstanceReference(u64),
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
            .ok_or_else(|| RuntimeError::new("Tried to pop an empty stack."))
    }
    pub fn contents(&self) -> &Vec<StackEntry> {
        &self.values
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.contents().is_empty() {
            f.write_str("         <empty stack>\n")?;
        } else {
            for (i, val) in self.values.iter().enumerate() {
                f.write_str(&format!("    [{}]  {:?}\n", i, val))?;
            }
        }
        Ok(())
    }
}
