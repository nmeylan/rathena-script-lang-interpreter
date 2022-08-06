use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::collections::hash_map::Iter;
use std::hash::{Hash, Hasher};
use crate::lang::noop_hasher::NoopHasher;
use std::io::{Stdout, Write};
use std::sync::Arc;
use std::time::SystemTime;
use crate::lang::chunk::{OpCode};
use crate::lang::class::Function;
use crate::lang::value::Variable;
use crate::lang::vm::{DebugFlag, Hashcode, Vm};

#[derive(Debug)]
pub struct CallFrame {
    debug_flag: u16,
    reference: Option<u64>,
    pub op_codes: Vec<OpCode>,
    pub stack_pointer: usize,
    pub arguments_count: usize,
    pub name: String,
    pub locals: HashMap<u64, Variable, NoopHasher>,
    current_op_code: usize,
}

impl CallFrame {
    pub fn new(function: &Function, stack_pointer: usize, arguments_count: usize, debug_flag: u16) -> Self {
        let mut call_frame = Self {
            debug_flag,
            reference: None,
            op_codes: function.code.clone(),
            stack_pointer,
            current_op_code: 0,
            name: function.name.clone(),
            locals: function.locals.clone(),
            arguments_count,
        };
        call_frame.reference = Some(Vm::calculate_hash(&call_frame) & SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);
        call_frame
    }


    pub fn get_local(&self, reference: u64) -> Option<&Variable> {
        self.locals.get(&reference)
    }

    pub fn locals(&self) -> Iter<'_, u64, Variable> {
        self.locals.iter()
    }

    pub fn dump(&self, out: &mut Stdout, vm: Arc<Vm>) {
        if self.debug_flag & DebugFlag::OpCode.value() == DebugFlag::OpCode.value() {
            writeln!(out, "========= Callframe({}) OpCode =========", self.name).unwrap();
            for (index, op_code) in self.op_codes.iter().enumerate() {
                writeln!(out, "[{}] {:?}", index, op_code).unwrap();
            }
            writeln!(out).unwrap();
        }
        self.dump_locals(out, vm);
    }

    pub fn dump_locals(&self, out: &mut Stdout, vm: Arc<Vm>) {
        if self.debug_flag & DebugFlag::LocalsVariable.value() == DebugFlag::LocalsVariable.value() {
            writeln!(out, "======== Callframe({}) Locals =========", self.name).unwrap();
            for (reference, local) in self.locals.iter() {
                let value_ref = &local.value_ref;
                writeln!(out, "({}) name: {}, scope: {:?}, type: {}, value: {}", reference,
                         local.name, local.scope, value_ref.value_type.display_type(),
                         if let Some(value_reference) = value_ref.reference() {
                             if vm.get_from_constant_pool(value_reference).is_some() {
                                 vm.get_from_constant_pool(value_reference).unwrap().value().display_value()
                             } else { "<not a constant>".to_string() }
                         } else {"<uninitialized>".to_string()}
                ).unwrap();
            }
        }
    }
}

impl Hash for CallFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.op_codes.hash(state);
        self.locals.len().hash(state);
    }
}

impl Hashcode for CallFrame {
    fn hash_code(&self) -> u64 {
        self.reference.unwrap()
    }
}