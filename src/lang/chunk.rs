

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{BuildHasher, Hash, Hasher};
use crate::lang::chunk::OpCode::{*};
use crate::lang::noop_hasher::NoopHasher;

pub type AccountId = String;
pub type CharId = String;
pub type NpcName = String;
pub type InstanceId = String;

#[derive(Debug, Clone, Hash)]
pub enum Value {
    Nil,
    Number(u32),
    String(String),
    Function(Function),
    Native(Native),
}

pub enum Scope {
    Server,
    Account(AccountId),
    Character(CharId),
    Npc(NpcName),
    Instance(InstanceId),
    Local,
}

#[derive(Debug, Clone, Hash)]
pub struct Function {arity: usize, name: String, chunk: usize}
#[derive(Debug, Clone, Hash)]
pub struct Native {
    pub(crate) arity: usize, pub(crate) name: String}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(<{}>)", self.name, self.arity)
    }
}
impl Display for Native {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "native function {}(<{}>)", self.name, self.arity)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => f.write_str("nil"),
            Value::Number(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Function(v) => write!(f, "{}", v),
            Value::Native(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    id: usize,
    parent_chunk: Option<usize>,
    op_codes: Vec<OpCode>,
    bytes: Vec<u64>,
    identifiers_storage: HashMap<u64, String, NoopHasher>,
    constants_storage: HashMap<u64, Value, NoopHasher>,

    current_bytes: usize,
    current_op_code: usize,
}

impl Chunk {
    pub fn new(id: usize, parent_chunk: Option<usize>) -> Self {
        Self {
            id,
            parent_chunk,
            op_codes: vec![],
            bytes: vec![],
            identifiers_storage: HashMap::with_hasher(NoopHasher::default()),
            constants_storage: HashMap::with_hasher(NoopHasher::default()),
            current_bytes: 0,
            current_op_code: 0,
        }
    }

    pub fn emit_op_code(&mut self, op_code: OpCode) {
        self.op_codes.push(op_code);
    }

    pub fn emit_bytes(&mut self, bytes: u64) {
        self.bytes.push(bytes);
    }

    pub fn add_constant(&mut self, constant: Value) {
        let hash = Self::calculate_hash(&constant);
        self.constants_storage.insert(hash, constant);
        self.emit_bytes(hash);
    }

    pub fn add_identifiers(&mut self, identifier: String) {
        let hash = Self::calculate_hash(&identifier);
        self.identifiers_storage.insert(hash,identifier);
        self.emit_bytes(hash);
    }

    pub fn next_bytes(&mut self) -> Option<u64> {
        if self.current_bytes >= self.bytes.len() {
            return None;
        }
        let bytes = self.bytes[self.current_bytes];
        self.current_bytes += 1;
        Some(bytes)
    }

    pub fn next_op_code(&mut self) -> Option<&OpCode> {
        if self.current_op_code >= self.op_codes.len() {
            return None;
        }
        let op_code = &self.op_codes[self.current_op_code];
        self.current_op_code += 1;
        Some(op_code)
    }

    pub fn next_constant(&mut self) -> Option<&Value> {
        let maybe_bytes = self.next_bytes();
        if maybe_bytes.is_none() {
            return None;
        }
        self.constants_storage.get(&maybe_bytes.unwrap())
    }
    pub fn next_identifiers(&mut self) -> Option<&String> {
        let maybe_bytes = self.next_bytes();
        if maybe_bytes.is_none() {
            return None;
        }
        self.identifiers_storage.get(&maybe_bytes.unwrap())
    }

    pub fn reset(&mut self) {
        self.current_bytes = 0;
        self.current_op_code = 0;
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

#[derive(Debug)]
pub enum OpCode {
    Constant,
    Pop,
    DefineIdentifier,
    SetIdentifier,
    GetIdentifier,
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
