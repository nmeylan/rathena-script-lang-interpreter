mod common;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use common::Event;

pub fn compile(script: &str) -> Function {
    Compiler::compile("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
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
fn assignment_with_number_operation() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a = 1 + 1;
    .@b = 4 - 1;
    .@c = 4 * 2;
    .@d = 4 / 2;
    .@e = 3 % 2;
    .@i = 4 * .@c;
    .@f = 2 + 3 * 2;
    .@g = 1 + (2 + 3) * (2 + (10 / 2 + 7) * 2 + (2 + 2 * 3));
    vm_dump_locals();"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(2, events.borrow().get("a").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b").unwrap().value.number_value().clone());
    assert_eq!(8, events.borrow().get("c").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("d").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("e").unwrap().value.number_value().clone());
    assert_eq!(32, events.borrow().get("i").unwrap().value.number_value().clone());
    assert_eq!(8, events.borrow().get("f").unwrap().value.number_value().clone());
    assert_eq!(171, events.borrow().get("g").unwrap().value.number_value().clone());
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
    .@a = 1;
    .@a += 2;
    vm_dump_var("a", .@a);"#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(3_i32, events.borrow().get("a").unwrap().value.number_value());
}