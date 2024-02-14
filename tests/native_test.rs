
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32};
use std::sync::atomic::Ordering::Relaxed;
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::value::Value;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn menu_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    menu "A", A, "B", B, "C", -, "D", D, "E:F", E;
    vm_dump_var("selected_menu", "C");
    end;
    A:
        vm_dump_var("selected_menu", "A");
        end;
    B:
        vm_dump_var("selected_menu", "B");
        end;
    D:
        vm_dump_var("selected_menu", "D");
        end;
    E:
        vm_dump_var("selected_menu", "E");
        end;
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let i = AtomicI32::new(1);
    // each time we will call the script, "menu" will select next option, start with 1 then 2, etc...
    let vm_hook = VmHook::new_with_custom_handler(
        Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }),
        Box::new(move |native_name, thread| {
            if native_name.eq("menu") {
                let i1 = i.load(Relaxed);
                thread.push_constant_on_stack(Value::new_number(i1));
                i.store(i1 + 1, Relaxed);
                return false;
            }
            true
        }),
    );
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    // Then
    Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    assert_eq!(String::from("A"), events.lock().unwrap().get("selected_menu").unwrap().value.string_value().unwrap().clone());
    Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    assert_eq!(String::from("B"), events.lock().unwrap().get("selected_menu").unwrap().value.string_value().unwrap().clone());
    Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    assert_eq!(String::from("C"), events.lock().unwrap().get("selected_menu").unwrap().value.string_value().unwrap().clone());
    Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    assert_eq!(String::from("D"), events.lock().unwrap().get("selected_menu").unwrap().value.string_value().unwrap().clone());
    Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    assert_eq!(String::from("E"), events.lock().unwrap().get("selected_menu").unwrap().value.string_value().unwrap().clone());
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    assert_eq!(String::from("E"), events.lock().unwrap().get("selected_menu").unwrap().value.string_value().unwrap().clone());
}


#[test]
fn implode_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    setarray .@a$[1], "Hello", "World", "this", "is implode function";
    vm_dump_var("a", implode(.@a$, " "));
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("Hello World this is implode function"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}