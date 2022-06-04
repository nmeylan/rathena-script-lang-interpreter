use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::collections::hash_map::Iter;
use crate::lang::noop_hasher::NoopHasher;
use std::io::{Stdout, Write};
use crate::lang::chunk::{OpCode};
use crate::lang::class::Function;
use crate::lang::value::Variable;

#[derive(Debug)]
pub struct CallFrame {
    pub code: Vec<OpCode>,
    pub stack_pointer: usize,
    pub arguments_count: usize,
    pub name: String,
    pub locals: HashMap<u64, Variable, NoopHasher>,
    current_op_code: usize,
}

impl Display for CallFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.code.len() <= self.current_op_code {
            return f.write_str("<end>\n");
        }
        write!(f, "   OpCode [{}] {:?}", self.current_op_code, self.code[self.current_op_code]).unwrap();
        f.write_str("\n")
    }
}

impl CallFrame {
    pub fn new(function: &Function, stack_pointer: usize, arguments_count: usize) -> Self {
        Self {
            code: function.code.clone(),
            stack_pointer,
            current_op_code: 0,
            name: function.name.clone(),
            locals: function.locals.clone(),
            arguments_count
        }
    }

    pub fn get_local(&self, reference: u64) -> Option<&Variable> {
        self.locals.get(&reference)
    }

    pub fn locals(&self) -> Iter<'_, u64, Variable> {
        self.locals.iter()
    }

    pub fn dump(&self, out: &mut Stdout) {
        writeln!(out, "========= OpCode =========").unwrap();
        for (index, op_code) in self.code.iter().enumerate() {
            writeln!(out, "[{}] {:?}", index, op_code).unwrap();
        }
        writeln!(out).unwrap();
        writeln!(out, "======== Locals =========").unwrap();
        for ( reference, local) in self.locals.iter() {
            writeln!(out, "({}) {:?}", reference, local).unwrap();
        }
    }
}