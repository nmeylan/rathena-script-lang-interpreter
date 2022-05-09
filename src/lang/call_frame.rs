use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{io, mem};
use crate::{NoopHasher};
use crate::lang::chunk::{Chunk, OpCode};
use std::io::Write;
use crate::lang::value::Variable;

#[derive(Debug)]
pub struct CallFrame {
    pub code: Vec<OpCode>,
    pub stack_pointer: usize,
    pub name: String,
    pub locals: HashMap<u64, Variable, NoopHasher>,
    current_op_code: usize,
}

impl Display for CallFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.code.len() <= self.current_op_code {
            return f.write_str("<end>\n");
        }
        write!(f, "   OpCode [{}] {:?}", self.current_op_code, self.code[self.current_op_code]);
        f.write_str("\n")
    }
}

impl CallFrame {
    pub fn new(mut chunk: &mut Chunk, stack_pointer: usize, name: String, locals: HashMap<u64, Variable, NoopHasher>) -> Self {
        Self {
            code: mem::replace(&mut chunk.op_codes, vec![]),
            stack_pointer,
            current_op_code: 0,
            name,
            locals,
        }
    }

    pub fn next_op_code(&mut self) -> Option<&OpCode> {
        if self.current_op_code >= self.code.len() {
            return None;
        }
        let op_code = &self.code[self.current_op_code];
        self.current_op_code += 1;
        Some(op_code)
    }

    pub fn get_local(&self, reference: u64) -> Option<&Variable> {
        self.locals.get(&reference)
    }

    pub fn as_immut(&mut self) -> &Self {
        self
    }

    pub fn dump(&self) {
        let mut out = io::stdout();
        writeln!(out, "========= OpCode =========");
        for (index, op_code) in self.code.iter().enumerate() {
            writeln!(out, "[{}] {:?}", index, op_code);
        }
        writeln!(out, "");
        writeln!(out, "======== Locals =========");
        for ( reference, local) in self.locals.iter() {
            writeln!(out, "({}) {:?}", reference, local);
        }
    }
}