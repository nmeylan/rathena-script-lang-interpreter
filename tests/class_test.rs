use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use ragnarok_script_interpreter::lang::chunk::ClassFile;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::vm::Vm;
use crate::common::{compile, Event};

mod common;

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
    "#).unwrap();
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
    "#).unwrap();
    let events_clone = events.clone();
    let i = Arc::new(RefCell::new(0));
    let vm = crate::common::setup_vm(move |e| {
        let i1 = *i.borrow();
        *i.borrow_mut() = i1 + 1;
        events_clone.borrow_mut().insert(format!("{}{}", e.name.clone(), *i.borrow()), e);
    });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm.clone(), "Myclass".to_string()).unwrap();
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(3, events.borrow().get("counter1").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("counter2").unwrap().value.number_value().clone());
}

#[test]
fn static_variable_test() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        .counter = 0;
        function inc1 {
            .counter = .counter + 1;
        }
        function inc2 {
            .counter = .counter + 2;
        }
        inc1();
        inc2();
        vm_dump_var("counter", .counter);
    }
    "#).unwrap();
    let events_clone = events.clone();
    let i = Arc::new(RefCell::new(0));
    let vm = crate::common::setup_vm(move |e| {
        let i1 = *i.borrow();
        *i.borrow_mut() = i1 + 1;
        events_clone.borrow_mut().insert(format!("{}{}", e.name.clone(), *i.borrow()), e);
    });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm.clone(), "Myclass".to_string()).unwrap();
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(3, events.borrow().get("counter1").unwrap().value.number_value().clone());
    assert_eq!(3, events.borrow().get("counter2").unwrap().value.number_value().clone());
}

#[test]
fn on_init_hook_test() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        inc();
        function inc {
            .counter = .counter + 1;
            .hello$ = .hello$ + .hello$;
            vm_dump_var("counter", .counter);
            vm_dump_var("hello_str", .hello$);
        }
        end;
        OnInit:
            .@zero = 0;
            .@hello$ = "hello";
            .counter = .@zero;
            .hello$ = .@hello$;
    }
    "#).unwrap();
    let events_clone = events.clone();
    let i = Arc::new(RefCell::new(0));
    let vm = crate::common::setup_vm(move |e| {
        let i1 = *i.borrow();
        *i.borrow_mut() = i1 + 1;
        events_clone.borrow_mut().insert(format!("{}{}", e.name.clone(), *i.borrow()), e);
    });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm.clone(), "Myclass".to_string()).unwrap();
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(1, events.borrow().get("counter1").unwrap().value.number_value().clone());
    assert_eq!(2, events.borrow().get("counter3").unwrap().value.number_value().clone());
    assert_eq!(String::from("hellohello"), events.borrow().get("hello_str2").unwrap().value.string_value().clone());
    assert_eq!(String::from("hellohellohellohello"), events.borrow().get("hello_str4").unwrap().value.string_value().clone());
}
#[test]
fn on_instance_init_hook_test() {
    // Given
    let events = Rc::new(RefCell::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        inc();
        end;
        function inc {
            'counter = 'counter + 1;
            'hello$ = 'hello$ + 'hello$;
            vm_dump_var("counter", 'counter);
            vm_dump_var("hello_str", 'hello$);
        }
        OnInstanceInit:
            .@zero = 0;
            .@hello$ = "hello";
            'counter = .@zero;
            'hello$ = .@hello$;
    }
    "#).unwrap();
    let events_clone = events.clone();
    let i = Arc::new(RefCell::new(0));
    let vm = crate::common::setup_vm(move |e| {
        let i1 = *i.borrow();
        *i.borrow_mut() = i1 + 1;
        events_clone.borrow_mut().insert(format!("{}{}", e.name.clone(), *i.borrow()), e);
    });
    // When
    Vm::bootstrap(vm.clone(), classes);
    Vm::execute_class(vm.clone(), "Myclass".to_string()).unwrap();
    Vm::execute_class(vm, "Myclass".to_string()).unwrap();
    // Then
    assert_eq!(1, events.borrow().get("counter1").unwrap().value.number_value().clone());
    assert_eq!(1, events.borrow().get("counter3").unwrap().value.number_value().clone());
    assert_eq!(String::from("hellohello"), events.borrow().get("hello_str2").unwrap().value.string_value().clone());
    assert_eq!(String::from("hellohello"), events.borrow().get("hello_str4").unwrap().value.string_value().clone());
}