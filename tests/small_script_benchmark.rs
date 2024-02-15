use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use criterion::{Criterion, criterion_group, criterion_main};
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::value::Value;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::common::{compile_script, Event, GlobalVariableEntry, VmHook};

mod common;

pub fn criterion_benchmark(c: &mut Criterion) {
    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let vm_hook = VmHook::new_with_behavior_on_missing_native( Box::new(move |e| { events.lock().unwrap().insert(e.name.clone(), e); }), false);
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "BaseClass".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "Eff_Stun".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "RC_Undead".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "RC_Demon".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "bDex".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });

    let classes = compile_script(r#"
        if (!isequipped(4172,4257,4230,4272))
           bonus3 bAutoSpell,"RG_INTIMIDATE",1,20;
        if (BaseClass == 12)
           bonus bFlee,20;
        bonus2 bAddEffWhenHit,Eff_Stun,300+600*(readparam(bDex)>=77);
        bonus3 bAutoSpell,"RG_STRIPWEAPON",1,50;
        bonus2 bHPDrainRate,50,8;
        bonus2 bSPDrainRate,10,4;
        bonus2 bHPLossRate,10,5000;
        bonus bInt,1;
        if (BaseClass == 13) {
           bonus bInt,1;
           bonus bMdef,1;
        }
        bonus bFlee,3;
        bonus bHit,10;
        if (BaseClass == 13) {
           bonus2 bCriticalAddRace,RC_Undead,9;
           bonus2 bCriticalAddRace,RC_Demon,9;
        }
        "#, compiler::DebugFlag::None.value()).unwrap();
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    c.bench_function("small_script_execute_main_script", |b| b.iter(|| {
        Vm::execute_main_script(vm.clone(), Box::new(&vm_hook), vec![]).unwrap();
    }));


    let vm = crate::common::setup_vm(DebugFlag::None.value());
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let vm_hook = VmHook::new_with_behavior_on_missing_native( Box::new(move |e| { events.lock().unwrap().insert(e.name.clone(), e); }), false);
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "BaseClass".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "Eff_Stun".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "RC_Undead".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "RC_Demon".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
    vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "bDex".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });

    let classes = compile_script(r#"
        if (!isequipped(4172,4257,4230,4272))
           bonus3 bAutoSpell,"RG_INTIMIDATE",1,20;
        if (BaseClass == 12)
           bonus bFlee,20;
        bonus2 bAddEffWhenHit,Eff_Stun,300+600*(readparam(bDex)>=77);
        bonus3 bAutoSpell,"RG_STRIPWEAPON",1,50;
        bonus2 bHPDrainRate,50,8;
        bonus2 bSPDrainRate,10,4;
        bonus2 bHPLossRate,10,5000;
        bonus bInt,1;
        if (BaseClass == 13) {
           bonus bInt,1;
           bonus bMdef,1;
        }
        bonus bFlee,3;
        bonus bHit,10;
        if (BaseClass == 13) {
           bonus2 bCriticalAddRace,RC_Undead,9;
           bonus2 bCriticalAddRace,RC_Demon,9;
        }
        "#, compiler::DebugFlag::None.value()).unwrap();
    c.bench_function("repl", |b| b.iter(|| {
        Vm::repl(vm.clone(), &classes[1], Box::new(&vm_hook), vec![]).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);