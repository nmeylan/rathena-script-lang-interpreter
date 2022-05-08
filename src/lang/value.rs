use std::env::var;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::lang::chunk::Chunk;

pub type AccountId = String;
pub type CharId = String;
pub type NpcName = String;
pub type InstanceId = String;

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Constant {
    String(String),
    Number(u32),
}

impl Constant {
    pub fn value(&self) -> Value {
        match self {
            Constant::String(s) => Value::String(Some(s.clone())),
            Constant::Number(n) => Value::Number(Some(n.clone()))
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Value {
    String(Option<String>),
    Number(Option<u32>),
}

impl Value {
    pub fn new_string() -> Value {
        Value::String(None)
    }
    pub fn new_number() -> Value {
        Value::Number(None)
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Scope {
    Server,
    Account,
    Character,
    Npc,
    Instance,
    Local,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) scope: Scope,
    pub value: Value
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub(crate) chunk: Chunk,
    pub nested_functions: Vec<Function>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
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
            arity: 0,
            chunk: Default::default(),
            nested_functions: vec![]
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Native {
    pub(crate) name: String,
}



impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(<{}>)", self.name, self.arity)
    }
}

impl Display for Native {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "native function {}", self.name)
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::String(str) => { f.write_str(str) }
            Constant::Number(num) => { write!(f, "{}", num) }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => { write!(f, "String({})", str.as_ref().map_or("<no value>".to_string(), |v| v.clone())) }
            Value::Number(num) => { write!(f, "{}", num.as_ref().map_or("<no value>".to_string(), |v| v.to_string())) }
        }
    }
}
