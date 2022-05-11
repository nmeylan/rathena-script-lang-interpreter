use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use antlr_rust::InputStream;
use ragnarok_script_interpreter::lang::value::Function;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

mod common;

pub fn compile(script: &str) -> Function {
    let char_stream = InputStream::new(script);
    Compiler::compile("test_script".to_string(), char_stream)
}

#[test]
fn simple_assigment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let function = compile(r#"
    .@a$ = "hello world" + " console " + 1;
    vm_dump_var("a", .@a$);"#);
    let events_clone = events.clone();
    let vm = common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::execute_program(vm, function);
    // Then
    assert_eq!(String::from("hello world console 1"), events.borrow().get("a").unwrap().value.string_value().clone());
}