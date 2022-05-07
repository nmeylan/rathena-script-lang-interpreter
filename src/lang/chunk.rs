use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{BuildHasher, Hash, Hasher};
use std::process::id;
use crate::lang::chunk::OpCode::{*};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Constant, Identifier, Variable};
use crate::Vm;


#[derive(Debug)]
pub struct Chunk {
    pub op_codes: Vec<OpCode>,
    pub references: Vec<u64>,
    pub globals: HashMap<u64, Identifier, NoopHasher>,
    pub instances: HashMap<u64, Identifier, NoopHasher>,
    pub locals: Vec<Identifier>,
    pub constants_storage: HashMap<u64, Constant, NoopHasher>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            op_codes: vec![],
            references: vec![],
            globals: HashMap::with_hasher(NoopHasher::default()),
            instances: HashMap::with_hasher(NoopHasher::default()),
            locals: vec![],
            constants_storage: HashMap::with_hasher(NoopHasher::default()),
        }
    }
}

impl Chunk {
    pub fn emit_op_code(&mut self, op_code: OpCode) {
        println!("emit opcode {:?}", op_code);
        self.op_codes.push(op_code);
    }

    pub fn emit_reference(&mut self, reference: u64) {
        println!("emit reference {:?}", reference);
        self.references.push(reference);
    }

    pub fn add_constant(&mut self, constant: Constant) {
        let hash = Vm::calculate_hash(&constant);
        self.constants_storage.insert(hash, constant);
        self.emit_reference(hash);
    }

    pub fn add_global(&mut self, identifier: Identifier) {
        let hash = Vm::calculate_hash(identifier.value());
        self.globals.insert(hash, identifier);
        self.emit_reference(hash);
    }

    pub fn add_instance(&mut self, identifier: Identifier) {
        let hash = Vm::calculate_hash(identifier.value());
        self.instances.insert(hash, identifier);
        self.emit_reference(hash);
    }

    pub fn add_local(&mut self, identifier: Identifier) {

        self.locals.push(identifier);
        let index = self.locals.len() - 1;
        self.emit_reference(index as u64);
    }

    pub fn load_local(&mut self, identifier: Identifier) -> Result<usize, String> {
        let maybe_found = self.locals.iter().enumerate().find(|(_, local)| **local == identifier);
        if maybe_found.is_some() {
            let (index, _) = maybe_found.unwrap();
            self.emit_reference(index as u64);
            Ok(index)
        } else {
            Err(format!("Can't find local identifier: {}", identifier))
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    LoadConstant,
    Pop,
    StoreGlobal,
    LoadGlobal,
    StoreLocal,
    LoadLocal,
    StoreInstance,
    LoadInstance,
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
