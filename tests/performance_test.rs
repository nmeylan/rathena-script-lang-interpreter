use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::Instant;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::compiler::{Compiler, DebugFlag};
use rathena_script_lang_interpreter::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use rathena_script_lang_interpreter::parser::rathenascriptlangparser::RathenaScriptLangParser;
use rathena_script_lang_interpreter::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;
use crate::common::{compile, compile_script};

mod common;
#[test]
fn parser_compiler_performance() {
    // Given
    let file = File::open(Path::new("tests/fixtures/warper.txt")).unwrap();
    let mut reader = BufReader::new(file);
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content).unwrap();
    // When
    let mut start_process = Instant::now();
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
