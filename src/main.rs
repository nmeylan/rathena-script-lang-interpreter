#![feature(try_blocks)]
#![feature(in_band_lifetimes)]

extern crate core;

mod parser;
mod lang;

use std::collections::HashMap;
use antlr_rust::{InputStream};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use crate::lang::compiler::Compiler;
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Constant, Identifier, Value};
use crate::lang::vm::{NativeMethodHandler, Vm};
use crate::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use crate::parser::rathenascriptlangparser::RathenaScriptLangParser;
use crate::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;

#[derive(Default)]
struct Native;

impl NativeMethodHandler for Native {
    fn handle(&self, native: &lang::value::Native, params: Vec<Value>, identifiers_pool: &HashMap<u64, Identifier, NoopHasher>, constant_pool: &HashMap<u64, Constant, NoopHasher>) {
        if native.name.eq("println") {
            println!("{}", params.iter().fold(String::new(), |acc, arg| acc + format!("{}", arg).as_str()));
        }
    }
}

fn main() {
    // let script = ".@a = \"hello world\";\nprint(.@a);";
    let script = "print(\"hello world\", \"console\");";
    let charstream = InputStream::new(script);


    let mut function = Compiler::compile("test_script".to_string(), charstream);
    // for chunk in chunks {
    //     println!("{:?}", chunk);
    // }
    let mut vm = Vm::new(Box::new(Native::default()));
    vm.run_main(function).unwrap();
}
