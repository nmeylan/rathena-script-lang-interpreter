use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Variable};

#[derive(Debug)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) functions_pool: HashMap<u64, Function, NoopHasher>,
    pub(crate) instances_references: RefCell<u64>,
    pub(crate) instance_variables: HashMap<u64, Variable, NoopHasher>, // Only instance variables definition
}

impl Class {
    pub fn new(name: String, functions_pool: HashMap<u64, Function, NoopHasher>, instance_variables: HashMap<u64, Variable, NoopHasher>) -> Self {
        Self {
            name,
            functions_pool,
            instances_references: RefCell::new(0),
            instance_variables
        }
    }

    pub fn new_instance(&self) -> Instance {
        *self.instances_references.borrow_mut() += 1;
        Instance {
            reference: self.instances_references.borrow().clone(),
            class_name: self.name.clone(),
            variables: self.instance_variables.clone()
        }
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