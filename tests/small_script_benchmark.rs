use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use criterion::{Criterion, criterion_group, criterion_main};
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, VmHook};

mod common;

pub fn criterion_benchmark(c: &mut Criterion) {
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let vm_hook = VmHook::new( Box::new(move |e| { events.lock().unwrap().insert(e.name.clone(), e); }));
    c.bench_function("small_script_execute_main_script", |b| b.iter(|| {
        let classes = compile_script(r#"
        vm_dump_var("a", "hello");
        "#, compiler::DebugFlag::None.value()).unwrap();
        Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
        Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    }));
    c.bench_function("repl", |b| b.iter(|| {
        let classes = compile_script(r#"
        vm_dump_var("a", "hello");
        "#, compiler::DebugFlag::None.value()).unwrap();
        Vm::repl(vm.clone(), &classes[1], Box::new(&vm_hook), vec![]).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);