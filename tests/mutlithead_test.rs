use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn multithread_support_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    vm_dump_var("a", .@a$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook { hook: Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }) };
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    thread::spawn(move || {
        Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    }).join().unwrap();

    // Then
    assert_eq!(true, events.lock().unwrap().get("a").is_some());
}