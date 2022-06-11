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
    .@c$ = add_s(.@a$[0]);

    function add_s {
        .@local$ = getarg(0) + "s";
        return .@local$;
    }

    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("b", .@b$);
    vm_dump_var("c", .@c$);
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
    assert_eq!(String::from("hellos"), events.borrow().get("c").unwrap().value.string_value().clone());
}

#[test]
fn getarraysize_should_array_size() {
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

#[test]
fn cleararray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@b[0] = 1;
    .@b[1] = 1;
    cleararray(.@a$[0], "world", 10);
    cleararray(.@b[1], 2, 10);
    vm_dump_var("a_len", getarraysize(.@a$));
    vm_dump_var("b_len", getarraysize(.@b));
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("b0", .@b[0]);
    vm_dump_var("b1", .@b[1]);
    vm_dump_var("a9", .@a$[9]);
    vm_dump_var("b10", .@b[10]);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(10, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(11, events.borrow().get("b_len").unwrap().value.number_value().clone());
    assert_eq!("world", events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!("world", events.borrow().get("a9").unwrap().value.string_value().clone());
    assert_eq!(1, events.borrow().get("b0").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("b1").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("b10").unwrap().value.number_value().clone());
}
#[test]
fn setarray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    setarray .@a$[0], "hello", "world";
    setarray .@b[1], 1, 2, 3;
    vm_dump_var("a_len", getarraysize(.@a$));
    vm_dump_var("b_len", getarraysize(.@b));
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("b1", .@b[1]);
    vm_dump_var("b2", .@b[2]);
    vm_dump_var("b3", .@b[3]);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(2, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(4, events.borrow().get("b_len").unwrap().value.number_value().clone());
    assert_eq!("hello", events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!("world", events.borrow().get("a1").unwrap().value.string_value().clone());
    assert_eq!(1, events.borrow().get("b1").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("b2").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b3").unwrap().value.number_value().clone());
}