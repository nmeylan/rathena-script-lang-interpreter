use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event};

mod common;

#[test]
fn multithread_support_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = "hello world";
    vm_dump_var("a", .@a$);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value(), move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    thread::spawn(move || {
        Vm::execute_main_script(vm).unwrap();
    }).join().unwrap();

    // Then
    assert_eq!(true, events.lock().unwrap().get("a").is_some());
}