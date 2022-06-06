use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::chunk::ClassFile;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

mod common;

pub fn compile(script: &str) -> Vec<ClassFile> {
    Compiler::compile_script("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
}


#[test]
fn simple_array_assignment() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    .@a$[0] = "hello";
    .@a$[1] = "world";
    vm_dump_var("a0", .@a$[0]);
    vm_dump_var("a1", .@a$[1]);
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_main_script(vm).unwrap();
    // Then
    assert_eq!(String::from("hello"), events.borrow().get("a0").unwrap().value.string_value().clone());
    assert_eq!(String::from("world"), events.borrow().get("a1").unwrap().value.string_value().clone());
}