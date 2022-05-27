use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

mod common;

pub fn compile(script: &str) -> Function {
    Compiler::compile("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
}


#[test]
fn simple_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let loop_script = compile(r#"
    .@j = 100;
    for(.@i = 0; .@i < 100; .@i += 1) {
		.@j -= 1;
    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, loop_script);
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn simple_loop_with_already_init() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let loop_script = compile(r#"
    .@j = 100;
    .@i = 0;
    for(;.@i < 100; .@i += 1) {
		.@j -= 1;
    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, loop_script);
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn simple_loop_with_increment_in_the_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let loop_script = compile(r#"
    .@j = 100;
    .@i = 0;
    for(;.@i < 100;) {
        .@i += 1;
		.@j -= 1;
    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, loop_script);
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn nested_simple_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let loop_script = compile(r#"
    .@k = 0;
    for(.@i = 0; .@i < 10; .@i += 1) {
		for(.@j = 10; .@j > 0; .@j -= 1) {
		    .@k += 1;
        }
    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, loop_script);
    // Then
    assert_eq!(10, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
    assert_eq!(100, events.borrow().get("k").unwrap().value.number_value());
}

// #[test]
fn return_should_stop_loop() {

}

#[test]
fn break_should_stop_loop() {
// Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let loop_script = compile(r#"
    .@j = 100;
    .@i = 0;
    for(;; .@i += 1) {
        if (.@i >= 100) {
         break;
        }
        .@j -= 1;
    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, loop_script);
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(1, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn break_should_stop_loop2() {
// Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let loop_script = compile(r#"
    .@j = 100;
    .@i = 0;
    for(;; .@i += 1) {
        if (.@i >= 100) {
         break;
        }

		.@j -= 1;
		if (.@j < 50) {
         break;
        }

    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, loop_script);
    // Then
    assert_eq!(51, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(49, events.borrow().get("j").unwrap().value.number_value());
}