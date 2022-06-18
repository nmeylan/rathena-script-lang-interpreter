use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::{compile_script, Event};

mod common;

#[test]
fn simple_for_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@j = 100;
    for(.@i = 0; .@i < 100; .@i += 1) {
		.@j -= 1;
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn simple_while_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@j = 100;
    while (.@j >= 0) {
        .@j -= 1;
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(-1, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn simple_while_loop_in_a_function() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    function iterate {
        .@i = getarg(0);
        while (.@i >= 0) {
            .@i -= 1;
        }
        return .@i;
    }
    .@j = iterate(100);
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(-1, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn simple_for_loop_with_already_init() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@j = 100;
    .@i = 0;
    for(;.@i < 100; .@i += 1) {
		.@j -= 1;
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn simple_for_loop_with_increment_in_the_for_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@j = 100;
    .@i = 0;
    for(;.@i < 100;) {
        .@i += 1;
		.@j -= 1;
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn nested_simple_for_loop() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@k = 0;
    for(.@i = 0; .@i < 10; .@i += 1) {
        for(.@j = 10; .@j > 0; .@j -= 1) {
          .@k += 1;
        }
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(10, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value());
    assert_eq!(100, events.borrow().get("k").unwrap().value.number_value());
}

// #[test]
// fn return_should_stop_for_loop() {
//
// }

#[test]
fn break_should_stop_for_loop() {
// Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@j = 100;
    .@i = 0;
    for(;; .@i += 1) {
        if (.@i >= 100) {
         break;
        }
        .@j -= 1;
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(100, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(1, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn break_should_stop_for_loop2() {
// Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(51, events.borrow().get("i").unwrap().value.number_value());
    assert_eq!(49, events.borrow().get("j").unwrap().value.number_value());
}

#[test]
fn break_should_stop_nested_for_loop2() {
// Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@i = 0;
    .@c = 0;
    for(;; .@i += 1) {
        .@j = 0;
        if (.@i > 10) {
         break;
        }
        for(;;) {
            .@c += 1;
            .@j += 1;
            if (.@j > 10) {
             break;
            }
        }

    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(110, events.borrow().get("c").unwrap().value.number_value());
}