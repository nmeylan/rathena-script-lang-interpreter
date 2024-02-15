#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::compiler::{Compiler};
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use rathena_script_lang_interpreter::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use rathena_script_lang_interpreter::parser::rathenascriptlangparser::RathenaScriptLangParser;
use rathena_script_lang_interpreter::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;
use crate::common::{compile_script, Event, VmHook};


mod common;
#[test]
fn parser_compiler_performance() {
    // Given
    let file = File::open(Path::new("tests/fixtures/warper.txt")).unwrap();
    let mut reader = BufReader::new(file);
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content).unwrap();
    // When
    let start_process = Instant::now();
    let mut start = Instant::now();
    let file_content_clone = file_content.clone();
    let lexer = RathenaScriptLangLexer::new(InputStream::new(file_content_clone.as_str()));
    println!("Lexer tooks {}ms", start.elapsed().as_millis() as f32 );
    start = Instant::now();
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = RathenaScriptLangParser::new(token_stream);
    println!("Parser tooks {}ms", start.elapsed().as_millis() as f32 );
    start = Instant::now();
    let tree = parser.compilationUnit();
    println!("Tree building tooks {}ms", start.elapsed().as_millis() as f32 );
    start = Instant::now();
    let mut compiler = Compiler::new("warper.txt".to_string(), file_content, "native_functions_list.txt", compiler::DebugFlag::None.value());
    compiler.visit_compilationUnit(tree.as_ref().unwrap());
    println!("Compilation took {}ms", start.elapsed().as_millis() as f32 );
    // Then
    assert_eq!(true, 2 > start_process.elapsed().as_secs());
}


#[cfg(test)]
mod benchmark {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use rathena_script_lang_interpreter::lang::compiler;
    use rathena_script_lang_interpreter::lang::value::Value;
    use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
    use crate::common::{compile_script, Event, GlobalVariableEntry, VmHook};

    #[bench]
    fn profile_item_script(bencher: &mut test::Bencher) {
        let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
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
        let events_clone = events.clone();
        let vm = crate::common::setup_vm(DebugFlag::None.value());
        // When
        let vm_hook = VmHook::new_with_behavior_on_missing_native( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }), false);
        vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "BaseClass".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
        vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "Eff_Stun".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
        vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "RC_Undead".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
        vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "RC_Demon".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
        vm_hook.global_variable_store.lock().unwrap().push(GlobalVariableEntry { name: "bDex".to_string(), value: Value::Number(Some(13)), scope: "char_permanent".to_string(), index: None });
        bencher.iter(|| {
            // Then
            Vm::repl(vm.clone(), &classes[1], Box::new(&vm_hook), vec![]).unwrap();
        });
    }
}