use std::env::var;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::lang::chunk::Chunk;

pub type AccountId = String;
pub type CharId = String;
pub type NpcName = String;
pub type InstanceId = String;

#[derive(Debug, Hash)]
pub enum Instruction {
    Identifier(Identifier),
    Constant(Constant),
}

#[derive(Debug, Hash, PartialEq)]
pub enum Identifier {
    Variable(Variable),
    Function(Function),
    Native(Native),
    String(String),
}

impl Identifier {
    pub fn value(&self) -> &String {
        match self {
            Identifier::Variable(v) => &v.name,
            Identifier::Function(f) => &f.name,
            Identifier::Native(n) => &n.name,
            Identifier::String(s) => &s
        }
    }

    pub fn to_variable(&self) -> Result<&Variable, String> {
        match self {
            Identifier::Variable(variable) => Ok(variable),
            v => Err(format!("Expected identifier to be variable but was {}", v))
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Constant {
    String(String),
    Number(u32),
}

impl Constant {
    pub fn value(&self) -> Value {
        match self {
            Constant::String(s) => Value::String(s.clone()),
            Constant::Number(n) => Value::Number(n.clone())
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Value {
    String(String),
    Number(u32),
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
    pub variable_type: VariableType,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum VariableType {
    String,
    Number
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
    pub(crate) arity: usize,
    pub(crate) name: String,
}


impl Identifier {
    pub fn can_access(&self) -> bool {
        true // TODO handle scope, add context argument.
    }
}


impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Identifier(_) => { f.write_str("Identifier") }
            Instruction::Constant(_) => { f.write_str("Constant") }
        }
    }
}

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
            Value::String(str) => { f.write_str(str) }
            Value::Number(num) => { write!(f, "{}", num) }
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::String(v) =>  write!(f, "{}", v),
            Identifier::Function(v) => write!(f, "function {}({})", v.name, v.arity),
            Identifier::Native(v) => write!(f, "<native> {}({})", v.name, v.arity),
            Identifier::Variable(v) => write!(f, "Variable {}", v.name),
        }
    }
}