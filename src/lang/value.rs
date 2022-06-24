use std::cell::RefCell;
use std::env::var;

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;


pub type AccountId = String;
pub type CharId = String;
pub type NpcName = String;
pub type InstanceId = String;

// Variables are struct stored in variable pools (locals, static, instances)
// They don't contains the value, but just reference of these value from constant pool.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub scope: Scope,
    pub value_ref: RefCell<ValueRef>,
}

// Variables scope
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Scope {
    Server,
    Account,
    Character,
    Npc,
    Instance,
    Local,
}

// A structure containing type and reference of a value
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct ValueRef {
    pub reference: Option<u64>,
    pub value_type: ValueType,
}

// Type of a value
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum ValueType {
    String,
    Number,
    Array(Box<ValueType>),
}

// Actual values are contains in this enum
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Constant {
    String(String),
    Number(i32),
}

// Value wrap actual values stored in constant pool.
// Values can be also value behind references.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Value {
    String(Option<String>),
    Number(Option<i32>),
    Reference(Option<(u64, u64)>),
    ArrayEntry(Option<(u64, u64, Option<Constant>, usize)>),
}

impl Scope {
    pub fn prefix(&self) -> String {
        match self {
            Scope::Server => String::from("$"),
            Scope::Account => String::from(""),
            Scope::Character => String::from("@"),
            Scope::Npc => String::from("."),
            Scope::Instance => String::from("'"),
            Scope::Local => String::from(".@"),
        }
    }
}

impl Constant {
    pub fn value(&self) -> Value {
        match self {
            Constant::String(s) => Value::String(Some(s.clone())),
            Constant::Number(n) => Value::Number(Some(*n))
        }
    }

    pub fn is_string(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&Constant::String(String::new()))
    }
    pub fn is_number(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&Constant::Number(0))
    }
}

impl Value {
    pub fn new_string() -> Value {
        Value::String(None)
    }
    pub fn new_number() -> Value {
        Value::Number(None)
    }
    pub fn new_reference() -> Value {
        Value::Reference(None)
    }
    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            Value::ArrayEntry(v) => v.as_ref().unwrap().2.as_ref().unwrap().is_string(),
            _ => false
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            Value::ArrayEntry(v) => v.as_ref().unwrap().2.as_ref().unwrap().is_number(),
            _ => false
        }
    }
    pub fn is_reference(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&Value::new_reference())
    }
    pub fn string_value(&self) -> &String {
        match self {
            Value::String(str) => str.as_ref().unwrap(),
            Value::Number(_) => { panic!("Value is a number not a string.") }
            Value::Reference(_) => { panic!("Value is a reference not a string.") }
            Value::ArrayEntry(entry) => {
                let (_, _, constant, _) = entry.as_ref().unwrap();
                match constant.as_ref().unwrap() {
                    Constant::String(str) => str,
                    Constant::Number(_) => panic!("Value is a number not a string.")
                }
            }
        }
    }
    pub fn number_value(&self) -> i32 {
        match self {
            Value::Number(num) => num.unwrap(),
            Value::String(_) => { panic!("Value is string not a number.") }
            Value::Reference(_) => { panic!("Value is a reference not a number.") }
            Value::ArrayEntry(entry) => {
                let (_, _, constant, _) = entry.as_ref().unwrap();
                println!("constant {}", constant.as_ref().unwrap());
                constant.as_ref().unwrap().value().number_value()
            }
        }
    }
    pub fn reference_value(&self) -> (u64, u64) {
        match self {
            Value::Number(_) => panic!("Value is number not a reference."),
            Value::String(_) => { panic!("Value is string not a reference.") }
            Value::Reference(references) => references.unwrap(),
            Value::ArrayEntry(_) => { panic!("Value is a array entry not a reference.") }
        }
    }
    pub fn array_entry_value(&self) -> &(u64, u64, Option<Constant>, usize) {
        match self {
            Value::Number(_) => panic!("Value is number not a array entry."),
            Value::String(_) => { panic!("Value is string not a array entry.") }
            Value::Reference(_) => { panic!("Value is reference not a array entry.") }
            Value::ArrayEntry(entry) => { entry.as_ref().unwrap() }
        }
    }

    pub fn match_value_type(&self, value_type: &ValueType) -> bool {
        match self {
            Value::String(_) => value_type.is_string(),
            Value::Number(_) => value_type.is_number(),
            Value::Reference(_) => false,
            Value::ArrayEntry(entry) => {
                match entry.as_ref().unwrap().2.as_ref().unwrap() {
                    Constant::String(_) => value_type.is_string(),
                    Constant::Number(_) => value_type.is_number(),
                }
            }
        }
    }

    pub fn display_type(&self) -> String {
        match self {
            Value::String(_) => String::from("String"),
            Value::Number(_) => String::from("Number"),
            Value::Reference(_) => String::from("Reference"),
            Value::ArrayEntry(entry) => match entry.as_ref().unwrap().2.as_ref().unwrap() {
                Constant::String(_) => String::from("String"),
                Constant::Number(_) => String::from("Number"),
            }
        }
    }

    pub fn display_value(&self) -> String {
        match self {
            Value::String(v) => format!("\"{}\"", if let Some(v) = v { v.to_string() } else { "<unitialized>".to_string() }),
            Value::Number(v) => format!("{}", if let Some(v) = v { format!("{}", v) } else { "<unitialized>".to_string() }),
            Value::Reference(v) => format!("#{}", if let Some((owner_ref, reference)) = v { format!("{},{}", owner_ref, reference) } else { "<unitialized>".to_string() }),
            Value::ArrayEntry(entry) => match entry.as_ref().unwrap().2.as_ref().unwrap() {
                Constant::String(constant) => format!("\"{}\"", constant),
                Constant::Number(constant) => format!("{}", constant),
            }
        }
    }
}

impl ValueType {
    pub fn is_string(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&ValueType::String)
    }
    pub fn is_number(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&ValueType::Number)
    }

    pub fn display_type(&self) -> String {
        match self {
            ValueType::String => String::from("String"),
            ValueType::Number => String::from("Number"),
            ValueType::Array(subtype) => subtype.display_type(),
        }
    }

    pub fn match_value(&self, value: &Value) -> bool {
        match self {
            ValueType::String => value.is_string(),
            ValueType::Number => value.is_number(),
            ValueType::Array(subtype) => {
                subtype.match_value(value)
            }
        }
    }
}

impl ValueRef {
    pub fn new_empty_string() -> Self {
        Self {
            reference: None,
            value_type: ValueType::String,
        }
    }
    pub fn new_empty_number() -> Self {
        Self {
            reference: None,
            value_type: ValueType::Number,
        }
    }
    pub fn new_empty_array(value_type: ValueType) -> Self {
        Self {
            reference: None,
            value_type: ValueType::Array(Box::new(value_type)),
        }
    }

    pub fn new_string(reference: u64) -> Self {
        Self {
            reference: Some(reference),
            value_type: ValueType::String,
        }
    }
    pub fn new_number(reference: u64) -> Self {
        Self {
            reference: Some(reference),
            value_type: ValueType::Number,
        }
    }
    pub fn new_array(value_type: ValueType, reference: u64) -> Self {
        Self {
            reference: Some(reference),
            value_type: ValueType::Array(Box::new(value_type)),
        }
    }

    pub fn duplicate_with_reference(&self, reference: u64) -> Self {
        Self {
            reference: Some(reference),
            value_type: self.value_type.clone(),
        }
    }

    pub fn is_string(&self) -> bool {
        mem::discriminant(&self.value_type) == mem::discriminant(&ValueType::String)
    }

    pub fn is_number(&self) -> bool {
        mem::discriminant(&self.value_type) == mem::discriminant(&ValueType::Number)
    }

    pub fn is_string_array(&self) -> bool {
        if let ValueType::Array(value) = &self.value_type {
            value.is_string()
        } else {
            false
        }
    }

    pub fn is_number_array(&self) -> bool {
        if let ValueType::Array(value) = &self.value_type {
            value.is_number()
        } else {
            false
        }
    }

    pub fn is_array(&self) -> bool {
        self.is_number_array() || self.is_string_array()
    }

    pub fn get_ref(&self) -> u64 {
        self.reference.unwrap()
    }

    pub fn is_set(&self) -> bool {
        self.reference.is_some()
    }
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
        self.scope.prefix()
    }

    pub fn suffix(&self) -> String {
        let value_ref = self.value_ref.borrow();
        match value_ref.deref().value_type {
            ValueType::String => String::from("$"),
            ValueType::Number => String::from(""),
            ValueType::Array(_) => {
                if value_ref.is_string_array() {
                    String::from("$[]")
                } else {
                    String::from("[]")
                }
            }
        }
    }

    pub fn from_string(string: &String) -> Self {
        let (scope, scope_len) =  if string.starts_with(&Scope::Local.prefix()){
            (Scope::Local, Scope::Local.prefix().len())
        } else if string.starts_with(&Scope::Character.prefix()) {
            (Scope::Character, Scope::Character.prefix().len())
        } else if string.starts_with(&Scope::Server.prefix()){
            (Scope::Server, Scope::Server.prefix().len())
        } else if string.starts_with(&Scope::Npc.prefix()) {
            (Scope::Npc, Scope::Npc.prefix().len())
        } else if string.starts_with(&Scope::Instance.prefix()) {
            (Scope::Instance, Scope::Instance.prefix().len())
        } else if string.starts_with(&Scope::Account.prefix()) {
            (Scope::Account, Scope::Account.prefix().len())
        } else {
            panic!("Can't find variable scope from string {}", string)
        };
        let mut variable_name = string[scope_len..string.len()].to_string();
        let has_dollar = variable_name.ends_with("$");
        let has_bracket = variable_name.ends_with("]");
        if has_dollar {
            variable_name = variable_name[0..variable_name.len() - 1].to_string();
        }
        if has_bracket {
            variable_name = variable_name[0..variable_name.len() - 3].to_string();
        }
        Self {
            scope,
            value_ref: RefCell::new(Self::variable_value(has_dollar, has_bracket)),
            name: variable_name,
        }
    }

    pub(crate) fn variable_value(has_dollar: bool, has_bracket: bool) -> ValueRef {
        if has_dollar {
            if has_bracket {
                ValueRef::new_empty_array(ValueType::String)
            } else {
                ValueRef::new_empty_string()
            }
        } else if has_bracket {
            ValueRef::new_empty_array(ValueType::Number)
        } else {
            ValueRef::new_empty_number()
        }
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
        self.scope.hash(state);
        let value_type = self.value_ref.borrow().deref().value_type.clone();
        match value_type {
            ValueType::String | ValueType::Number => value_type.hash(state),
            ValueType::Array(v) => {
                v.hash(state);
            }
        }
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
            Value::Reference(references) => write!(f, "{:?}", references),
            Value::ArrayEntry(array_entry) => {
                write!(f, "{}", array_entry.as_ref()
                    .map_or("<no value>".to_string(),
                            |(owner_reference, reference, constant, index)|
                                format!("Array({},{})[{}] {:?}", owner_reference, reference, index, constant)))
            }
        }
    }
}
