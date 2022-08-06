
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

#[test]
fn simple_condition() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    if(1) {
        .@a$ = "i am true";
    } else {
        .@d$ = "i am false";
    }
    .@c$ = "i am not part of condition";
    if(0) {
        .@b$ = "i am false";
    } else {
        .@e$ = "i am true";
    }
    if(0) {
        .@f$ = "i am false";
    } else if (0) {
        .@g$ = "i am false";
    } else {
        .@h$ = "i am true";
    }
    if(true)
        .@i$ = "i am true";

    if(0)
        .@j$ = "i am false";

    if(0) {
        .@k$ = "i am false";
    } else if (1) {
        .@l$ = "i am true";
    } else {
        .@m$ = "i am false";
    }
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("i am true"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(false, events.lock().unwrap().get("b").is_some());
    assert_eq!(String::from("i am not part of condition"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!(false, events.lock().unwrap().get("d").is_some());
    assert_eq!(String::from("i am true"), events.lock().unwrap().get("e").unwrap().value.string_value().unwrap().clone());
    assert_eq!(false, events.lock().unwrap().get("f").is_some());
    assert_eq!(false, events.lock().unwrap().get("g").is_some());
    assert_eq!(String::from("i am true"), events.lock().unwrap().get("h").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("i am true"), events.lock().unwrap().get("i").unwrap().value.string_value().unwrap().clone());
    assert_eq!(false, events.lock().unwrap().get("k").is_some());
    assert_eq!(true, events.lock().unwrap().get("l").is_some());
    assert_eq!(false, events.lock().unwrap().get("m").is_some());
}

#[test]
fn condition_with_expressions() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    if((1 == 1 || 1) && 1) {
        .@a$ = "i am true";
    }
    .@c$ = "i am not part of condition";
    if((1 == 1 || 1) && 0) {
        .@b$ = "i am false";
    }
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("i am true"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(false, events.lock().unwrap().get("b").is_some());
    assert_eq!(String::from("i am not part of condition"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn ternary_condition() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a$ = 1 == 2 ? "hello" : "world";
    .@b$ = 1 == 1 ? "hello" : "world";
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn conditional_statements() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile_script(r#"
    .@a = 1 == 1;
    .@b = 1 == 2;
    .@c = "1" == "1";
    .@d = "1" == "2";
    .@e = 0 == 1 && 1 == 1;
    .@f = (0 == 1 && 1 == 1) || 1 == 1;
    .@g = 0 == 1 && (1 == 1 || 1 == 1);
    .@h = 1000;
    .@i = .@h > 999 && .@h < 10000;
    .@j = .@h > 1000 && .@h < 10000;
    .@k = .@h >= 1000 && .@h < 10000;
    .@l = .@h > 999 && .@h <= 1000;
    .@m = .@h > 999 && .@h < 1000;
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("e").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("f").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("g").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("i").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("j").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("k").unwrap().value.number_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("m").unwrap().value.number_value().unwrap().clone());
}


#[test]
fn switch_statement() {
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    function num_to_str {
        .@a = getarg(0);
        switch (.@a) {
            case 1:
                return "one";
            case 2:
                .@res$ = "two";
                break;
            case 3:
                .@res$ = "three";
            case 4:
                .@res$ = "four";
                break;
            default:
                .@res$ = "greater than 4";
                break;
        }
        return .@res$;
    }
    .@a$ = num_to_str(1);
    .@b$ = num_to_str(2);
    .@c$ = num_to_str(3);
    .@d$ = num_to_str(4);
    .@e$ = num_to_str(5);
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!("one", events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!("two", events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!("four", events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone()); // no break in case 3:
    assert_eq!("four", events.lock().unwrap().get("d").unwrap().value.string_value().unwrap().clone());
    assert_eq!("greater than 4", events.lock().unwrap().get("e").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn nested_switch_statement() {
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    function num_to_str {
        .@a = getarg(0);
        .@b$ = getarg(1, "default");
        switch (.@a) {
            case 1:
                return "one";
            case 2:

                switch(.@b$) {
                    case "toto":
                        return "return";
                    case "tutu":
                        .@i = 0;
                        for (;;) {
                            .@i += 1;
                            if (.@i > 2) break;
                        }
                        .@res$ = "break";
                        break;
                }
                break;
            case 3:
                .@res$ = "three";
            case 4:
                .@res$ = "four";
                break;
            default:
                .@res$ = "greater than 4";
                break;
        }
        return .@res$;
    }
    .@a$ = num_to_str(1);
    .@b1$ = num_to_str(2, "toto");
    .@b2$ = num_to_str(2, "tutu");
    .@c$ = num_to_str(3);
    .@d$ = num_to_str(4);
    .@e$ = num_to_str(5);
    vm_dump_locals();
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!("one", events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!("return", events.lock().unwrap().get("b1").unwrap().value.string_value().unwrap().clone());
    assert_eq!("break", events.lock().unwrap().get("b2").unwrap().value.string_value().unwrap().clone());
    assert_eq!("four", events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!("four", events.lock().unwrap().get("d").unwrap().value.string_value().unwrap().clone());
    assert_eq!("greater than 4", events.lock().unwrap().get("e").unwrap().value.string_value().unwrap().clone());
}