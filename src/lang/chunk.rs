use std::collections::HashMap;

use crate::lang::compiler::CompilationError;
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Constant, Function, Native, Variable};
use crate::lang::vm::Vm;


#[derive(Debug)]
pub struct Chunk {
    pub op_codes: Vec<OpCode>,
    pub references: Vec<u64>,
    pub functions: HashMap<u64, Function, NoopHasher>,
    pub natives: HashMap<u64, Native, NoopHasher>,
    pub globals: HashMap<u64, Variable, NoopHasher>,
    pub instances: HashMap<u64, Variable, NoopHasher>,
    pub locals: HashMap<u64, Variable, NoopHasher>,
    pub constants_storage: HashMap<u64, Constant, NoopHasher>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            op_codes: vec![],
            references: vec![],
            functions: HashMap::with_hasher(NoopHasher::default()),
            natives: HashMap::with_hasher(NoopHasher::default()),
            globals: HashMap::with_hasher(NoopHasher::default()),
            instances: HashMap::with_hasher(NoopHasher::default()),
            locals: HashMap::with_hasher(NoopHasher::default()),
            constants_storage: HashMap::with_hasher(NoopHasher::default()),
        }
    }
}

impl Chunk {
    pub fn emit_op_code(&mut self, op_code: OpCode) {
        println!("emit opcode {:?}", op_code);
        self.op_codes.push(op_code);
    }

    pub fn add_constant(&mut self, constant: Constant) -> u64 {
        let hash = Vm::calculate_hash(&constant);
        self.constants_storage.insert(hash, constant);
        hash
    }

    pub fn add_global(&mut self, variable: Variable) -> u64 {
        let hash = Vm::calculate_hash(&variable);
        self.globals.insert(hash, variable);
        hash
    }

    pub fn add_instance(&mut self, variable: Variable) -> u64 {
        let hash = Vm::calculate_hash(&variable);
        self.instances.insert(hash, variable);
        hash
    }

    pub fn add_local(&mut self, variable: Variable) -> u64 {
        let hash = Vm::calculate_hash(&variable);
        self.locals.insert(hash, variable);
        hash
    }

    pub fn add_function(&mut self, function: Function) -> u64 {
        let hash = Vm::calculate_hash(&function);
        self.functions.insert(hash, function);
        hash
    }

    pub fn add_native(&mut self, native: Native) -> u64 {
        let hash = Vm::calculate_hash(&native);
        self.natives.insert(hash, native);
        hash
    }

    pub fn load_local(&mut self, variable: &Variable) -> Result<u64, CompilationError> {
        let maybe_found = self.locals.iter().find(|(_reference, local)| *local == variable);
        if let Some((reference, _)) = maybe_found {
            Ok(*reference)
        } else {
            Err(CompilationError::UndefinedVariable(format!("Undefined local variable: {}", variable.name)))
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    LoadConstant(u64),
    Pop,
    StoreGlobal(u64),
    LoadGlobal(u64),
    StoreLocal(u64),
    LoadLocal(u64),
    StoreInstance(u64),
    LoadInstance(u64),
    DefineFunction(u64),
    CallNative { reference: u64, argument_count: usize },
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Jump,
    Invoke,
    Call,
    Return,
    Command,
}
