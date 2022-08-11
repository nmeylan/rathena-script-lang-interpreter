use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};
use crate::lang::error::{RuntimeError, TemporaryRuntimeError};
use crate::lang::value::Scope;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum StackEntry {
    ConstantPoolReference(u64),
    LocalVariableReference(u64),
    NativeReference(u64),
    FunctionReference(u64),
    InstanceVariableReference(u64),
    StaticVariableReference(u64),
    VariableReference((Scope, u64, u64)), // Scope, owner reference, entry reference
    HeapReference((u64, u64)), // owner reference, entry reference
    ArrayHeapReference((u64, u64, usize)), // owner reference, entry reference, index
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

    pub fn pop(&self) -> Result<StackEntry, TemporaryRuntimeError> {
        self.values.borrow_mut()
            .pop()
            .ok_or_else(|| TemporaryRuntimeError::new_string("Tried to pop an empty stack.".to_string()))
    }

    pub fn peek(&self, index: usize) -> Result<StackEntry, TemporaryRuntimeError> {
        self.values.borrow()
            .get(index).cloned()
            .ok_or_else(|| TemporaryRuntimeError::new_string(format!("Tried to access index \"{}\" is out of bounds \"{}\"", index, self.len() - 1)))
    }

    pub fn peek_last(&self) -> Result<StackEntry, TemporaryRuntimeError> {
        let index = self.len() - 1;
        self.values.borrow()
            .get(index).cloned()
            .ok_or_else(|| TemporaryRuntimeError::new_string(format!("Tried to access index \"{}\" is out of bounds \"{}\"", index, self.len() - 1)))
    }
    pub fn contents(&self) -> Ref<Vec<StackEntry>> {
        self.values.borrow()
    }

    pub fn truncate(&self, index: usize) {
        self.values.borrow_mut().truncate(index);
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
