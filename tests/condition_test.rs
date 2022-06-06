use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::chunk::ClassFile;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::{compile_script, Event};

mod common;

#[test]
fn simple_condition() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    if(1) {
        .@a$ = "i am true";
    } else {
        .@d$ = "i am false";
    }
    .@c$ = "i am not part of condition";
    if(0) {
        .@b$ = "i am false";
    } else {
        .@e$ = "i am true";
    }
    if(0) {
        .@f$ = "i am false";
    } else if (0) {
        .@g$ = "i am false";
    } else {
        .@h$ = "i am true";
    }
    if(1) {
        .@i$ = "i am true";
    }
    if(0) {
        .@j$ = "i am false";
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("i am true"), events.borrow().get("a").unwrap().value.string_value().clone());
    assert_eq!(false, events.borrow().get("b").is_some());
    assert_eq!(String::from("i am not part of condition"), events.borrow().get("c").unwrap().value.string_value().clone());
    assert_eq!(false, events.borrow().get("d").is_some());
    assert_eq!(String::from("i am true"), events.borrow().get("e").unwrap().value.string_value().clone());
    assert_eq!(false, events.borrow().get("f").is_some());
    assert_eq!(false, events.borrow().get("g").is_some());
    assert_eq!(String::from("i am true"), events.borrow().get("h").unwrap().value.string_value().clone());
    assert_eq!(String::from("i am true"), events.borrow().get("i").unwrap().value.string_value().clone());
}

#[test]
fn condition_with_expressions() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    if((1 == 1 || 1) && 1) {
        .@a$ = "i am true";
    }
    .@c$ = "i am not part of condition";
    if((1 == 1 || 1) && 0) {
        .@b$ = "i am false";
    }
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("i am true"), events.borrow().get("a").unwrap().value.string_value().clone());
    assert_eq!(false, events.borrow().get("b").is_some());
    assert_eq!(String::from("i am not part of condition"), events.borrow().get("c").unwrap().value.string_value().clone());
}



#[test]
fn conditional_statements() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a = 1 == 1;
    .@b = 1 == 2;
    .@c = "1" == "1";
    .@d = "1" == "2";
    .@e = 0 == 1 && 1 == 1;
    .@f = (0 == 1 && 1 == 1) || 1 == 1;
    .@g = 0 == 1 && (1 == 1 || 1 == 1);
    .@h = 1000;
    .@i = .@h > 999 && .@h < 10000;
    .@j = .@h > 1000 && .@h < 10000;
    .@k = .@h >= 1000 && .@h < 10000;
    .@l = .@h > 999 && .@h <= 1000;
    .@m = .@h > 999 && .@h < 1000;
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(1, events.borrow().get("a").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("b").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("c").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("d").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("e").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("f").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("g").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("i").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("j").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("k").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("m").unwrap().value.number_value().clone());
}