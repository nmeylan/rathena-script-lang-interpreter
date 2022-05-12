#![feature(try_blocks)]
extern crate core;

use std::collections::HashMap;
use std::sync::Arc;
use antlr_rust::{InputStream};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use ragnarok_script_interpreter::lang::compiler::Compiler;
use ragnarok_script_interpreter::lang::noop_hasher::NoopHasher;
use ragnarok_script_interpreter::lang::value::{Constant, Value};
use ragnarok_script_interpreter::lang::vm::{NativeMethodHandler, Vm};
use ragnarok_script_interpreter::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use ragnarok_script_interpreter::parser::rathenascriptlangparser::RathenaScriptLangParser;
use ragnarok_script_interpreter::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;

#[derive(Default)]
struct Native;

impl NativeMethodHandler for Native {
    fn handle(&self, native: &ragnarok_script_interpreter::lang::value::Native, params: Vec<Value>) {
        if native.name.eq("println") {
            println!("{}", params.iter().map(|p| p.string_value().clone()).collect::<Vec<String>>().join(" "));
        }
    }
}

fn main() {
    let script = ".@a$ = \"hello world\" + \" console \" + 1;\nprint(.@a$);";
    // let script = ".@a$ = \"hello world\";.@a$ = \"console\";\nprint(.@a$);";
    // let script = "print(\"hello world\", \"console\");";
    let charstream = InputStream::new(script);


    let mut function = Compiler::compile("test_script".to_string(), charstream).unwrap();
    // for chunk in chunks {
    //     println!("{:?}", chunk);
    // }
    let mut vm = Vm::new(Box::new(Native::default()));
    let vm_ref = Arc::new(vm);
    Vm::execute_program(vm_ref.clone(), function).unwrap();
}
