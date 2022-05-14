use ragnarok_script_interpreter::lang::compiler::{CompilationError, Compiler};
use ragnarok_script_interpreter::lang::value::Function;

mod common;

pub fn compile(script: &str) -> Result<Function, Vec<CompilationError>> {
    Compiler::compile("test_script".to_string(), script)
}

#[test]
fn undefined_variable() {
    // Given
    let script = r#"print(.@a$);"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 1:6. Variable \".@a$\" is undefined.\nl1\tprint(.@a$);\n\t      ^^^^\n", result.err().unwrap().get(0).unwrap().message());
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
    let script = r#".@a$ = "toto";
.@b$ = 1 + "toto";
.@c$ = "toto" + 1;
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_ok());
}

#[test]
fn type_checking_string_invalid() {
    // Given
    let script = r#".@a$ = 1;
.@b$ = "1" - "2";
"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 1:0. Variable \".@a$\" is a String but was assigned to a Number.\nl1\t.@a$ = 1;\n\t^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("test_script 1:0. Variable \".@a$\" is a String but was assigned to a Number.\nl1\t.@a$ = 1;\n\t^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
}

#[test]
fn type_checking_number_valid() {
    // Given
    let script = r#"
    .@a = 1;
    .@a = 1 + 3;
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_ok());
}

#[test]
fn type_checking_number_invalid() {
    // Given
    let script = r#".@a = "1";
.@b = 1 + "2";
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 1:0. Variable \".@a\" is a Number but was assigned to a String.\nl1\t.@a = \"1\";\n\t^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("test_script 2:0. Variable \".@b\" is a Number but was assigned to a String.\nl2\t.@b = 1 + \"2\";\n\t^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
}