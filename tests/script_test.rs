use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn simple_script() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
        vm_dump_var("a", "hello");
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::repl(vm, &classes[1], Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("hello"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}