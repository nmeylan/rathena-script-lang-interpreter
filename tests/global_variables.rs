
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn char_variable_set_get() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    a$ = "hello world";
    b = 1;
    vm_dump_var("a", a$);
    vm_dump_var("b", b);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::All.value());
    // When
    let vm_hook = VmHook::new(Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn char_variable_plus_equal() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    a$ = "hello";
    a$ += " world";
    vm_dump_var("a", a$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new(Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}