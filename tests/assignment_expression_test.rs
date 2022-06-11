mod common;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use common::Event;
use ragnarok_script_interpreter::lang::chunk::ClassFile;
use crate::common::compile_script;


#[test]
fn simple_assigment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    vm_dump_var("a", .@a$);"#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn assigment_to_local_variable() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    .@b$ = .@a$;
    vm_dump_var("a", .@a$);
    vm_dump_var("b", .@b$);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
    assert_eq!(String::from("hello world"), events.borrow().get("b").unwrap().value.string_value().clone());
}

#[test]
fn assignment_with_string_concat() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello" + " world " + 1;
    vm_dump_var("a", .@a$);"#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello world 1"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn assignment_with_number_operation() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a = 1 + 1;
    .@b = 4 - 1;
    .@c = 4 * 2;
    .@d = 4 / 2;
    .@e = 3 % 2;
    .@i = 4 * .@c;
    .@f = 2 + 3 * 2;
    .@g = 1 + (2 + 3) * (2 + (10 / 2 + 7) * 2 + (2 + 2 * 3));
    .@h = 2 - 3 + 2 - 1 + (2 - 3 - 1 + 1);
    set .@a1, 1 + 1;
    set .@b1, 4 - 1;
    set .@c1, 4 * 2;
    set .@d1, 4 / 2;
    set .@e1, 3 % 2;
    set .@i1, 4 * .@c;
    set .@f1, 2 + 3 * 2;
    set .@g1, 1 + (2 + 3) * (2 + (10 / 2 + 7) * 2 + (2 + 2 * 3));
    set .@h1, 2 - 3 + 2 - 1 + (2 - 3 - 1 + 1);
    vm_dump_locals();"#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(2, events.borrow().get("a").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b").unwrap().value.number_value().clone());
    assert_eq!(8, events.borrow().get("c").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("d").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("e").unwrap().value.number_value().clone());
    assert_eq!(32, events.borrow().get("i").unwrap().value.number_value().clone());
    assert_eq!(8, events.borrow().get("f").unwrap().value.number_value().clone());
    assert_eq!(171, events.borrow().get("g").unwrap().value.number_value().clone());
    assert_eq!(-1, events.borrow().get("h").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("a1").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b1").unwrap().value.number_value().clone());
    assert_eq!(8, events.borrow().get("c1").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("d1").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("e1").unwrap().value.number_value().clone());
    assert_eq!(32, events.borrow().get("i1").unwrap().value.number_value().clone());
    assert_eq!(8, events.borrow().get("f1").unwrap().value.number_value().clone());
    assert_eq!(171, events.borrow().get("g1").unwrap().value.number_value().clone());
    assert_eq!(-1, events.borrow().get("h1").unwrap().value.number_value().clone());
}

#[test]
fn simple_re_assigment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    .@a$ = "hello wrld";
    vm_dump_var("a", .@a$);"#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello wrld"), events.borrow().get("a").unwrap().value.string_value().clone());
}
#[test]
fn plus_equal_string_concat() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello";
    .@a$ += " world";
    vm_dump_var("a", .@a$);"#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}
#[test]
fn plus_equal_num_addition() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a = 1;
    .@a += 2;
    vm_dump_var("a", .@a);"#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(3_i32, events.borrow().get("a").unwrap().value.number_value());
}