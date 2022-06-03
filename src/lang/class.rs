use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Variable};

pub struct Class {
    pub(crate) name: String,
    pub(crate) functions_pool: HashMap<u64, Function, NoopHasher>,
}

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