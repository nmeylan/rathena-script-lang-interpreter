use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use crate::lang::error::TemporaryRuntimeError;
use crate::lang::value::{Scope, ValueType};

#[derive(Debug)]
pub struct Array {
    pub(crate) reference: u64,
    pub(crate) values: RwLock<Vec<Option<u64>>>,
    pub(crate) value_type: ValueType,
    pub(crate) scope: Scope,
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference,
            values: RwLock::new(self.values.read().unwrap().clone()),
            value_type: self.value_type.clone(),
            scope: self.scope.clone()
        }
    }
}

impl Array {
    pub fn new(reference: u64, value_type: ValueType, scope: Scope) -> Self {
        Self {
            reference,
            values: RwLock::new(vec![]),
            value_type,
            scope
        }
    }

    pub fn is_scope_global(&self) -> bool {
        self.scope.is_global()
    }

    pub fn assign(&self, index: usize, constant_pool_reference: u64) {
        let len = self.values.read().unwrap().len();
        if index >= len {
            for _ in len..index + 1 {
                self.values.write().unwrap().push(None);
            }
        }
        self.values.write().unwrap()[index] = Some(constant_pool_reference);
    }

    pub fn get(&self, index: usize) -> Result<Option<u64>, TemporaryRuntimeError> {
        let len = self.values.read().unwrap().len();
        if index >= len {
            return Err(TemporaryRuntimeError::new_string(format!("Array index out of bounds: index {}, length {}", index, len)));
        }
        Ok(*self.values.read().unwrap().get(index).unwrap())
    }

    pub fn remove(&self, index: usize, count: usize) {
        let len = self.len();
        self.values.write().unwrap().drain(index..count.min(len));
    }

    pub fn index_of(&self, reference: u64) -> isize {
        self.values.read().unwrap().iter().position(|entry_ref| entry_ref.is_some() && entry_ref.unwrap() == reference)
            .map_or(-1, |index| index as isize)
    }

    pub fn copyarray(&self, source_array: Arc<Array>, destination_array_start_index: usize, source_array_index: usize, count: usize) -> Result<(), TemporaryRuntimeError> {
        let mut destination_array_index = destination_array_start_index;
        for index in source_array_index..(source_array_index + count) {
            let value = source_array.get(index)?;
            if let Some(value) = value {
                self.assign(destination_array_index, value);
                destination_array_index += 1;
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.values.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.read().unwrap().is_empty()
    }

    pub fn assign_multiple(&self, start_index: usize, size: usize, value_reference: u64) {
        for i in start_index..(start_index + size) {
            self.assign(i, value_reference);
        }
    }
}


impl Hash for Array {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
    }
}