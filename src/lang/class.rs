use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;

use std::sync::{RwLock};
use std::time::SystemTime;
use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::compiler::CompilationDetail;
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Variable};
use crate::lang::vm::{Hashcode, Vm};

#[derive(Debug)]
pub struct Class {
    pub(crate) reference: u64,
    pub(crate) name: String,
    pub(crate) functions_pool: HashMap<u64, Function, NoopHasher>,
    pub(crate) sources: HashMap<u64, Vec<CompilationDetail>, NoopHasher>, // Key is function reference
    pub(crate) instances_references: RwLock<u64>,
    pub(crate) static_variables: RwLock<HashMap<u64, Variable, NoopHasher>>,
    pub(crate) instance_variables: HashMap<u64, Variable, NoopHasher>, // Only instance variables definition
}

impl Class {
    pub fn new(name: String, reference: u64, functions_pool: HashMap<u64, Function, NoopHasher>, sources: HashMap<u64, Vec<CompilationDetail>, NoopHasher>, static_variables: HashMap<u64, Variable, NoopHasher>,
               instance_variables: HashMap<u64, Variable, NoopHasher>) -> Self {
        
        Self {
            reference,
            name,
            functions_pool,
            sources,
            instances_references: RwLock::new(0),
            static_variables: RwLock::new(static_variables),
            instance_variables
        }
    }

    pub fn new_instance(&self) -> Instance {
        *self.instances_references.write().unwrap() += 1;
        Instance {
            reference: Vm::calculate_hash(&self.instances_references.read().unwrap().clone()) & SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64,
            class_name: self.name.clone(),
            variables: RwLock::new(self.instance_variables.clone())
        }
    }
    pub fn get_variable(&self, reference: u64) -> Option<Variable> {
        self.static_variables.read().unwrap().get(&reference).cloned()
    }
    pub fn insert_variable(&self, reference: u64, variable: Variable) {
        self.static_variables.write().unwrap().insert(reference, variable);
    }
}

impl Hash for Class {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.sources.len().hash(state);
        self.functions_pool.len().hash(state);
    }
}

#[derive(Debug)]
pub struct Instance {
    pub reference: u64,
    pub class_name: String,
    pub variables: RwLock<HashMap<u64, Variable, NoopHasher>>,
}

impl Hash for Instance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state)
    }
}

impl Instance {
    pub fn insert_variable(&self, reference: u64, variable: Variable) {
        let mut variable_guard = self.variables.write().unwrap();
        variable_guard.insert(reference, variable);
    }
}

#[derive(Debug, Clone)]
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

impl Hashcode for Class {
    fn hash_code(&self) -> u64 {
        self.reference
    }
}
impl Hashcode for Instance {
    fn hash_code(&self) -> u64 {
        self.reference
    }
}