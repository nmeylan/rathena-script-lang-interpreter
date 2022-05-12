mod common;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use antlr_rust::InputStream;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use common::Event;

pub fn compile(script: &str) -> Function {
    let char_stream = InputStream::new(script);
    Compiler::compile("test_script".to_string(), char_stream).unwrap()
}

#[test]
fn simple_assigment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = "hello world";
    vm_dump_var("a", .@a$);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn assignment_with_string_concat() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = "hello" + " world " + 1;
    vm_dump_var("a", .@a$);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(String::from("hello world 1"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn assignment_with_number_addition() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a = 1 + 1;
    vm_dump_var("a", .@a);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(2, events.borrow().get("a").unwrap().value.number_value().clone());
}

#[test]
fn simple_re_assigment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = "hello world";
    .@a$ = "hello wrld";
    vm_dump_var("a", .@a$);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(String::from("hello wrld"), events.borrow().get("a").unwrap().value.string_value().clone());
}
#[test]
fn plus_equal_string_concat() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = "hello";
    .@a$ += " world";
    vm_dump_var("a", .@a$);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}
#[test]
fn plus_equal_num_addition() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = 1;
    .@a$ += 2;
    vm_dump_var("a", .@a$);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(3 as u32, events.borrow().get("a").unwrap().value.number_value());
}