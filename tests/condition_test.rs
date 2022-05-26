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
fn simple_condition() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let condition_branches = compile(r#"
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
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, condition_branches);
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
    let condition_branches = compile(r#"
    if((1 == 1 || 1) && 1) {
        .@a$ = "i am true";
    }
    .@c$ = "i am not part of condition";
    if((1 == 1 || 1) && 0) {
        .@b$ = "i am false";
    }
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, condition_branches);
    // Then
    assert_eq!(String::from("i am true"), events.borrow().get("a").unwrap().value.string_value().clone());
    assert_eq!(false, events.borrow().get("b").is_some());
    assert_eq!(String::from("i am not part of condition"), events.borrow().get("c").unwrap().value.string_value().clone());
}



#[test]
fn conditional_statements() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a = 1 == 1;
    .@b = 1 == 2;
    .@c = "1" == "1";
    .@d = "1" == "2";
    .@e = 0 == 1 && 1 == 1;
    .@f = (0 == 1 && 1 == 1) || 1 == 1;
    .@g = 0 == 1 && (1 == 1 || 1 == 1);
    vm_dump_locals();
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(1, events.borrow().get("a").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("b").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("c").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("d").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("e").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("f").unwrap().value.number_value().clone());
    assert_eq!(0, events.borrow().get("g").unwrap().value.number_value().clone());
}