use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::time::SystemTime;
use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{ValueType, Variable};
use crate::lang::vm::{Hashcode, RuntimeError, Vm};

#[derive(Debug)]
pub struct Class {
    reference: Option<u64>,
    pub(crate) name: String,
    pub(crate) functions_pool: HashMap<u64, Function, NoopHasher>,
    pub(crate) instances_references: RefCell<u64>,
    pub(crate) static_variables: HashMap<u64, Variable, NoopHasher>,
    pub(crate) instance_variables: HashMap<u64, Variable, NoopHasher>, // Only instance variables definition
}

impl Class {
    pub fn new(name: String, functions_pool: HashMap<u64, Function, NoopHasher>, static_variables: HashMap<u64, Variable, NoopHasher>,
               instance_variables: HashMap<u64, Variable, NoopHasher>) -> Self {
        let mut class = Self {
            reference: None,
            name,
            functions_pool,
            instances_references: RefCell::new(0),
            static_variables,
            instance_variables
        };
        class.reference = Some(Vm::calculate_hash(&class) & SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);
        class
    }

    pub fn new_instance(&self) -> Instance {
        *self.instances_references.borrow_mut() += 1;
        Instance {
            reference: Vm::calculate_hash(&self.instances_references.borrow().clone()) & SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64,
            class_name: self.name.clone(),
            variables: self.instance_variables.clone()
        }
    }
    pub fn get_variable(&self, reference: u64) -> Option<&Variable> {
        self.static_variables.get(&reference)
    }
}

impl Hash for Class {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub reference: u64,
    pub class_name: String,
    pub variables: HashMap<u64, Variable, NoopHasher>,
}

impl Hash for Instance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state)
    }
}

impl Instance {
    pub fn get_variable(&self, reference: u64) -> Option<&Variable> {
        self.variables.get(&reference)
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub code: Vec<OpCode>,
    pub locals: HashMap<u64, Variable, NoopHasher>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
    }
}

impl Function {
    pub fn new(name: String) -> Self {
        Self {
            name,
            code: Default::default(),
            locals: Default::default()
        }
    }

    pub fn from_chunk(name: String, chunk: Chunk) -> Self {
        Self {
            name,
            code: mem::take(&mut chunk.op_codes.borrow_mut()),
            locals: mem::take(&mut chunk.locals.borrow_mut())
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}()", self.name)
    }
}

#[derive(Clone, Debug)]
pub struct Array {
    pub(crate) reference: u64,
    pub(crate) values: RefCell<Vec<Option<u64>>>,
    pub(crate) value_type: ValueType,
}

impl Array {
    pub fn new(reference: u64, value_type: ValueType) -> Self {
        Self {
            reference,
            values: RefCell::new(vec![]),
            value_type
        }
    }

    pub fn assign(&self, index: usize, constant_pool_reference: u64) {
        let len = self.values.borrow().len();
        if index >= len {
            for _ in len..index + 1 {
                self.values.borrow_mut().push(None);
            }
        }
        self.values.borrow_mut()[index] = Some(constant_pool_reference);
    }

    pub fn get(&self, index: usize) -> Result<u64, RuntimeError> {
        let len = self.values.borrow().len();
        if index >= len {
            return Err(RuntimeError::new_string(format!("Array index out of bounds: index {}, length {}", index, len)));
        }
        if let Some(value) = self.values.borrow().get(index) {
            Ok(value.unwrap())
        } else {
            Err(RuntimeError::new_string(format!("Array index {} has not been initialized", index)))
        }
    }

    pub fn len(&self) -> usize {
        self.values.borrow().len()
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

impl Hashcode for Class {
    fn hash_code(&self) -> u64 {
        self.reference.unwrap()
    }
}
impl Hashcode for Instance {
    fn hash_code(&self) -> u64 {
        self.reference
    }
}