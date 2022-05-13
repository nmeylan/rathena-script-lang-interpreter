use antlr_rust::InputStream;
use ragnarok_script_interpreter::lang::compiler::{CompilationError, Compiler};
use ragnarok_script_interpreter::lang::value::Function;

mod common;

pub fn compile(script: &str) -> Result<Function, Vec<CompilationError>> {
    let char_stream = InputStream::new(script);
    Compiler::compile("test_script".to_string(), char_stream)
}

#[test]
fn undefined_variable() {
    // Given
    let script = r#"print(.@a$);"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 1:6. Variable \".@a$\" is undefined.", result.err().unwrap().get(0).unwrap().message());
}

#[test]
fn undefined_function() {
    // Given
    let script = r#"undefined();"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
}

#[test]
fn type_checking_string_valid() {
    // Given
    let script = r#".@a$ = "toto";"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_ok());
}

#[test]
fn type_checking_string_invalid() {
    // Given
    let script = r#".@a$ = 1;"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 1:0. Variable \".@a$\" is a String but was assigned to a Number.", result.err().unwrap().get(0).unwrap().message());
}

#[test]
fn type_checking_number_valid() {
    // Given
    let script = r#".@a = 1;"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_ok());
}

#[test]
fn type_checking_number_invalid() {
    // Given
    let script = r#".@a = "1";"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 1:0. Variable \".@a\" is a Number but was assigned to a String.", result.err().unwrap().get(0).unwrap().message());
}