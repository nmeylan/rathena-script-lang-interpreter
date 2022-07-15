use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::compiler;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile, Event, VmHook};

mod common;

#[test]
fn simple_class_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook { hook: Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }) };
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm, class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(String::from("hello world"), events.lock().unwrap().get("a").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn instance_variable_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let i = Arc::new(Mutex::new(0));
    let vm_hook = VmHook { hook: Box::new(move |e| {
        let i1 = *i.lock().unwrap();
        *i.lock().unwrap() = i1 + 1;
        events_clone.lock().unwrap().insert(format!("{}{}", e.name, *i.lock().unwrap()), e);
    }) };
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(3, events.lock().unwrap().get("counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("counter2").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn static_variable_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let i = Arc::new(Mutex::new(0));
    let vm_hook = VmHook { hook: Box::new(move |e| {
        let i1 = *i.lock().unwrap();
        *i.lock().unwrap() = i1 + 1;
        events_clone.lock().unwrap().insert(format!("{}{}", e.name, *i.lock().unwrap()), e);
    }) };
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(3, events.lock().unwrap().get("counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("counter2").unwrap().value.number_value().unwrap().clone());
}

#[test]
fn on_init_hook_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        inc();
        function inc {
            .counter = .counter + 1;
            .array[0] += 1;
            .hello$ = .hello$ + .hello$;
            vm_dump_var("counter", .counter);
            vm_dump_var("hello_str", .hello$);
            vm_dump_var("array_index0_", .array[0]);
        }
        end;
        OnInit:
            .@zero = 0;
            setarray(.array[0], 1, 2, 3, 4);
            .@hello$ = "hello";
            .counter = .@zero;
            .hello$ = .@hello$;
            end;
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let i: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let vm_hook = VmHook { hook: Box::new(move |e| {
        let i1 = i.lock().unwrap().get(&e.name).unwrap_or(&(0 as usize)).clone();
        i.lock().unwrap().insert(e.name.clone(), i1 + 1);
        events_clone.lock().unwrap().insert(format!("{}{}", e.name, i1 + 1), e);
    }) };
    let vm = crate::common::setup_vm(DebugFlag::All.value());
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("counter2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("array_index0_1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(3, events.lock().unwrap().get("array_index0_2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hellohello"), events.lock().unwrap().get("hello_str1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hellohellohellohello"), events.lock().unwrap().get("hello_str2").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn on_instance_init_hook_test() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        inc();
        end;
        function inc {
            'counter = 'counter + 1;
            'hello$ = 'hello$ + 'hello$;
            'array_counter = 'array[0] + 1;
            vm_dump_var("counter", 'counter);
            vm_dump_var("hello_str", 'hello$);
            vm_dump_var("array_counter", 'array_counter);
            vm_dump_var("array_index3_", 'array[3]);
        }
        OnInstanceInit:
            .@zero = 0;
            setarray('array[0], 1, 2, 3, 4);
            .@hello$ = "hello";
            'counter = .@zero;
            'hello$ = .@hello$;
            end;
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let i: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let vm_hook = VmHook { hook: Box::new(move |e| {
        let i1 = i.lock().unwrap().get(&e.name).unwrap_or(&(0 as usize)).clone();
        i.lock().unwrap().insert(e.name.clone(), i1 + 1);
        events_clone.lock().unwrap().insert(format!("{}{}", e.name, i1 + 1), e);
    }) };
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("counter2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("array_counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("array_counter2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(4, events.lock().unwrap().get("array_index3_1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(4, events.lock().unwrap().get("array_index3_2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hellohello"), events.lock().unwrap().get("hello_str1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("hellohello"), events.lock().unwrap().get("hello_str2").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn setd_instance_variable() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        inc();
        end;
        function inc {
            setd "'counter", 'counter + 1;
            vm_dump_var("counter", 'counter);
            vm_dump_var("counter_getd", getd("'cou"+"nter"));
            vm_dump_var("my_array", getd("'my_array[" + 1 + "]$"));
        }
        OnInstanceInit:
            setd "'counter", 0;
            setd("'my_array[" + 1 + "]$", "hello_world array");
            end;
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let i: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let vm_hook = VmHook { hook: Box::new(move |e| {
        let i1 = i.lock().unwrap().get(&e.name).unwrap_or(&(0 as usize)).clone();
        i.lock().unwrap().insert(e.name.clone(), i1 + 1);
        events_clone.lock().unwrap().insert(format!("{}{}", e.name, i1 + 1), e);
    }) };
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("counter2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("counter_getd1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("counter_getd2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hello_world array"), events.lock().unwrap().get("my_array1").unwrap().value.string_value().unwrap().clone());
}

#[test]
fn setd_static_variable() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My class -1, {
        inc();
        end;
        function inc {
            setd ".counter", .counter + 1;
            vm_dump_var("counter", .counter);
            vm_dump_var("counter_getd", getd(".co"+"unter"));
            vm_dump_var("my_array", getd(".my_array[" + 1 + "]$"));
        }
        OnInit:
            setd ".counter", 0;
            setd(".my_array[" + 1 + "]$", "hello_world array");
            end;
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let i: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let vm_hook = VmHook { hook: Box::new(move |e| {
        let i1 = i.lock().unwrap().get(&e.name).unwrap_or(&(0 as usize)).clone();
        i.lock().unwrap().insert(e.name.clone(), i1 + 1);
        events_clone.lock().unwrap().insert(format!("{}{}", e.name, i1 + 1), e);
    }) };
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "Myclass".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("counter1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("counter2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(1, events.lock().unwrap().get("counter_getd1").unwrap().value.number_value().unwrap().clone());
    assert_eq!(2, events.lock().unwrap().get("counter_getd2").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hello_world array"), events.lock().unwrap().get("my_array1").unwrap().value.string_value().unwrap().clone());
}


#[test]
fn getvariableofnpc_when_npc_exist() {
    // Given
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let classes = compile(r#"
    - script My NPC1 -1, {
        OnInit:
            .value = 1;
            setarray .array$[0], "hello", "world";
            end;
    }

    - script My NPC2 -1, {
        vm_dump_var("npc_1_value", getvariableofnpc(.value, "MyNPC1");
        // vm_dump_var("npc_1_value_getd", getvariableofnpc(getd(".value"), "MyNPC1");
        vm_dump_var("array1", getvariableofnpc(.array$[0], "MyNPC1");
        vm_dump_var("array2", getvariableofnpc(.array$[1], "MyNPC1");
    }
    "#, compiler::DebugFlag::None.value()).unwrap();
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    // When
    let vm_hook = VmHook { hook: Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }) };
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    let (class_reference, instance_reference) = Vm::create_instance(vm.clone(), "MyNPC2".to_string(), Box::new(&vm_hook)).unwrap();
    Vm::run_main_function(vm.clone(), class_reference, instance_reference, Box::new(&vm_hook)).unwrap();
    // Then
    assert_eq!(1, events.lock().unwrap().get("npc_1_value").unwrap().value.number_value().unwrap().clone());
    // assert_eq!(1, events.lock().unwrap().get("npc_1_value_getd").unwrap().value.number_value().unwrap().clone());
    assert_eq!(String::from("hello"), events.lock().unwrap().get("array1").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("world"), events.lock().unwrap().get("array2").unwrap().value.string_value().unwrap().clone());
}