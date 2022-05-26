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
.@a$ = "1" - "2";
.@a$ = 1 - "2";
.@a$ = "1" - 2;
.@a$ = "1" * 2;
.@a$ = "1" % 2;
.@a$ = "1" / 2;
"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(7, result.as_ref().err().unwrap().len());
    assert_eq!("test_script 1:0. Variable \".@a$\" is a String but was assigned to a Number.\nl1\t.@a$ = 1;\n\t^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("test_script 2:7. Subtraction operator \"-\" is not allowed for String\nl2\t.@a$ = \"1\" - \"2\";\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
    assert_eq!("test_script 3:7. Subtraction operator \"-\" is not allowed for String\nl3\t.@a$ = 1 - \"2\";\n\t       ^^^^^\n", result.as_ref().err().unwrap().get(2).unwrap().message());
    assert_eq!("test_script 4:7. Subtraction operator \"-\" is not allowed for String\nl4\t.@a$ = \"1\" - 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(3).unwrap().message());
    assert_eq!("test_script 5:7. Multiply operator \"*\" is not allowed for String\nl5\t.@a$ = \"1\" * 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(4).unwrap().message());
    assert_eq!("test_script 6:7. Modulo operator \"%\" is not allowed for String\nl6\t.@a$ = \"1\" % 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(5).unwrap().message());
    assert_eq!("test_script 7:7. Divide operator \"/\" is not allowed for String\nl7\t.@a$ = \"1\" / 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(6).unwrap().message());
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
fn function_definition() {
    // Given
    let script = r#"function my_func() {}"#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_ok());
}

#[test]
fn function_redefinition_should_be_an_error() {
    // Given
    let script = r#"
    function my_func {
        .@a = 1;
    }
    function my_func {
        .@a = 2;
    }
    function print { // this is a native function, it is forbidden to declare a func with the same name as native function.
        .@a = 2;
    }
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(2, result.as_ref().err().unwrap().len());
    result.as_ref().err().unwrap().iter().for_each(|e| println!("{}", e));
    assert_eq!(r#"test_script 5:4. A function with name "my_func" already exists.
l5	    function my_func {
	    ^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!(r#"test_script 8:4. A native function with name "print" already exists.
l8	    function print { // this is a native function, it is forbidden to declare a func with the same name as native function.
	    ^
"#, result.as_ref().err().unwrap().get(1).unwrap().message());
}

#[test]
fn function_call_with_number_arguments_with_default_different_type_assigned_to_number() {
    // Given
    let script = r#"
    my_func(2);
    function my_func {
        .@a = getarg(1, "3") + 4;
        vm_dump_var("a", .@a);
    }
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"test_script 4:8. Variable ".@a" is a Number but was assigned to a String.
l4	        .@a = getarg(1, "3") + 4;
	        ^^^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
}

#[test]
fn type_checking_conditional_statement() {
    // Given
    let script = r#"
    .@a = 1 == 1;
    .@b = 1 == "1";
    .@c = "1" == "1";
    .@d = "1" == "1" && 1 == "1";
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(3, result.as_ref().err().unwrap().len());
    assert_eq!(r#"test_script 3:10. Can't perform comparison when left and right are not same types
l3	    .@b = 1 == "1";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"test_script 5:24. Can't perform comparison when left and right are not same types
l5	    .@d = "1" == "1" && 1 == "1";
	                        ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
}

#[test]
fn type_checking_logical_and_or() {
    // Given
    let script = r#"
    .@a = 1 && 1;
    .@b = 1 && "1";
    .@c = 1 || 0;
    .@d = 1 && "0";
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(2, result.as_ref().err().unwrap().len());
    assert_eq!(r#"test_script 3:10. Can't perform logical and (&&) when left and right are not same types
l3	    .@b = 1 && "1";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"test_script 5:10. Can't perform logical and (&&) when left and right are not same types
l5	    .@d = 1 && "0";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
}