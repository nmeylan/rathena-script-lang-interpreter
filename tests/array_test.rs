use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::{compile_script, Event};

mod common;


#[test]
fn simple_array_assignment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello"), events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!(String::from("world"), events.borrow().get("a1").unwrap().value.string_value().clone());
    assert_eq!(String::from("hello world"), events.borrow().get("b").unwrap().value.string_value().clone());
    assert_eq!(String::from("hellos"), events.borrow().get("c").unwrap().value.string_value().clone());
}

#[test]
fn getarraysize_should_array_size() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@a$[0] = "hello";
    .@a$[1] = "world";
    .@b$[3] = "world";
    .@a_len = getarraysize(.@a$);
    .@b_len = getarraysize(.@b$);
    vm_dump_var("a_len", .@a_len);
    vm_dump_var("b_len", .@b_len);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(2, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(4, events.borrow().get("b_len").unwrap().value.number_value().clone());
}

#[test]
fn cleararray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(10, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(11, events.borrow().get("b_len").unwrap().value.number_value().clone());
    assert_eq!("world", events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!("world", events.borrow().get("a9").unwrap().value.string_value().clone());
    assert_eq!(1, events.borrow().get("b0").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("b1").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("b10").unwrap().value.number_value().clone());
}

#[test]
fn setarray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@c$ = "toto";
    setarray .@a$[0], "hello", "world", .@c$;
    setarray(.@b[1], 1, 2, 3);
    vm_dump_var("a_len", getarraysize(.@a$));
    vm_dump_var("b_len", getarraysize(.@b));
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    vm_dump_var("b1", .@b[1]);
    vm_dump_var("b2", .@b[2]);
    vm_dump_var("b3", .@b[3]);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(3, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(4, events.borrow().get("b_len").unwrap().value.number_value().clone());
    assert_eq!("hello", events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!("world", events.borrow().get("a1").unwrap().value.string_value().clone());
    assert_eq!(1, events.borrow().get("b1").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("b2").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b3").unwrap().value.number_value().clone());
}

#[test]
fn getelementofarray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    function plus_one_to_array_elem_0 {
        return getelementofarray(getarg(0), 0) + 1;
    }
    setarray .@b[0], 1, 2, 3;
    .@c = plus_one_to_array_elem_0(.@b);
    .@d = getelementofarray(.@b, 2);
    vm_dump_var("two", .@c);
    vm_dump_var("three", .@d);
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(2, events.borrow().get("two").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("three").unwrap().value.number_value().clone());
}

#[test]
fn deletearray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(1, events.borrow().get("b_0_after_first_delete").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b_2_after_first_delete").unwrap().value.number_value().clone());
    assert_eq!(5, events.borrow().get("b_len_after_first_delete").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("b_0_after_second_delete").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b_2_after_second_delete").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("b_len_after_second_delete").unwrap().value.number_value().clone());
}

#[test]
fn inarray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(2, events.borrow().get("toto_index").unwrap().value.number_value().clone());
    assert_eq!(4, events.borrow().get("four_index").unwrap().value.number_value().clone());
    assert_eq!(-1, events.borrow().get("not_found_index").unwrap().value.number_value().clone());
}


#[test]
fn copyarray() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
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
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(3, events.borrow().get("a_len").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("b_len").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("c_len").unwrap().value.number_value().clone());
    assert_eq!(String::from("hello"), events.borrow().get("b0").unwrap().value.string_value().clone());
    assert_eq!(String::from("world"), events.borrow().get("c1").unwrap().value.string_value().clone());
    assert_eq!(String::from("toto"), events.borrow().get("c2").unwrap().value.string_value().clone());
}

#[test]
fn setarray_errors() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let script = compile_script(r#"
    .@toto = 1;
    setarray .@a$[0], "hello", "world", .@toto;
    setarray .@a$[0], "hello", "world", "2";
    "#).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), script);
    let runtime_error = Vm::execute_main_script(vm).err().unwrap();
    // Then
    assert_eq!(r#"setarray - tried to assign Number to an array of String
test_script 4:4.
l4	    setarray .@a$[0], "hello", "world", .@toto;
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

0: _main
	at test_script(_MainScript:4)"#, runtime_error.to_string().trim());
}