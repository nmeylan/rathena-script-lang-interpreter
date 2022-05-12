
use antlr_rust::InputStream;
use ragnarok_script_interpreter::lang::compiler::{CompilationError, Compiler};
use ragnarok_script_interpreter::lang::value::Function;

mod common;

pub fn compile(script: &str) -> Result<Function, Vec<CompilationError>> {
    let char_stream = InputStream::new(script);
    Compiler::compile("test_script".to_string(), char_stream)
}

#[test]
fn type_checking_string_valid() {
    // Given
    // When

    // Then
    assert_eq!(1, 1);
}