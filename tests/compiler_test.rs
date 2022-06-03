use ragnarok_script_interpreter::lang::chunk::ClassFile;
use ragnarok_script_interpreter::lang::compiler::{CompilationError, Compiler};

mod common;

pub fn compile(script: &str) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
    Compiler::compile_script("test_script".to_string(), script)
}

#[test]
fn undefined_variable() {
    // Given
    let script = r#"
print(.@a$);
print(.@b$);
.@b$ = "hello world";
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("test_script 3:6. Variable \".@a$\" is undefined.\nl3\tprint(.@a$);\n\t      ^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("test_script 4:6. Variable \".@b$\" is undefined.\nl4\tprint(.@b$);\n\t      ^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
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
    assert_eq!("test_script 2:0. Variable \".@a$\" is a String but was assigned to a Number.\nl2\t.@a$ = 1;\n\t^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("test_script 3:7. Subtraction operator \"-\" is not allowed for String\nl3\t.@a$ = \"1\" - \"2\";\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
    assert_eq!("test_script 4:7. Subtraction operator \"-\" is not allowed for String\nl4\t.@a$ = 1 - \"2\";\n\t       ^^^^^\n", result.as_ref().err().unwrap().get(2).unwrap().message());
    assert_eq!("test_script 5:7. Subtraction operator \"-\" is not allowed for String\nl5\t.@a$ = \"1\" - 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(3).unwrap().message());
    assert_eq!("test_script 6:7. Multiply operator \"*\" is not allowed for String\nl6\t.@a$ = \"1\" * 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(4).unwrap().message());
    assert_eq!("test_script 7:7. Modulo operator \"%\" is not allowed for String\nl7\t.@a$ = \"1\" % 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(5).unwrap().message());
    assert_eq!("test_script 8:7. Divide operator \"/\" is not allowed for String\nl8\t.@a$ = \"1\" / 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(6).unwrap().message());
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
    assert_eq!("test_script 2:0. Variable \".@a\" is a Number but was assigned to a String.\nl2\t.@a = \"1\";\n\t^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("test_script 3:0. Variable \".@b\" is a Number but was assigned to a String.\nl3\t.@b = 1 + \"2\";\n\t^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
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
    assert_eq!(r#"test_script 6:4. A function with name "my_func" already exists.
l6	    function my_func {
	    ^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!(r#"test_script 9:4. A native function with name "print" already exists.
l9	    function print { // this is a native function, it is forbidden to declare a func with the same name as native function.
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
    assert_eq!(r#"test_script 5:8. Variable ".@a" is a Number but was assigned to a String.
l5	        .@a = getarg(1, "3") + 4;
	        ^^^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
}

#[test]
fn type_checking_conditional_statement() {
    // Given
    let script = r#"
    .@a = 1 == 1;
    .@a = 1 == "1";
    .@a = "1" == "1";
    .@a = "1" == "1" && 1 == "1";
    .@a = "1" > 2;
    .@a = "1" >= 2;
    .@a = 1 > 2;
    .@a = "1" < 2;
    .@a = "1" <= 2;
    .@a = 1 < 2;
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"test_script 4:10. Can't perform comparison when left and right are not same types
l4	    .@a = 1 == "1";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"test_script 6:24. Can't perform comparison when left and right are not same types
l6	    .@a = "1" == "1" && 1 == "1";
	                        ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
    assert_eq!(r#"test_script 7:10. Can't perform comparison when left and right are not same types
l7	    .@a = "1" > 2;
	          ^^^^^^^
"#, result.as_ref().err().unwrap()[3].message()); //TODO fix this, at index 2 there is an error which should not be there
}

#[test]
fn type_checking_logical_and_or() {
    // Given
    let script = r#"
    .@a = 1 && 1;
    .@a = 1 && "1";
    .@a = 1 || 0;
    .@a = 1 && "0";
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"test_script 4:10. Can't perform logical and (&&) when left and right are not same types
l4	    .@a = 1 && "1";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"test_script 6:10. Can't perform logical and (&&) when left and right are not same types
l6	    .@a = 1 && "0";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
}

#[test]
fn undefined_label() {
    // Given
    let script = r#"

    "#;
    // When
    let _result = compile(script);
    // Then

}


#[test]
fn label_defined_in_a_function_is_not_valid() {
    // Given
    let script = r#"
        function my_func {
            Assign:
                .@a = 0;
        }
    "#;
    // When
    let result = compile(script);
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"test_script 4:12. Label "Assign" is declared in "my_func" function scope but label should be declared in script scope only.
l4	            Assign:
	            ^^^^^^^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
}