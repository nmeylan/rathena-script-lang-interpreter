use rathena_script_lang_interpreter::lang::compiler;
use crate::common::{compile, compile_script};

mod common;

#[test]
fn undefined_variable() {
    // Given
    let script = r#"
print(.@a$);
print(.@b$);
.@b$ = "hello world";
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("Variable \".@a$\" is undefined.\ntest_script 3:6.\nl3\tprint(.@a$);\n\t      ^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("Variable \".@b$\" is undefined.\ntest_script 4:6.\nl4\tprint(.@b$);\n\t      ^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
}

#[test]
fn type_checking_string_valid() {
    // Given
    let script = r#".@a$ = "toto";
.@b$ = 1 + "toto";
.@c$ = "toto" + 1;
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
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
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(7, result.as_ref().err().unwrap().len());
    assert_eq!("Variable \".@a$\" is declared as a String but is assigned with a Number.\ntest_script 2:0.\nl2\t.@a$ = 1;\n\t^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("Subtraction operator \"-\" is not allowed for String\ntest_script 3:7.\nl3\t.@a$ = \"1\" - \"2\";\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
    assert_eq!("Subtraction operator \"-\" is not allowed for String\ntest_script 4:7.\nl4\t.@a$ = 1 - \"2\";\n\t       ^^^^^\n", result.as_ref().err().unwrap().get(2).unwrap().message());
    assert_eq!("Subtraction operator \"-\" is not allowed for String\ntest_script 5:7.\nl5\t.@a$ = \"1\" - 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(3).unwrap().message());
    assert_eq!("Multiply operator \"*\" is not allowed for String\ntest_script 6:7.\nl6\t.@a$ = \"1\" * 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(4).unwrap().message());
    assert_eq!("Modulo operator \"%\" is not allowed for String\ntest_script 7:7.\nl7\t.@a$ = \"1\" % 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(5).unwrap().message());
    assert_eq!("Divide operator \"/\" is not allowed for String\ntest_script 8:7.\nl8\t.@a$ = \"1\" / 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(6).unwrap().message());
}

#[test]
fn type_checking_number_valid() {
    // Given
    let script = r#"
    .@a = 1;
    .@a = 1 + 3;
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
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
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!("Variable \".@a\" is declared as a Number but is assigned with a String.\ntest_script 2:0.\nl2\t.@a = \"1\";\n\t^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("Variable \".@b\" is declared as a Number but is assigned with a String.\ntest_script 3:0.\nl3\t.@b = 1 + \"2\";\n\t^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
}

#[test]
fn type_checking_function_call() {
    // Given
    let script = r#"
    function str {
        return "hello";
    }

    function num {
        return 1;
    }
    .@a$ = str();
    .@b = num();
    .@c = str();
    .@d$ = num();
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(2, errors.len());
    assert_eq!(r#"Variable ".@c" is declared as a Number but is assigned with a String.
test_script 12:4.
l12	    .@c = str();
	    ^^^
"#, errors.get(0).unwrap().message());
    assert_eq!(r#"Variable ".@d$" is declared as a String but is assigned with a Number.
test_script 13:4.
l13	    .@d$ = num();
	    ^^^^
"#, errors.get(1).unwrap().message());
}

#[test]
fn undefined_function() {
    // Given
    let script = r#"undefined();"#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
}

#[test]
fn function_definition() {
    // Given
    let script = r#"function my_func {}"#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
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
    function println { // this is a native function, it is forbidden to declare a func with the same name as native function.
        .@a = 2;
    }
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(2, result.as_ref().err().unwrap().len());
    result.as_ref().err().unwrap().iter().for_each(|e| println!("{}", e));
    assert_eq!(r#"A function with name "my_func" already exists.
test_script 6:4.
l6	    function my_func {
	    ^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!(r#"A native function with name "println" already exists.
test_script 9:4.
l9	    function println { // this is a native function, it is forbidden to declare a func with the same name as native function.
	    ^
"#, result.as_ref().err().unwrap().get(1).unwrap().message());
}


#[test]
fn class_redefinition_should_be_an_error() {
    // Given
    let script = r#"
    - script My class -1, {
    }
    - script My class2 -1, {
    }
    - script My class -1, {
    }
    "#;
    // When
    let result = compile(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"Class Myclass is already defined in file "test_script" at line "l2"
test_script 6:4.
l6	    - script My class -1, {
	    ^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
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
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"Variable ".@a" is declared as a Number but is assigned with a String.
test_script 5:8.
l5	        .@a = getarg(1, "3") + 4;
	        ^^^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
}

#[test]
fn type_checking_conditional_statement() {
    // Given
    let script = r#"
    .@a = 1 == 1;                               // correct
    .@a = 1 == "1";                             // incorrect
    .@a = "1" == "1";                           // correct
    .@a = "1" == "1" && 1 == "1";               // incorrect (right part of &&)
    .@a = "1" == "1" && "1" == "1";             // correct
    .@a = "1" == "1" || "1" == "1";             // correct
    .@a = "1" > 2;                              // incorrect
    .@a = "1" >= 2;                             // incorrect
    .@a = 1 > 2;                                // incorrect
    .@a = "1" < 2;                              // incorrect
    .@a = "1" <= 2;                             // incorrect
    .@a = 1 < 2;                                // incorrect
    .@a = (getarg(0) == "RE" && !RENEWAL) || (getarg(0) == "Pre-RE" && RENEWAL);    // correct
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(6, result.as_ref().err().unwrap().len());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
test_script 4:10.
l4	    .@a = 1 == "1";                             // incorrect
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
test_script 6:24.
l6	    .@a = "1" == "1" && 1 == "1";               // incorrect (right part of &&)
	                        ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
test_script 9:10.
l9	    .@a = "1" > 2;                              // incorrect
	          ^^^^^^^
"#, result.as_ref().err().unwrap()[2].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
test_script 10:10.
l10	    .@a = "1" >= 2;                             // incorrect
	          ^^^^^^^^
"#, result.as_ref().err().unwrap()[3].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
test_script 12:10.
l12	    .@a = "1" < 2;                              // incorrect
	          ^^^^^^^
"#, result.as_ref().err().unwrap()[4].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
test_script 13:10.
l13	    .@a = "1" <= 2;                             // incorrect
	          ^^^^^^^^
"#, result.as_ref().err().unwrap()[5].message());
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
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"Can't perform logical and (&&) when left and right are not same types
test_script 4:10.
l4	    .@a = 1 && "1";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Can't perform logical and (&&) when left and right are not same types
test_script 6:10.
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
    let _result = compile_script(script, compiler::DebugFlag::None.value());
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
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"Label "Assign" is declared in "my_func" function scope but label should be declared in script scope only.
test_script 4:12.
l4	            Assign:
	            ^
"#, result.as_ref().err().unwrap()[0].message());
}

#[test]
fn array_type_checking() {
    // Given
    let script = r#"
    .@a$[0] = "hello";
    .@a$[1] = 2;
    .@b[0] = 1;
    .@b[1] = "2";
    .@c$ = .@a$[0] + .@b[0];
    .@d = .@a$[0] + .@b[0];
    .@a$ = "hello"; // variable .@a$ become a string
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(3, result.as_ref().err().unwrap().len());
    assert_eq!(r#"Variable ".@a$[]" is declared as an Array of string but an index is assigned with a Number.
test_script 4:4.
l4	    .@a$[1] = 2;
	    ^^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Variable ".@b[]" is declared as an Array of number but an index is assigned with a String.
test_script 6:4.
l6	    .@b[1] = "2";
	    ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
    assert_eq!(r#"Variable ".@d" is declared as a Number but is assigned with a String.
test_script 8:4.
l8	    .@d = .@a$[0] + .@b[0];
	    ^^^
"#, result.as_ref().err().unwrap()[2].message());
}

#[test]
fn array_type_checking_copy_array() {
    // Given
    let script = r#"
    .@toto$ = "toto";
    setarray .@a$[0], "hello", "world", .@toto$;
    copyarray .@b[0], .@a$[0], 1;
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(1, result.as_ref().err().unwrap().len());
    assert_eq!(r#"Variable ".@b[]" is declared as an Array of number but an index is assigned with a String.
test_script 5:14.
l5	    copyarray .@b[0], .@a$[0], 1;
	              ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
}


#[test]
fn native_wrong_argument_count() {
    // Given
    let script = r#"
    .@b[0] = 1;
    .@a$[0] = "1";
    getarraysize(.@b[0], .@a$[0]);
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(1, result.as_ref().err().unwrap().len());
    assert_eq!(r#"Wrong arguments: getarraysize accept at least 1 argument(s) and at most 1 argument(s) but received 2 argument(s)
test_script 5:4.
l5	    getarraysize(.@b[0], .@a$[0]);
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
}
