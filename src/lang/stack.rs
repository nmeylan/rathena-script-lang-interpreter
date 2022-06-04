use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};


use crate::lang::vm::RuntimeError;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum StackEntry {
    ConstantPoolReference(u64),
    HeapReference(u64),
    LocalVariableReference(u64),
    NativeReference(u64),
    FunctionReference(u64),
    InstanceVariableReference(u64),
}

#[derive(Debug)]
pub struct Stack {
    values: RefCell<Vec<StackEntry>>,
}

impl Default for Stack {
    fn default() -> Self {
        Stack { values: RefCell::new(vec![]) }
    }
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, value: StackEntry) {
        self.values.borrow_mut().push(value)
    }

    pub fn pop(&self) -> Result<StackEntry, RuntimeError> {
        self.values.borrow_mut()
            .pop()
            .ok_or_else(|| RuntimeError::new("Tried to pop an empty stack."))
    }

    pub fn peek(&self, index: usize) -> Result<StackEntry, RuntimeError> {
        self.values.borrow()
            .get(index).cloned()
            .ok_or_else(|| RuntimeError::new_string(format!("Tried to access index \"{}\" is out of bounds \"{}\"", index, self.len() - 1)))
    }
    pub fn contents(&self) -> Ref<Vec<StackEntry>> {
        self.values.borrow()
    }

    pub fn truncate(&self, index: usize) {
        self.values.borrow_mut().truncate(index + 1);
    }

    pub fn len(&self) -> usize {
        self.values.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.borrow().is_empty()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.contents().is_empty() {
            f.write_str("         <empty stack>\n")?;
        } else {
            for (i, val) in self.contents().iter().enumerate() {
                f.write_str(&format!("    [{}]  {:?}\n", i, val))?;
            }
        }
        Ok(())
    }
}
