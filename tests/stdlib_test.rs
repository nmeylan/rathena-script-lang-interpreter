
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn pow() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@two = 2;
    vm_dump_var("nine", pow(3, .@two));
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(9, events.lock().unwrap().get("nine").unwrap().value.number_value().unwrap());
}
#[test]
fn pow_with_wrong_type() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@two = 2;
    vm_dump_var("nine", pow("3", .@two));
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).err().unwrap();
    // Then
    assert_eq!(r#"Pow first argument should be a number. Value is string not a number.
_MainScript 4:24.
l4	    vm_dump_var("nine", pow("3", .@two));
	                        ^^^^^^^^^^^^^^^

0: _main
	at _MainScript(_MainScript:4)"#, runtime_error.to_string().trim());
}

#[test]
fn rand() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    vm_dump_var("randomv", rand(1, 9));
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(true, events.lock().unwrap().get("randomv").unwrap().value.number_value().is_ok());
}