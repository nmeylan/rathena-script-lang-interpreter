mod common;


use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::compiler::Compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

#[test]
fn simple_function_call() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func();
    function my_func {
        .@a$ = "hello world";
        vm_dump_var("a", .@a$);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn underscore_function_call() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func();
    function my_func {
        .@a$ = "hello world";
        vm_dump_var("a", _(.@a$));
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn function_call_with_arguments() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func("hello");
    function my_func {
        .@a$ = getarg(0) + " world";
        vm_dump_var("a", .@a$);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn function_call_with_negative_number_arguments() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func(-10);
    function my_func {
        .@a = getarg(0);
        vm_dump_var("a", .@a);
        .@b = -10;
        vm_dump_var("b", .@b);
        .@c = .@a-10;
        vm_dump_var("c", .@c);
        .@d = .@a - -10;
        vm_dump_var("d", .@d);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(-10, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(-10, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(-20, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn function_call_with_variable_arguments() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    .@a$ = "hello";
    my_func(.@a$);
    function my_func {
        .@a$ = getarg(0) + " world";
        vm_dump_var("a", .@a$);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn function_call_with_two_arguments() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    .@a$ = "hello";
    my_func(.@a$, "world");
    function my_func {
        .@a$ = getarg(0) + " " + getarg(1);
        vm_dump_var("a", .@a$);
        vm_dump_var("b", getargcount());
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn function_call_with_arguments_out_of_bounds() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func("hello");
    function my_func {
        .@a$ = getarg(1) + " world";
        vm_dump_var("a", .@a$);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).err().unwrap();
    // Then
    assert_eq!(r#"Can't call getarg(1) which is greater than number of arguments provided: 1. Maximum allow index is 0. Consider calling getarg with a default value: getarg(1, DEFAULT_VALUE)
_MainScript 5:15.
l5	        .@a$ = getarg(1) + " world";
	               ^^^^^^^^^

0: _main
	at _MainScript(_MainScript:3)
1: my_func
	at _MainScript(_MainScript:5)"#, runtime_error.to_string().trim());
}
#[test]
fn function_call_with_arguments_with_default() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func("hello");
    function my_func {
        .@a$ = getarg(1, "default") + " world";
        vm_dump_var("a", .@a$);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("default world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn function_call_with_number_arguments() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func(2);
    function my_func {
        .@a = getarg(0) + 4;
        vm_dump_var("a", .@a);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(6_i32, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn function_call_with_number_arguments_with_default() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func(2);
    function my_func {
        .@a = getarg(1, 3) + 4;
        vm_dump_var("a", .@a);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(7_i32, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn function_call_with_number_arguments_with_default_different_type_assigned_to_string() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    my_func(2);
    function my_func {
        .@a$ = getarg(1, "3") + 4;
        vm_dump_var("a", .@a$);
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("34"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn function_with_return_type() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    function plus_four {
        .@a = getarg(0) + 4;
        .@b = 0;
        return .@a + .@b;
    }
    function one {
        return 1;
    }
    .@a = plus_four(2);
    .@b = callfunc "plus_four", 4;
    .@c = callfunc("plus_four", 6);
    .@d = callfunc("one", "one");
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(6_i32, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(8_i32, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(10_i32, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1_i32, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn function_with_return_type_multicall() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    function plus_four {
        .@a = getarg(0) + 4;
        return .@a;
    }
    function minus_one {
        return getarg(0) - 1;
    }
    function print_arg {
        println(getarg(0));
    }
    .@a = minus_one(plus_four(2));
    print_arg(.@a);
    vm_dump_var("a", .@a);
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(5_i32, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn recursive_function_call_with_return() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    .@a = my_func(10);
    function my_func {
        .@my_local = getarg(0) - 1;
        if (.@my_local > 0) {
            return my_func(.@my_local);
        }
        return .@my_local;
    }
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(0, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}


#[test]
fn nested_function_call_with_return() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    .@a = add(add(10));
    function add {
        return getarg(0) + 1;
    }
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(12, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn nested_function_call_with_expression_in_args() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    .@a = two_args(1, add((add(16)&255)|32));
    function add {
        return getarg(0) + 1;
    }
    function two_args {
        return getarg(0) + getarg(1);
    }
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(51, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn nested_function_call_with_expression_in_args_2() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let classes = compile_script(r#"
    function a{return getarg(0) + 1;}
    .@a = a(a(a(a(a(a(a(a(a(a(a(1)))))))))));
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(12, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
}