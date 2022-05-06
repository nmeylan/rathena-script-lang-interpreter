#![feature(try_blocks)]
#![feature(in_band_lifetimes)]

mod parser;
mod lang;

use antlr_rust::{InputStream};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use crate::lang::compiler::Compiler;
use crate::lang::disassemble::Disassemble;
use crate::lang::vm::Vm;
use crate::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use crate::parser::rathenascriptlangparser::RathenaScriptLangParser;
use crate::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;


fn main() {
    // let script = "a = \"hello world\";\nprint(a);";
    let script = "print(\"hello world\", \"console\");";
    let charstream = InputStream::new(script);


    let mut compiler = Compiler::default();
    let mut chunks = compiler.compile(charstream);
    // for chunk in chunks {
    //     println!("{:?}", chunk);
    // }
    Disassemble::print(&mut chunks);
    let mut vm = Vm::new();
    vm.test(&mut chunks).unwrap();
}
