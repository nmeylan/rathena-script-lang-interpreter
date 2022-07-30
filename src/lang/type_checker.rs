use std::borrow::Borrow;
use std::mem;
use std::ops::Deref;
use antlr_rust::token::Token;

use crate::lang::value::{ValueType, Variable};
use crate::parser::rathenascriptlangparser::RathenaScriptLangParserContext;

pub(crate) struct TypeChecker {
    current_expression_types: Vec<Vec<(ValueType, Option<String>)>>,
    current_expression_index: usize,
    debug_enabled: bool,
    script_lines: Vec<String>,
}

impl TypeChecker {
    pub fn new(debug_enabled: bool, script_lines: Vec<String>) -> Self {
        let mut current_expression_types = vec![];
        current_expression_types.push(vec![]);
        Self {
            current_expression_types,
            current_expression_index: 0,
            debug_enabled,
            script_lines
        }
    }
    pub fn reset_state(&mut self) {
        let mut current_expression_types = vec![];
        current_expression_types.push(vec![]);
        self.current_expression_index = 0;
        self.current_expression_types = current_expression_types;
    }

    pub fn inc_current_expression_index(&mut self) {
        self.current_expression_index += 1;
    }

    pub fn dec_current_expression_index(&mut self) {
        if self.current_expression_index > 0 {
            self.current_expression_index -= 1;
        }
    }

    pub fn pop_current_expression_types(&mut self) -> Option<(ValueType, Option<String>)> {
        self.current_expression_types[self.current_expression_index].pop()
    }

    pub fn add_current_assignment_type_from_variable<'input>(&mut self, var: &Variable, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        match var.value_ref.borrow().deref().value_type {
            ValueType::String => self.add_current_assigment_type(ValueType::String, node),
            ValueType::Number => self.add_current_assigment_type(ValueType::Number, node),
            ValueType::Array(_) => {
                if var.value_ref.borrow().is_string_array() {
                    self.add_current_assigment_type(ValueType::String, node)
                } else {
                    self.add_current_assigment_type(ValueType::Number, node)
                }
            }
        }
    }

    pub fn add_current_assigment_type<'input>(&mut self, value_type: ValueType, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        let debug_context = if self.debug_enabled {
            Some(node.get_text())
        } else {
            None
        };
        self.current_expression_types[self.current_expression_index].push((value_type, debug_context))
    }

    pub fn remove_type(&mut self, count: usize) {
        for _i in 0..count {
            self.current_expression_types[self.current_expression_index].pop();
        }
    }

    pub fn remove_type_at(&mut self, index: usize) {
        self.current_expression_types[self.current_expression_index].remove(index);
    }
    pub fn current_type_len(&mut self) -> usize {
        self.current_expression_types[self.current_expression_index].len()
    }

    pub fn drop_current_type(&mut self) -> Option<ValueType> {
        let assignment_types = mem::take(&mut self.current_expression_types);
        self.reset_state();
        if assignment_types.is_empty() {
            return None;
        }
        let mut expression_assignment_types: Vec<Option<ValueType>> = vec![];
        for sub_expression_assigment_types in assignment_types.iter() {
            expression_assignment_types.push(Self::calculate_type(
                &sub_expression_assigment_types.iter().map(|(value_type, _)| Some(value_type.clone())).collect::<Vec<Option<ValueType>>>()));
        }
        Self::calculate_type(&expression_assignment_types)
    }

    pub fn current_type(&mut self) -> Option<ValueType> {
        if self.current_expression_types.is_empty() {
            return None;
        }
        let mut expression_assignment_types: Vec<Option<ValueType>> = vec![];
        for sub_expression_assigment_types in self.current_expression_types.iter() {
            expression_assignment_types.push(Self::calculate_type(
                &sub_expression_assigment_types.iter().map(|(value_type, _)| Some(value_type.clone())).collect::<Vec<Option<ValueType>>>()));
        }
        Self::calculate_type(&expression_assignment_types)
    }

    pub fn calculate_type(expression_assignment_types: &Vec<Option<ValueType>>) -> Option<ValueType> {
        let filter = expression_assignment_types.iter().filter(|v| v.is_some());
        if filter.clone().count() == 0 {
            return None;
        }
        if filter.map(|v| v.as_ref().unwrap()).all(|v| v.is_number()) {
            Some(ValueType::Number)
        } else {
            Some(ValueType::String)
        }
    }

    pub fn current_types_are_same_type(&self) -> bool {
        Self::types_are_same_type(&self.current_expression_types[self.current_expression_index].iter().map(|(v, _)| Some(v.clone())).collect::<Vec<Option<ValueType>>>())
    }

    pub fn types_are_same_type(types: &[Option<ValueType>]) -> bool {
        types.iter().all(|v| v.is_none() || v.as_ref().unwrap().is_number())
            || types.iter().all(|v| v.is_none() || v.as_ref().unwrap().is_string())
    }

    pub fn debug_type_checking<'input>(&self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        if self.debug_enabled {
            println!("L{}. {} -> ", node.start().line, self.script_lines[node.start().get_line() as usize - 1]);
            self.current_expression_types[self.current_expression_index].iter().for_each(|(t, context)| {
                if context.is_some() {
                    print!("\t[({}) -> {:?}],", context.as_ref().unwrap(), t)
                } else {
                    print!("\t{:?},", t)
                }
            });
            println!();
        }
    }
}