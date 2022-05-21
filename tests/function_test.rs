mod common;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

pub fn compile(script: &str) -> Function {
    Compiler::compile("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
}


#[test]
fn simple_function_call() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    let function = compile(r#"
    my_func();
    function my_func {
        .@a$ = "hello world";
        vm_dump_var("a", .@a$);
    }
    "#);
    // When
    Vm::execute_program(vm, function).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}
