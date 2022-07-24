use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use crate::lang::error::TemporaryRuntimeError;
use crate::lang::value::{Scope, ValueType};

#[derive(Debug)]
pub struct Array {
    pub(crate) reference: u64,
    pub(crate) values: RwLock<Vec<Option<u64>>>,
    pub(crate) value_type: ValueType,
    pub(crate) name: String,
    pub(crate) scope: Scope,
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference,
            values: RwLock::new(self.values.read().unwrap().clone()),
            value_type: self.value_type.clone(),
            name: self.name.clone(),
            scope: self.scope.clone()
        }
    }
}

impl Array {
    pub fn new(reference: u64, value_type: ValueType, scope: Scope, name: String) -> Self {
        Self {
            reference,
            values: RwLock::new(vec![]),
            value_type,
            name,
            scope
        }
    }

    pub fn is_scope_global(&self) -> bool {
        self.scope.is_global()
    }

    pub fn assign<F>(&self, index: usize, constant_pool_reference: u64, callback: Option<F>)
    where F: Fn(Self)
    {
        let len = self.values.read().unwrap().len();
        if index >= len {
            for _ in len..index + 1 {
                self.values.write().unwrap().push(None);
            }
        }
        self.values.write().unwrap()[index] = Some(constant_pool_reference);
        if let Some(callback) = callback {
            callback(self.clone());
        }
    }

    pub fn get(&self, index: usize) -> Result<Option<u64>, TemporaryRuntimeError> {
        let len = self.values.read().unwrap().len();
        if index >= len {
            return Err(TemporaryRuntimeError::new_string(format!("Array index out of bounds: index {}, length {}", index, len)));
        }
        Ok(*self.values.read().unwrap().get(index).unwrap())
    }

    pub fn remove<F>(&self, index: usize, count: usize, callback: Option<F>)
        where F: Fn(Self)
    {
        let len = self.len();
        self.values.write().unwrap().drain(index..count.min(len));
        if let Some(callback) = callback {
            callback(self.clone());
        }
    }

    pub fn index_of(&self, reference: u64) -> isize {
        self.values.read().unwrap().iter().position(|entry_ref| entry_ref.is_some() && entry_ref.unwrap() == reference)
            .map_or(-1, |index| index as isize)
    }

    pub fn copyarray<F>(&self, source_array: Arc<Array>, destination_array_start_index: usize, source_array_index: usize, count: usize, callback: Option<F>) -> Result<(), TemporaryRuntimeError>
        where F: Fn(Self)
    {
        let mut destination_array_index = destination_array_start_index;
        for index in source_array_index..(source_array_index + count) {
            let value = source_array.get(index)?;
            if let Some(value) = value {
                self.assign::<F>(destination_array_index, value, None);
                destination_array_index += 1;
            } else {
                break;
            }
        }
        if let Some(callback) = callback {
            callback(self.clone());
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.values.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.read().unwrap().is_empty()
    }

    pub fn assign_multiple<F>(&self, start_index: usize, size: usize, value_reference: u64, callback: Option<F>)
        where F: Fn(Self)
    {
        for i in start_index..(start_index + size) {
            self.assign::<F>(i, value_reference, None);
        }
        if let Some(callback) = callback {
            callback(self.clone());
        }
    }
}


impl Hash for Array {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
    }
}