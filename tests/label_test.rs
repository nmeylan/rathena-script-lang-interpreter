use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::{compile_script, Event};

mod common;

#[test]
fn simple_label() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    ItDoesNothing:
        .@b$ = "variable in label 1";
    ItDoesNothing1:
        .@c$ = "variable in label 2";
        vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
    assert_eq!(String::from("variable in label 1"), events.borrow().get("b").unwrap().value.string_value().clone());
    assert_eq!(String::from("variable in label 2"), events.borrow().get("c").unwrap().value.string_value().clone());
}


#[test]
fn simple_label_with_goto() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    goto AssignC;
    Skipped:
        .@b$ = "variable in label 1";
    AssignC:
        .@c$ = "variable in label 2";
    AssignD:
        .@d$ = "variable in label 3";
    vm_dump_locals();

    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
    assert_eq!(true, events.borrow().get("b").is_none());
    assert_eq!(String::from("variable in label 2"), events.borrow().get("c").unwrap().value.string_value().clone());
    assert_eq!(String::from("variable in label 3"), events.borrow().get("d").unwrap().value.string_value().clone());
}

#[test]
fn label_with_goto_inside() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    goto Second;
    First:
        .@b$ = "variable in label 1";
        goto End;
    Second:
        .@c$ = "variable in label 2";
        goto First;
    Third:
        .@d$ = "variable in label 3";
    End:
    vm_dump_locals();
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.borrow().get("b").unwrap().value.string_value().clone());
    assert_eq!(String::from("variable in label 2"), events.borrow().get("c").unwrap().value.string_value().clone());
    assert_eq!(true, events.borrow().get("d").is_none());
}


#[test]
fn label_with_goto_in_a_function() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    goto Second;
    First:
        .@b$ = "variable in label 1";
        goto_end();
    Second:
        .@c$ = "variable in label 2";
        goto First;
    Third:
        .@d$ = "variable in label 3";
        goto First; // Hopefully this is never reach or we would have infinite loop
    End:
        .@endd$ = "the end";

    vm_dump_locals();
    function goto_end {
        goto End;
    }
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.borrow().get("b").unwrap().value.string_value().clone());
    assert_eq!(String::from("variable in label 2"), events.borrow().get("c").unwrap().value.string_value().clone());
    assert_eq!(true, events.borrow().get("d").is_none());
    assert_eq!(String::from("the end"), events.borrow().get("endd").unwrap().value.string_value().clone());
}

#[test]
fn label_with_goto_in_a_nested_function() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    goto Second;
    First:
        .@b$ = "variable in label 1";
        my_func();
    Second:
        .@c$ = "variable in label 2";
        goto First;
    Third:
        .@d$ = "variable in label 3";
        goto First; // Hopefully this is never reach or we would have infinite loop
    End:
        .@endd$ = "the end";

    vm_dump_locals();
    function goto_end {
        goto End;
    }
    function my_func {
        goto_end();
    }
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.borrow().get("b").unwrap().value.string_value().clone());
    assert_eq!(String::from("variable in label 2"), events.borrow().get("c").unwrap().value.string_value().clone());
    assert_eq!(true, events.borrow().get("d").is_none());
    assert_eq!(String::from("the end"), events.borrow().get("endd").unwrap().value.string_value().clone());
}