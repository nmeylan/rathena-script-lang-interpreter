#![feature(try_blocks)]
extern crate core;


use std::sync::Arc;
use antlr_rust::{InputStream};


use ragnarok_script_interpreter::lang::compiler::Compiler;

use ragnarok_script_interpreter::lang::value::{Value};
use ragnarok_script_interpreter::lang::vm::{NativeMethodHandler, Vm};




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


    let function = Compiler::compile("test_script".to_string(), charstream).unwrap();
    // for chunk in chunks {
    //     println!("{:?}", chunk);
    // }
    let vm = Vm::new(Box::new(Native::default()));
    let vm_ref = Arc::new(vm);
    Vm::execute_program(vm_ref, function).unwrap();
}
