use std::cell::RefCell;

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;

pub type AccountId = String;
pub type CharId = String;
pub type NpcName = String;
pub type InstanceId = String;

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Constant {
    String(String),
    Number(i32),
}

impl Constant {
    pub fn value(&self) -> Value {
        match self {
            Constant::String(s) => Value::String(Some(s.clone())),
            Constant::Number(n) => Value::Number(Some(*n))
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Value {
    String(Option<String>),
    Number(Option<i32>),
}

impl Value {
    pub fn new_string() -> Value {
        Value::String(None)
    }
    pub fn new_number() -> Value {
        Value::Number(None)
    }
    pub fn is_string(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&Value::new_string())
    }
    pub fn is_number(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&Value::new_number())
    }
    pub fn string_value(&self) -> &String {
        match self {
            Value::String(str) => str.as_ref().unwrap(),
            Value::Number(_) => { panic!("Value is a number not a string.") }
        }
    }
    pub fn number_value(&self) -> i32 {
        match self {
            Value::Number(num) => num.unwrap(),
            Value::String(_) => { panic!("Value is string not a number.") }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum ValueRef {
    String(Option<u64>),
    Number(Option<u64>),
}

#[derive(Debug, Clone)]
pub enum ValueType {
    String,
    Number,
}

impl ValueType {
    pub fn is_string(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&ValueType::String)
    }
    pub fn is_number(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&ValueType::Number)
    }
}

impl ValueRef {
    pub fn new_empty_string() -> ValueRef {
        ValueRef::String(None)
    }
    pub fn new_empty_number() -> ValueRef {
        ValueRef::Number(None)
    }

    pub fn new_string(reference: u64) -> ValueRef {
        ValueRef::String(Some(reference))
    }
    pub fn new_number(reference: u64) -> ValueRef {
        ValueRef::Number(Some(reference))
    }

    pub fn duplicate_with_reference(&self, reference: u64) -> ValueRef {
        match self {
            ValueRef::String(_) => ValueRef::new_string(reference),
            ValueRef::Number(_) => ValueRef::new_number(reference)
        }
    }

    pub fn is_string(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&ValueRef::new_empty_string())
    }

    pub fn is_number(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&ValueRef::new_empty_number())
    }

    pub fn get_ref(&self) -> u64 {
        match self {
            ValueRef::String(reference) => reference.unwrap(),
            ValueRef::Number(reference) => reference.unwrap()
        }
    }

    pub fn is_set(&self) -> bool {
        match self {
            ValueRef::String(reference) => reference.is_some(),
            ValueRef::Number(reference) => reference.is_some()
        }
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

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub scope: Scope,
    pub value_ref: RefCell<ValueRef>,
}

impl Variable {
    pub fn set_value_ref(&self, reference: u64) {
        let mut ref_mut = self.value_ref.borrow_mut();
        *ref_mut = ref_mut.duplicate_with_reference(reference);
    }

    pub fn to_script_identifier(&self) -> String {
        format!("{}{}{}", self.prefix(), self.name, self.suffix())
    }

    pub fn prefix(&self) -> String {
        match self.scope {
            Scope::Server => String::from("$"),
            Scope::Account => String::from(""),
            Scope::Character => String::from("@"),
            Scope::Npc => String::from("."),
            Scope::Instance => String::from("'"),
            Scope::Local => String::from(".@"),
        }
    }

    pub fn suffix(&self) -> String {
        let value_ref = self.value_ref.borrow();
        match value_ref.deref() {
            ValueRef::String(_) => String::from("$"),
            ValueRef::Number(_) => String::from(""),
        }
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
        self.scope.hash(state);
        self.value_ref.borrow().hash(state);
    }
}

impl PartialEq<Self> for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.scope == other.scope
    }
}

impl Eq for Variable {}


#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Native {
    pub name: String,
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
