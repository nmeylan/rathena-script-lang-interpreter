use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event};

mod common;

#[test]
fn pow() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@two = 2;
    vm_dump_var("nine", pow(3, .@two));
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::All.value(), move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::All.value(), move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    let runtime_error = Vm::execute_main_script(vm).err().unwrap();
    // Then
    assert_eq!(r#"Pow first argument should be a number. Value is string not a number.
test_script 4:24.
l4	    vm_dump_var("nine", pow("3", .@two));
	                        ^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:4)"#, runtime_error.to_string().trim());
}