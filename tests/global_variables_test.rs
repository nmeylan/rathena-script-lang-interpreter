
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::value::Value;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, GlobalVariableEntry, VmHook};

mod common;

#[test]
fn char_variable_set_get() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    a$ = "hello world";
    set c, 2;
    setd "char_variable" + "$", "using setd";
    set getd("char_variable_set_getd" + "$"), "using set + getd";
    b = 1;
    vm_dump_var("a", a$);
    vm_dump_var("b", b);
    vm_dump_var("c", c);
    vm_dump_var("d", char_variable$);
    vm_dump_var("e", char_variable_set_getd$);
    vm_dump_var("f", getd("char_variable" + "$"));
    vm_dump_var("g", existing_variable); // Variable already exist in DB
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let vm_hook = VmHook::new(Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry {
        name: "existing_variable".to_string(),
        value: Value::Number(Some(11)),
        scope: "char_permanent".to_string(),
        index: None
    });
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello world"), vm_hook.find_global_by_name_and_scope(&String::from("a$"), &String::from("char_permanent")).unwrap().value.string_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("using setd"), events.lock().unwrap().get("d").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("using set + getd"), events.lock().unwrap().get("e").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("using setd"), events.lock().unwrap().get("f").unwrap().value.string_value().unwrap().clone());
    assert_eq!(11, events.lock().unwrap().get("g").unwrap().value.number_value().unwrap().clone());
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
    vm_hook.global_variable_store.lock().unwrap().iter().for_each(|entry| println!("{:?}", entry));
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn char_variable_array() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    setarray a$[0], "hello", "world";
    set b[0], 1;
    b[1] = 2;
    copyarray c$[2], a$[1], 1;
    vm_dump_var("array_a_size", getarraysize(a$));
    vm_dump_var("array_c_size", getarraysize(c$));
    vm_dump_var("a0", a$[0]);
    vm_dump_var("a1", a$[1]);
    vm_dump_var("c0", c$[2]);
    vm_dump_var("existing_array_3", existing_array[3]);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let vm_hook = VmHook::new(Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry {
        name: "existing_array".to_string(),
        value: Value::Number(Some(11)),
        scope: "char_permanent".to_string(),
        index: Some(3)
    });vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry {
        name: "existing_array".to_string(),
        value: Value::Number(Some(12)),
        scope: "char_permanent".to_string(),
        index: Some(4)
    });
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    vm_hook.global_variable_store.lock().unwrap().iter().for_each(|entry| println!("{:?}", entry));
    assert_eq!(2, events.lock().unwrap().get("array_a_size").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("array_c_size").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hello"), events.lock().unwrap().get("a0").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("world"), events.lock().unwrap().get("a1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("world"), events.lock().unwrap().get("c0").unwrap().value.string_value().unwrap().clone());
    assert_eq!(11, events.lock().unwrap().get("existing_array_3").unwrap().value.number_value().unwrap().clone());
}