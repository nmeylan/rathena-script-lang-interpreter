use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{io, mem};
use crate::{Identifier, NoopHasher};
use crate::lang::chunk::{Chunk, OpCode};
use std::io::Write;

#[derive(Debug)]
pub struct CallFrame {
    pub code: Vec<OpCode>,
    pub references: Vec<u64>,
    pub identifiers_storage: HashMap<u64, Identifier, NoopHasher>,
    pub stack_pointer: usize,
    pub name: String,
    pub locals: Vec<u64>,
    current_reference: usize,
    current_op_code: usize,
}

impl Display for CallFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.code.len() <= self.current_op_code {
            return f.write_str("<end>\n");
        }
        write!(f, "   OpCode [{}] {:?}", self.current_op_code, self.code[self.current_op_code]);
        f.write_str("\n");
        f.write_str("References");
        write!(f, "[{}] {}", self.current_reference, self.references[self.current_reference]);
        if self.references.len() - 1 > self.current_reference {
            write!(f, "\n          [{}] {}    ", self.current_reference + 1, self.references[self.current_reference + 1]);
        }
        f.write_str("\n")
    }
}

impl CallFrame {
    pub fn new(mut chunk: &mut Chunk, stack_pointer: usize, name: String) -> Self {
        Self {
            code: mem::replace(&mut chunk.op_codes, vec![]),
            references: mem::replace(&mut chunk.references, vec![]),
            identifiers_storage: Default::default(),
            stack_pointer,
            current_reference: 0,
            current_op_code: 0,
            name,
            locals: vec![]
        }
    }

    pub fn get_identifier(&mut self, reference: u64) -> Option<&Identifier> {
        self.identifiers_storage.get(&reference)
    }

    pub fn next_reference(&mut self) -> Option<u64> {
        if self.current_reference >= self.references.len() {
            return None;
        }
        let bytes = self.references[self.current_reference];
        self.current_reference += 1;
        Some(bytes)
    }

    pub fn next_op_code(&mut self) -> Option<&OpCode> {
        if self.current_op_code >= self.code.len() {
            return None;
        }
        let op_code = &self.code[self.current_op_code];
        self.current_op_code += 1;
        Some(op_code)
    }

    pub fn store_local(&mut self, index: usize, ) {

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
        writeln!(out, "======= References =======");
        for (index, reference) in self.references.iter().enumerate() {
            writeln!(out, "[{}] {:?}", index, reference);
        }
        writeln!(out, "");
        writeln!(out, "======== Identifiers =========");
        for ( reference, identifier) in self.identifiers_storage.iter() {
            writeln!(out, "({}) {}", reference, identifier);
        }
    }
}