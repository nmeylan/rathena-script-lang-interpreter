use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::{io, mem};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::sync::Arc;
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::program::Program;
use crate::lang::stack::{Stack, StackEntry};
use crate::lang::value::{Constant, Function, Native, Scope, Value, ValueRef, Variable};

#[derive(Clone, Debug, Hash)]
pub enum HeapEntry {
    Variable(Variable)
}

impl HeapEntry {
    pub fn value_ref(&self) -> ValueRef {
        match self {
            HeapEntry::Variable(var) => {
                var.value_ref.borrow().clone()
            }
        }
    }
}

pub struct Vm {
    heap: RefCell<HashMap<u64, HeapEntry, NoopHasher>>,
    constants_pool: RefCell<HashMap<u64, Constant, NoopHasher>>,
    native_pool: HashMap<u64, Native, NoopHasher>,
    native_method_handler: Box<dyn NativeMethodHandler>,
}

#[derive(Debug)]
pub enum RuntimeError {
    NoMoreOperations(usize),
    Other(String),
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self::Other(message.to_string())
    }
    pub fn new_string(message: String) -> Self {
        Self::Other(message)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::NoMoreOperations(ip) => f.write_str(&format!(
                "The VM was halted because there were no more operations at the ip {}",
                ip
            )),
            RuntimeError::Other(msg) => f.write_str(msg),
        }
    }
}

pub trait NativeMethodHandler {
    fn handle(&self, native: &Native, params: Vec<Value>) {
        panic!("Native function {}", native.name);
    }
}

impl Vm {
    pub fn new(native_method_handler: Box<dyn NativeMethodHandler>) -> Vm {
        let mut native_pool: HashMap<u64, Native, NoopHasher> = Default::default();
        native_pool.insert(Self::calculate_hash(&"print".to_string()),
                                Native { name: "println".to_string() });
        Self {
            heap: Default::default(),
            constants_pool: Default::default(),
            native_method_handler,
            native_pool
        }
    }

    pub fn execute_program(vm: Arc<Vm>, mut function: Function) -> Result<(), RuntimeError> {
        {
            let mut chunk = &mut function.chunk;
            vm.extend_constant_pool(mem::replace(&mut chunk.constants_storage, HashMap::default()));
        }
        let mut program = Program::new(vm);
        // TODO: init local variable pool
        // TODO: init instance variable pool
        // TODO: init program function pool
        // Surement besoin de passer un chunk plutot que CallFrame dans la fonction run
        program.run(function)
    }

    pub fn extend_constant_pool(&self, constant_pool: HashMap<u64, Constant, NoopHasher>) {
        let mut constant_pool_ref_mut = self.constants_pool.borrow_mut();
        constant_pool_ref_mut.extend(constant_pool);
    }

    pub fn get_from_constant_pool(&self, reference: u64) -> Option<Constant> {
        let constant_pool_ref = self.constants_pool.borrow();
        constant_pool_ref.get(&reference).map(|v| v.clone())
    }

    pub fn add_in_constant_pool(&self, value: Value) -> u64 {
        let constant = match value {
            Value::String(str) => Constant::String(str.unwrap()),
            Value::Number(num) => Constant::Number(num.unwrap()),
        };
        let hash = Self::calculate_hash(&constant);
        self.constants_pool.borrow_mut().insert(hash, constant);
        hash
    }

    pub fn get_from_heap_pool(&self, reference: u64) -> Option<HeapEntry> {
        let heap_ref = self.heap.borrow();
        heap_ref.get(&reference).map(|v| v.clone())
    }

    pub fn get_from_native_pool(&self, reference: u64) -> Option<&Native> {
        self.native_pool.get(&reference)
    }

    pub fn native_method_handler(&self) -> &Box<dyn NativeMethodHandler> {
        &self.native_method_handler
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    pub fn dump(&self) {
        let mut out = io::stdout();
        writeln!(out, "========= Constants Pool =========");
        for (reference, constant) in self.constants_pool.borrow().iter() {
            writeln!(out, "({}) {}", reference, constant);
        }
        writeln!(out, "========= Heap =========");
        for (reference, constant) in self.heap.borrow().iter() {
            writeln!(out, "({}) {:?}", reference, constant);
        }
    }
}