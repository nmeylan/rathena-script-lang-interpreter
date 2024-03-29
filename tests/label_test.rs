
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn simple_label() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    ItDoesNothing:
        .@b$ = "variable in label 1";
    ItDoesNothing1:
        .@c$ = "variable in label 2";
        vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 1"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 2"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
}


#[test]
fn simple_label_with_goto() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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

    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(true, events.lock().unwrap().get("b").is_none());
    assert_eq!(String::from("variable in label 2"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 3"), events.lock().unwrap().get("d").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn label_with_goto_inside() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 2"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!(true, events.lock().unwrap().get("d").is_none());
}


#[test]
fn label_with_goto_in_a_function() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 2"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!(true, events.lock().unwrap().get("d").is_none());
    assert_eq!(String::from("the end"), events.lock().unwrap().get("endd").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn label_with_goto_in_a_nested_function() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 2"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!(true, events.lock().unwrap().get("d").is_none());
    assert_eq!(String::from("the end"), events.lock().unwrap().get("endd").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn callsub_with_return() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    callsub L_Style, 1, 2;
    vm_dump_var("a", 0);
    callsub L_Style, 2, 2;
    vm_dump_var("b", 0);

    L_Style:
        .@a = getarg(0);
        .@b = getarg(1);
         .@c = 1;
        for (.@i = 0; .@i < 2; .@i +=1) {
            .@c += .@i;
        }
        vm_dump_var("c", .@c);
        if (.@a < .@b) {
            return;
        } else
            end;
        end;
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(0, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(true, events.lock().unwrap().get("b").is_none());
    assert_eq!(2, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
}