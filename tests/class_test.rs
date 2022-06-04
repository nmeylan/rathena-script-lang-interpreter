use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ragnarok_script_interpreter::lang::chunk::ClassFile;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::Event;

mod common;

pub fn compile(script: &str) -> Vec<ClassFile> {
    Compiler::compile("test_script".to_string(), script).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e))
    }).unwrap()
}

# [test]
fn simple_class_test() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        .@a$ = global_func_hello();
        my_func(.@a$);
        function my_func {
            .@a$ = getarg(0) + " world";
            vm_dump_var("a", .@a$);
        }
    }

    function global_func_hello {
        return "hello";
    }
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}

#[test]
fn instance_variable_test() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        'counter = 0;
        function inc1 {
            'counter = 'counter + 1;
        }
        function inc2 {
            'counter = 'counter + 2;
        }
        inc1();
        inc2();
        vm_dump_var("counter", 'counter);
    }
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(3, events.borrow().get("counter").unwrap().value.number_value().clone());
}

// TODO
fn on_init_hook_test() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        function inc {
            .counter = .counter + 1;
            vm_dump_var("counter", .counter);
        }
        OnInit:
            .counter = 0;
    }
    "#);
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(move |e| { events_clone.borrow_mut().insert(e.name.clone(), e); });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.borrow().get("a").unwrap().value.string_value().clone());
}