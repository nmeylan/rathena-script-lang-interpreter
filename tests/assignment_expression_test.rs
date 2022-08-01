mod common;


use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use common::Event;
use rathena_script_lang_interpreter::lang::compiler;


use crate::common::{compile_script, VmHook};


#[test]
fn simple_assigment() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$ = "hello world";
    vm_dump_var("a", .@a$);"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn double_assigment() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$ = .@b$ = "hello world";
    vm_dump_var("a", .@a$);
    vm_dump_var("b", .@b$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn hex_number() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a = 0xE;
    vm_dump_var("a", .@a);
    vm_dump_var("b", .@a + 1);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::All.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(14, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(15, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn assigment_to_local_variable() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$ = "hello world";
    .@b$ = .@a$;
    vm_dump_var("a", .@a$);
    vm_dump_var("b", .@b$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn assignment_with_string_concat() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$ = "hello" + " world " + 1;
    vm_dump_var("a", .@a$);"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world 1"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn assignment_with_complex_string_concat() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#" setarray .@Styles[1], 1;
    set .@Revert, 1;
    set .@Style, 1;
    setarray .@Styles[1], 10,20,30,4,5;
    set .@s,1;
    set .@menu$, " ~ Next (^0055FF"+((.@Style!=.@Styles[.@s])?.@Style+1:1)+"^000000): Jump to...: ~ Revert to original (^0055FF"+.@Revert+"^000000)";

    vm_dump_var("menu", .@menu$);"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::Execution.value() | DebugFlag::Stack.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from(" ~ Next (^0055FF2^000000): Jump to...: ~ Revert to original (^0055FF1^000000)"), events.lock().unwrap().get("menu").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn assignment_with_number_operation() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a = 1 + 1;
    .@b = 4 - 1;
    .@c = 4 * 2;
    .@d = 4 / 2;
    .@e = 3 % 2;
    .@i = 4 * .@c;
    .@f = 2 + 3 * 2;
    .@g = 1 + (2 + 3) * (2 + (10 / 2 + 7) * 2 + (2 + 2 * 3));
    .@h = 2 - 3 + 2 - 1 + (2 - 3 - 1 + 1);
    set .@a1, 1 + 1;
    set .@b1, 4 - 1;
    set .@c1, 4 * 2;
    set .@d1, 4 / 2;
    set .@e1, 3 % 2;
    set(.@i1, 4 * .@c + 2);
    set .@f1, 2 + 3 * 2;
    set .@g1, 1 + (2 + 3) * (2 + (10 / 2 + 7) * 2 + (2 + 2 * 3));
    set .@h1, 2 - 3 + 2 - 1 + (2 - 3 - 1 + 1);
    vm_dump_locals();"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    events.lock().unwrap().iter().for_each(|(k, v)| println!("{} = {}", k, v.value));
    assert_eq!(2, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("b").unwrap().value.number_value().unwrap().clone());
    assert_eq!(8, events.lock().unwrap().get("c").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("d").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("e").unwrap().value.number_value().unwrap().clone());
    assert_eq!(32, events.lock().unwrap().get("i").unwrap().value.number_value().unwrap().clone());
    assert_eq!(8, events.lock().unwrap().get("f").unwrap().value.number_value().unwrap().clone());
    assert_eq!(171, events.lock().unwrap().get("g").unwrap().value.number_value().unwrap().clone());
    assert_eq!(-1, events.lock().unwrap().get("h").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("a1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("b1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(8, events.lock().unwrap().get("c1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("d1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("e1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(34, events.lock().unwrap().get("i1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(8, events.lock().unwrap().get("f1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(171, events.lock().unwrap().get("g1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(-1, events.lock().unwrap().get("h1").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn simple_re_assigment() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$ = "hello world";
    .@a$ = "hello wrld";
    vm_dump_var("a", .@a$);"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello wrld"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn plus_equal_string_concat() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$ = "hello";
    .@a$ += " world";
    vm_dump_var("a", .@a$);"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}
#[test]
fn plus_equal_num_addition() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a = 1;
    .@a += 2;
    vm_dump_var("a", .@a);"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(3_i32, events.lock().unwrap().get("a").unwrap().value.number_value().unwrap());
}

#[test]
fn setd_function() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@var_name$ = "var";
    .@full_variable_name$ = ".@my_" + .@var_name$ + "$";
    setd ".@my_" + .@var_name$ + "$", "hello_world";
    setd(".@my_" + .@var_name$ + 2 + "$", "hello_world2");
    setd(".@my_array$[" + 1 + "]", "hello_world array");
    for(.@i=0;.@i<10;.@i+=1) {
        setd ".@v" + .@i, .@i;
    }
    set .@set_with_getd_value$, getd(".@my_" + .@var_name$ + "$");
    vm_dump_locals();
    vm_dump_var("arraysize", getarraysize(.@my_array$));
    vm_dump_var("a", .@my_array$[1]);
    vm_dump_var("my_var_via_getd", getd(".@my_" + .@var_name$ + "$"));
    vm_dump_var("my_var_via_getd_with_variable", getd(.@full_variable_name$));
    vm_dump_var("array_via_getd", getd(".@my_array$[" + 1 + "]"));
    vm_dump_var("set_with_getd_value$", .@set_with_getd_value$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello_world"), events.lock().unwrap().get("my_var").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello_world"), events.lock().unwrap().get("my_var_via_getd").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello_world"), events.lock().unwrap().get("my_var_via_getd_with_variable").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello_world"), events.lock().unwrap().get("set_with_getd_value").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello_world2"), events.lock().unwrap().get("my_var2").unwrap().value.string_value().unwrap().clone());
    assert_eq!(0, events.lock().unwrap().get("v0").unwrap().value.number_value().unwrap().clone());
    assert_eq!(9, events.lock().unwrap().get("v9").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hello_world array"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello_world array"), events.lock().unwrap().get("array_via_getd").unwrap().value.string_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("arraysize").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn set_with_getd_function() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@one = 1;
    set getd(".@"+"set_with_getd"), 2;
    setd(".@"+"setd", 2);
    vm_dump_var("set_with_getd", .@set_with_getd);
    vm_dump_var("set_with_getd_1", getd(".@set_with_getd"));
    vm_dump_var("setd", .@setd);
    vm_dump_var("setd_1", getd(".@setd"));
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(2, events.lock().unwrap().get("set_with_getd").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("set_with_getd_1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("setd").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("setd_1").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn setd_function_error_wrong_type() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@var_name$ = "var";
    setd ".@my_" + .@var_name$, "hello_world";
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook)).err().unwrap();
    // Then
    assert_eq!(r#"tried to assign a String to a variable declared as Number
test_script 4:4.
l4	    setd ".@my_" + .@var_name$, "hello_world";
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:4)"#, runtime_error.to_string().trim());
}

#[test]
fn setd_function_error_undefined_variable() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@var_name$ = "var";
    setd ".@my_" + .@var_name$, 1;
    println(.@a);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook)).err().unwrap();
    // Then
    assert_eq!(r#"Variable is not declared in local scope
test_script 5:12.
l5	    println(.@a);
	            ^^^"#, runtime_error.to_string().trim());
}

#[test]
fn input_assign_variable() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    input .@a$;
    vm_dump_var("a", .@a$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("Hello world from input"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}