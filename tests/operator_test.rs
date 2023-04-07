mod common;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};


#[test]
fn bit_and() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a = 1 & 255;
    .@b = 16 & 128;
    .@c = 16 & 255;
    .@d = 16 & 255 & 128;
    vm_dump_var("a", .@a);
    vm_dump_var("b", .@b);
    vm_dump_var("c", .@c);
    vm_dump_var("d", .@d);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(16, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn bit_or() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a = 1 | 255;
    .@b = 16 | 128;
    .@c = 16 | 255;
    .@d = 16 | 32 | 128 ;
    vm_dump_var("a", .@a);
    vm_dump_var("b", .@b);
    vm_dump_var("c", .@c);
    vm_dump_var("d", .@d);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(255, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(144, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(255, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(176, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn bitwise_not() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a = ~2;
    .@b = ~16;
    .@c = ~769;
    vm_dump_var("a", .@a & 1);
    vm_dump_var("b", .@b & 16);
    vm_dump_var("c", .@b & 32);
    vm_dump_var("d", .@c & 32);
    vm_dump_var("e", .@c & 512);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(32, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(32, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("e").unwrap().value.number_value().unwrap().clone());
}