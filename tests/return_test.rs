use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

mod common;

pub fn compile(script: &str) -> Function {
    Compiler::compile("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
}


#[test]
fn return_stop_script() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = "hello world";
    return;
    vm_dump_var("a", .@a$);
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(true, events.borrow().get("a").is_none());
}