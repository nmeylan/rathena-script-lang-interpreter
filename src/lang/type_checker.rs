
use std::borrow::Borrow;
use std::mem;
use std::ops::Deref;

use antlr_rust::token::Token;

use crate::lang::value::{ValueType, Variable};
use crate::parser::rathenascriptlangparser::RathenaScriptLangParserContext;

pub(crate) struct TypeChecker {
    current_sub_expression_types: Vec<Vec<(Option<ValueType>, Option<String>)>>,
    current_expression_types: Vec<(Option<ValueType>, Option<String>)>,
    current_expression_index: usize,
    debug_enabled: bool,
    script_lines: Vec<String>,
}

impl TypeChecker {
    pub fn new(debug_enabled: bool, script_lines: Vec<String>) -> Self {
        Self {
            current_expression_types: vec![],
            current_sub_expression_types: vec![vec![]],
            current_expression_index: 0,
            debug_enabled,
            script_lines,
        }
    }
    pub fn reset_state(&mut self) {
        self.current_expression_index = 0;
        self.current_sub_expression_types = vec![vec![]];
        self.current_expression_types = vec![];
    }

    pub fn inc_current_expression_index<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), caller: &str) {
        self.current_expression_index += 1;
        self.current_sub_expression_types.push(vec![]);
        self.debug_type_checking(node, format!("inc_current_expression_index - {}", caller).as_str());
    }

    pub fn dec_current_expression_index<'input>(&mut self, should_store_type: bool, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), caller: &str) {
        let value_type = self.current_sub_expression_type();
        if should_store_type {
            let debug_context = self.debug_context(node);
            self.current_expression_types.push((value_type.clone(), debug_context));
        }
        if self.current_expression_index > 0 {
            self.current_sub_expression_types.pop();
            self.current_expression_index -= 1;
            if self.current_expression_index > 0 {
                let debug_context = self.debug_context(node);
                if !should_store_type {
                    self.current_sub_expression_types[self.current_expression_index].push((value_type, debug_context));
                }
            }
        }
        self.debug_type_checking(node, format!("dec_current_expression_index - {}", caller).as_str());
    }

    pub fn pop_current_expression_types(&mut self) -> Option<(Option<ValueType>, Option<String>)> {
        self.current_sub_expression_types[self.current_expression_index].pop()
    }
    pub fn pop_expression_type(&mut self) -> Option<(Option<ValueType>, Option<String>)> {
        self.current_expression_types.pop()
    }

    pub fn clear_current_expression_types(&mut self) {
        self.current_sub_expression_types[self.current_expression_index] = vec![];
    }

    pub fn add_current_assignment_type_from_variable<'input>(&mut self, var: &Variable, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        match var.value_ref.borrow().deref().value_type {
            ValueType::String => self.add_current_assigment_type(ValueType::String, node),
            ValueType::Number => self.add_current_assigment_type(ValueType::Number, node),
            ValueType::Array(_) => {
                if var.value_ref.borrow().is_string_array() {
                    self.current_expression_types.pop();
                    self.add_current_assigment_type(ValueType::String, node)
                } else {
                    self.current_expression_types.pop();
                    self.add_current_assigment_type(ValueType::Number, node)
                }
            }
        }
    }

    pub fn add_current_assigment_type<'input>(&mut self, value_type: ValueType, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        let debug_context = self.debug_context(node);
        self.current_sub_expression_types[self.current_expression_index].push((Some(value_type), debug_context));

        self.debug_type_checking(node, format!("add_current_assigment_type {:?}", node).as_str());
    }

    fn debug_context<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) -> Option<String> {
        
        if self.debug_enabled {
            Some(node.get_text())
        } else {
            None
        }
    }

    pub fn remove_type(&mut self, count: usize) {
        for _i in 0..=count {
            self.current_expression_types.pop();
        }
    }

    pub fn remove_type_at(&mut self, index: usize) {
        if index >= self.current_expression_types.len() {
            return
        }
        self.current_expression_types.remove(index);
    }
    pub fn current_type_len(&mut self) -> usize {
        self.current_sub_expression_types[self.current_expression_index].len()
    }
    pub fn type_len(&mut self) -> usize {
        self.current_expression_types.len()
    }

    pub fn drop_current_type<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) -> Option<ValueType> {
        let assignment_types = mem::take(&mut self.current_expression_types);
        self.debug_type_checking(node, "drop_current_type");
        self.reset_state();
        if assignment_types.is_empty() {
            return None;
        }
        let vec = assignment_types.iter().filter(|(t, _)| t.is_some()).map(|(t, _)| t.clone().unwrap()).collect::<Vec<ValueType>>();
        Self::calculate_type(&vec)
    }

    pub fn current_type<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) -> Option<ValueType> {
        self.debug_type_checking(node, "current_type");
        if self.current_expression_types.is_empty() {
            return None;
        }
        let vec = self.current_expression_types.iter().filter(|(t, _)| t.is_some()).map(|(t, _)| t.clone().unwrap()).collect::<Vec<ValueType>>();
        Self::calculate_type(&vec)
    }

    pub fn current_sub_expression_type(&mut self) -> Option<ValueType> {
        if self.current_sub_expression_types[self.current_expression_index].is_empty() {
            return None;
        }
        let vec = self.current_sub_expression_types[self.current_expression_index].iter().filter(|(t, _)| t.is_some()).map(|(t, _)| t.clone().unwrap()).collect::<Vec<ValueType>>();
        Self::calculate_type(&vec)
    }

    pub fn calculate_type(expression_assignment_types: &Vec<ValueType>) -> Option<ValueType> {
        if expression_assignment_types.is_empty() {
            return None;
        }
        if expression_assignment_types.iter().all(|v| v.is_number()) {
            Some(ValueType::Number)
        } else {
            Some(ValueType::String)
        }
    }

    pub fn current_types_are_same_type(&self) -> bool {
        Self::types_are_same_type(&self.current_sub_expression_types[self.current_expression_index].iter()
            .map(|(v, _)| v.clone()).collect::<Vec<Option<ValueType>>>())
    }

    pub fn types_are_same_type(types: &[Option<ValueType>]) -> bool {
        types.iter().all(|v| v.is_none() || v.as_ref().unwrap().is_number())
            || types.iter().all(|v| v.is_none() || v.as_ref().unwrap().is_string())
    }

    pub fn debug_type_checking<'input, 'a>(&self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), function_name: &'a str) {
        if self.debug_enabled {
            println!("[{}]", function_name);
            println!("L{}. {} -> ", node.start().line, self.script_lines[node.start().get_line() as usize - 1]);
            self.current_expression_types.iter().for_each(|(t, context)| {
                print!("\t[({}) -> {:?}],", context.as_ref().unwrap(), t)
            });
            println!();
            for i in 0..=self.current_expression_index {
                print!("[{}] ", i);
                self.current_sub_expression_types[i].iter().for_each(|(t, context)| {
                    print!("\t[({}) -> {:?}],", context.as_ref().unwrap(), t)
                });
                println!();
            }

            println!();
        }
    }
}