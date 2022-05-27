mod common;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

pub fn compile(script: &str) -> Function {
    Compiler::compile("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
}


#[test]
fn simple_function_call() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func();
    function my_func {
        .@a$ = "hello world";
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn function_call_with_arguments() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func("hello");
    function my_func {
        .@a$ = getarg(0) + " world";
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn function_call_with_variable_arguments() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    .@a$ = "hello";
    my_func(.@a$);
    function my_func {
        .@a$ = getarg(0) + " world";
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn function_call_with_arguments_out_of_bounds() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func("hello");
    function my_func {
        .@a$ = getarg(1) + " world";
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    let runtime_error = Vm::execute_program(vm, function).err().unwrap();
    // Then
    assert_eq!(String::from("Can't call getarg(1) which is greater than number of arguments provided: 1. Maximum allow index is 0. Consider calling getarg with a default value: getarg(1, DEFAULT_VALUE)"), runtime_error.to_string());
}
#[test]
fn function_call_with_arguments_with_default() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func("hello");
    function my_func {
        .@a$ = getarg(1, "default") + " world";
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(String::from("default world"), events.borrow().get("a").unwrap().value.string_value().clone());
}
#[test]
fn function_call_with_number_arguments() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func(2);
    function my_func {
        .@a = getarg(0) + 4;
        vm_dump_var("a", .@a);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(6_i32, events.borrow().get("a").unwrap().value.number_value().clone());
}
#[test]
fn function_call_with_number_arguments_with_default() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func(2);
    function my_func {
        .@a = getarg(1, 3) + 4;
        vm_dump_var("a", .@a);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(7_i32, events.borrow().get("a").unwrap().value.number_value().clone());
}
#[test]
fn function_call_with_number_arguments_with_default_different_type_assigned_to_string() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func(2);
    function my_func {
        .@a$ = getarg(1, "3") + 4;
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(String::from("34"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn function_with_return_type() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    function plus_four {
        .@a = getarg(0) + 4;
        .@b = 0;
        return .@a + .@b;
    }
    .@a = plus_four(2);
    .@b = callfunc "plus_four", 4;
    .@c = callfunc("plus_four", 6);
    vm_dump_locals();
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(6_i32, events.borrow().get("a").unwrap().value.number_value().clone());
    assert_eq!(8_i32, events.borrow().get("b").unwrap().value.number_value().clone());
    assert_eq!(10_i32, events.borrow().get("c").unwrap().value.number_value().clone());
}

#[test]
fn function_with_return_type_multicall() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    function plus_four {
        .@a = getarg(0) + 4;
        return .@a;
    }
    function minus_one {
        return getarg(0) - 1;
    }
    function print_arg {
        print(getarg(0));
    }
    .@a = minus_one(plus_four(2));
    print_arg(.@a);
    vm_dump_var("a", .@a);
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(5_i32, events.borrow().get("a").unwrap().value.number_value().clone());
}

#[test]
fn recursive_function_call_with_return() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    .@a = my_func(10);
    function my_func {
        .@my_local = getarg(0) - 1;
        if (.@my_local > 0) {
            return my_func(.@my_local);
        }
        return .@my_local;
    }
    vm_dump_locals();
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(0, events.borrow().get("a").unwrap().value.number_value().clone());
}