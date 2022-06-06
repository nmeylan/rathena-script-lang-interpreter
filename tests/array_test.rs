use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::chunk::ClassFile;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::{compile_script, Event};

mod common;


#[test]
fn simple_array_assignment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@a$[1] = "world";
    .@b$ = .@a$[0] + " " + .@a$[1];
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("b", .@b$);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello"), events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!(String::from("world"), events.borrow().get("a1").unwrap().value.string_value().clone());
    assert_eq!(String::from("hello world"), events.borrow().get("b").unwrap().value.string_value().clone());
}

// #[test]
fn getarraysize() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@a$[1] = "world";
    .@b$[3] = "world";
    .@a_len = getarraysize(.@a$);
    .@b_len = getarraysize(.@b$);
    vm_dump_var("a_len", .@a_len);
    vm_dump_var("b_len", .@b_len);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(2, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(4, events.borrow().get("b_len").unwrap().value.number_value().clone());
}