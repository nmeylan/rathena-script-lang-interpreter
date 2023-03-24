
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};

use crate::common::{compile_script, Event, VmHook};

mod common;


#[test]
fn simple_array_assignment() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@a$[1] = "world";
    .@b$ = .@a$[0] + " " + .@a$[1];
    .@c$ = add_s(.@a$[0]);

    function add_s {
        .@local$ = getarg(0) + "s";
        return .@local$;
    }

    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("b", .@b$);
    vm_dump_var("c", .@c$);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello"), events.lock().unwrap().get("a0").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("world"), events.lock().unwrap().get("a1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hellos"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn getarraysize_should_array_size() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@a$[1] = "world";
    .@b$[3] = "world";
    .@a_len = getarraysize(.@a$);
    .@b_len = getarraysize(.@b$);
    vm_dump_var("a_len", .@a_len);
    vm_dump_var("b_len", .@b_len);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(2, events.lock().unwrap().get("a_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!(4, events.lock().unwrap().get("b_len").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn cleararray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@b[0] = 1;
    .@b[1] = 1;
    cleararray(.@a$[0], "world", 10);
    cleararray(.@b[1], 2, 10);
    vm_dump_var("a_len", getarraysize(.@a$));
    vm_dump_var("b_len", getarraysize(.@b));
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("b0", .@b[0]);
    vm_dump_var("b1", .@b[1]);
    vm_dump_var("a9", .@a$[9]);
    vm_dump_var("b10", .@b[10]);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(10, events.lock().unwrap().get("a_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!(11, events.lock().unwrap().get("b_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!("world", events.lock().unwrap().get("a0").unwrap().value.string_value().unwrap().clone());
    assert_eq!("world", events.lock().unwrap().get("a9").unwrap().value.string_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("b0").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("b1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("b10").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn setarray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@c$ = "toto";
    setarray .@a$[0], "hello", "world", .@c$;
    setarray(.@b[1], 1, 2, 3);
    setarray .@x[1], 1;
    vm_dump_var("a_len", getarraysize(.@a$));
    vm_dump_var("b_len", getarraysize(.@b));
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("b1", .@b[1]);
    vm_dump_var("b2", .@b[2]);
    vm_dump_var("x1", .@x[1]);
    .@two = 2;
    vm_dump_var("b3", .@b[.@two + 1]);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(3, events.lock().unwrap().get("a_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!(4, events.lock().unwrap().get("b_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!("hello", events.lock().unwrap().get("a0").unwrap().value.string_value().unwrap().clone());
    assert_eq!("world", events.lock().unwrap().get("a1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("b1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("b2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("b3").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("x1").unwrap().value.number_value().unwrap().clone());
}
#[test]
fn getelementofarray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    function plus_one_to_array_elem_0 {
        return getelementofarray(getarg(0), 0) + 1;
    }
    setarray .@b[0], 1, 2, 3;
    .@c = plus_one_to_array_elem_0(.@b);
    .@d = getelementofarray(.@b, 2);
    vm_dump_var("two", .@c);
    vm_dump_var("three", .@d);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(2, events.lock().unwrap().get("two").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("three").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn deletearray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    setarray(.@b[1], 1, 2, 3, 4,5);
    deletearray(.@b[0], 1);
    vm_dump_var("b_0_after_first_delete", .@b[0]);
    vm_dump_var("b_2_after_first_delete", .@b[2]);
    vm_dump_var("b_len_after_first_delete", getarraysize(.@b));

    deletearray .@b[3], 100;
    vm_dump_var("b_0_after_second_delete", .@b[0]);
    vm_dump_var("b_2_after_second_delete", .@b[2]);
    vm_dump_var("b_len_after_second_delete", getarraysize(.@b));
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("b_0_after_first_delete").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("b_2_after_first_delete").unwrap().value.number_value().unwrap().clone());
    assert_eq!(5, events.lock().unwrap().get("b_len_after_first_delete").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("b_0_after_second_delete").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("b_2_after_second_delete").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("b_len_after_second_delete").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn inarray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@c$ = "toto";
    setarray .@a$[0], "hello", "world", .@c$;
    setarray .@b[1], 1, 2, 3, 4,5;
    .@toto_index = inarray(.@a$[0], "toto");
    .@four_index = inarray .@b, 4;
    .@not_found_index = inarray .@b, 100;
    vm_dump_var("toto_index", .@toto_index);
    vm_dump_var("four_index", .@four_index);
    vm_dump_var("not_found_index", .@not_found_index);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(2, events.lock().unwrap().get("toto_index").unwrap().value.number_value().unwrap().clone());
    assert_eq!(4, events.lock().unwrap().get("four_index").unwrap().value.number_value().unwrap().clone());
    assert_eq!(-1, events.lock().unwrap().get("not_found_index").unwrap().value.number_value().unwrap().clone());
}


#[test]
fn copyarray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@toto$ = "toto";
    setarray .@a$[0], "hello", "world", .@toto$;
    copyarray .@b$[0], .@a$[0], 1;
    copyarray .@c$[1], .@a$[1], getarraysize(.@a$) - 1;

    vm_dump_var("a_len", getarraysize(.@a$));
    vm_dump_var("b_len", getarraysize(.@b$));
    vm_dump_var("c_len", getarraysize(.@c$));
    vm_dump_var("b0", .@b$[0]);
    vm_dump_var("c1", .@c$[1]);
    vm_dump_var("c2", .@c$[2]);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(3, events.lock().unwrap().get("a_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("b_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("c_len").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hello"), events.lock().unwrap().get("b0").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("world"), events.lock().unwrap().get("c1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("toto"), events.lock().unwrap().get("c2").unwrap().value.string_value().unwrap().clone());
}


#[test]
fn setarray_with_getelementofarray() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@toto$ = "toto";
    function my_fn {
        .@arr_size = getarraysize(getarg(0));
        println(getelementofarray(getarg(0), getarg(1)));
        setarray getelementofarray(getarg(0), getarg(1)),getarg(2);
    }
    setarray .@a$[0], "hello", "world", .@toto$;

    my_fn(.@a$, 1, "hello");
    setarray getelementofarray(.@a$, 2),"titi";

    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("a2", .@a$[2]);
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::All.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello"), events.lock().unwrap().get("a0").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hello"), events.lock().unwrap().get("a1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("titi"), events.lock().unwrap().get("a2").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn setarray_wrong_type_error() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@toto = 1;
    setarray .@a$[0], "hello", "world", .@toto;
    setarray .@a$[0], "hello", "world", "2";
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook)).err().unwrap();
    // Then
    assert_eq!(r#"setarray - tried to assign Number (4th arguments) to an array of String
test_script 4:4.
l4	    setarray .@a$[0], "hello", "world", .@toto;
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:4)"#, runtime_error.to_string().trim());
}

#[test]
fn cleararray_wrong_type_error() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    cleararray(.@a$[0], 0, 10);
"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook)).err().unwrap();
    // Then
    assert_eq!(r#"cleararray - tried to assign Number (second argument) to an array of String
test_script 4:4.
l4	    cleararray(.@a$[0], 0, 10);
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:4)"#, runtime_error.to_string().trim());
}


#[test]
fn copyarray_wrong_type_error() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@toto$ = "toto";
    setarray .@a$[0], "hello", "world", .@toto$;
    copyarray .@b[0], .@a[0], 1;
"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook)).err().unwrap();
    // Then
    assert_eq!(r#"copyarray - tried to assign an array of String (second argument) to an array of Number
test_script 5:4.
l5	    copyarray .@b[0], .@a[0], 1;
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:5)"#, runtime_error.to_string().trim());
}

#[test]
fn copyarray_outofbounds_error() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@toto$ = "toto";
    setarray .@a$[0], "hello", "world", .@toto$;
    copyarray .@b$[0], .@a$[18], 1;
"#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events;
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), script, Box::new(&vm_hook));
    let runtime_error = Vm::execute_main_script(vm, Box::new(&vm_hook)).err().unwrap();
    // Then
    assert_eq!(r#"Array index out of bounds: index 18, length 3
test_script 5:4.
l5	    copyarray .@b$[0], .@a$[18], 1;
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:5)"#, runtime_error.to_string().trim());
}