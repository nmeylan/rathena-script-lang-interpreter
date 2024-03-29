use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler;
use rathena_script_lang_interpreter::lang::compiler::{Compiler, DebugFlag};
use rathena_script_lang_interpreter::lang::vm::Vm;
use crate::common::{compile, compile_script, Event, VmHook};

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
    assert_eq!("Variable \".@a$\" is undefined.\n_MainScript 3:6.\nl3\tprint(.@a$);\n\t      ^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("Variable \".@b$\" is undefined.\n_MainScript 4:6.\nl4\tprint(.@b$);\n\t      ^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
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
    assert_eq!("Variable \".@a$\" is declared as a String but is assigned with a Number.\n_MainScript 2:0.\nl2\t.@a$ = 1;\n\t^^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("Subtraction operator \"-\" is not allowed for String\n_MainScript 3:7.\nl3\t.@a$ = \"1\" - \"2\";\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
    assert_eq!("Subtraction operator \"-\" is not allowed for String\n_MainScript 4:7.\nl4\t.@a$ = 1 - \"2\";\n\t       ^^^^^\n", result.as_ref().err().unwrap().get(2).unwrap().message());
    assert_eq!("Subtraction operator \"-\" is not allowed for String\n_MainScript 5:7.\nl5\t.@a$ = \"1\" - 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(3).unwrap().message());
    assert_eq!("Multiply operator \"*\" is not allowed for String\n_MainScript 6:7.\nl6\t.@a$ = \"1\" * 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(4).unwrap().message());
    assert_eq!("Modulo operator \"%\" is not allowed for String\n_MainScript 7:7.\nl7\t.@a$ = \"1\" % 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(5).unwrap().message());
    assert_eq!("Divide operator \"/\" is not allowed for String\n_MainScript 8:7.\nl8\t.@a$ = \"1\" / 2;\n\t       ^^^^^^^\n", result.as_ref().err().unwrap().get(6).unwrap().message());
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
    assert_eq!("Variable \".@a\" is declared as a Number but is assigned with a String.\n_MainScript 2:0.\nl2\t.@a = \"1\";\n\t^^^\n", result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!("Variable \".@b\" is declared as a Number but is assigned with a String.\n_MainScript 3:0.\nl3\t.@b = 1 + \"2\";\n\t^^^\n", result.as_ref().err().unwrap().get(1).unwrap().message());
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
_MainScript 12:4.
l12	    .@c = str();
	    ^^^
"#, errors.get(0).unwrap().message());
    assert_eq!(r#"Variable ".@d$" is declared as a String but is assigned with a Number.
_MainScript 13:4.
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
_MainScript 6:4.
l6	    function my_func {
	    ^
"#, result.as_ref().err().unwrap().get(0).unwrap().message());
    assert_eq!(r#"A native function with name "println" already exists.
_MainScript 9:4.
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
    assert_eq!(r#"Class Myclass is already defined in file "_MainScript" at line "l2"
_MainScript 6:4.
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
_MainScript 5:8.
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
_MainScript 4:10.
l4	    .@a = 1 == "1";                             // incorrect
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
_MainScript 6:24.
l6	    .@a = "1" == "1" && 1 == "1";               // incorrect (right part of &&)
	                        ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
_MainScript 9:10.
l9	    .@a = "1" > 2;                              // incorrect
	          ^^^^^^^
"#, result.as_ref().err().unwrap()[2].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
_MainScript 10:10.
l10	    .@a = "1" >= 2;                             // incorrect
	          ^^^^^^^^
"#, result.as_ref().err().unwrap()[3].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
_MainScript 12:10.
l12	    .@a = "1" < 2;                              // incorrect
	          ^^^^^^^
"#, result.as_ref().err().unwrap()[4].message());
    assert_eq!(r#"Can't perform comparison when left and right are not same types
_MainScript 13:10.
l13	    .@a = "1" <= 2;                             // incorrect
	          ^^^^^^^^
"#, result.as_ref().err().unwrap()[5].message());
}

#[test]
fn type_checking_with_operator() {
    // Given
    let script = r#"
    .@a = 1 + "1";
    .@a$ = 1 + "1";
    .@a$ = "1" + 0;
    .@a = "1" + 0;
    .@a$ = 1 - "0";
    .@a$ = 1 * "0";
    .@a$ = 1 / "0";
    .@a$ = 1 % "0";
    .@a = 1 - "0";
    .@a = 1 * "0";
    .@a = 1 / "0";
    .@a = 1 % "0";
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(true, result.is_err());
    assert_eq!(r#"Variable ".@a" is declared as a Number but is assigned with a String.
_MainScript 3:4.
l3	    .@a = 1 + "1";
	    ^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Variable ".@a" is declared as a Number but is assigned with a String.
_MainScript 6:4.
l6	    .@a = "1" + 0;
	    ^^^
"#, result.as_ref().err().unwrap()[1].message());
    assert_eq!(r#"Subtraction operator "-" is not allowed for String
_MainScript 7:11.
l7	    .@a$ = 1 - "0";
	           ^^^^^
"#, result.as_ref().err().unwrap()[2].message());
    assert_eq!(r#"Multiply operator "*" is not allowed for String
_MainScript 8:11.
l8	    .@a$ = 1 * "0";
	           ^^^^^
"#, result.as_ref().err().unwrap()[3].message());
    assert_eq!(r#"Divide operator "/" is not allowed for String
_MainScript 9:11.
l9	    .@a$ = 1 / "0";
	           ^^^^^
"#, result.as_ref().err().unwrap()[4].message());
    assert_eq!(r#"Modulo operator "%" is not allowed for String
_MainScript 10:11.
l10	    .@a$ = 1 % "0";
	           ^^^^^
"#, result.as_ref().err().unwrap()[5].message());
    assert_eq!(r#"Subtraction operator "-" is not allowed for String
_MainScript 11:10.
l11	    .@a = 1 - "0";
	          ^^^^^
"#, result.as_ref().err().unwrap()[6].message());
    assert_eq!(r#"Multiply operator "*" is not allowed for String
_MainScript 12:10.
l12	    .@a = 1 * "0";
	          ^^^^^
"#, result.as_ref().err().unwrap()[8].message());
    assert_eq!(r#"Divide operator "/" is not allowed for String
_MainScript 13:10.
l13	    .@a = 1 / "0";
	          ^^^^^
"#, result.as_ref().err().unwrap()[10].message());
    assert_eq!(r#"Modulo operator "%" is not allowed for String
_MainScript 14:10.
l14	    .@a = 1 % "0";
	          ^^^^^
"#, result.as_ref().err().unwrap()[12].message());
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
_MainScript 4:10.
l4	    .@a = 1 && "1";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Can't perform logical and (&&) when left and right are not same types
_MainScript 6:10.
l6	    .@a = 1 && "0";
	          ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
}

#[test]
fn type_checking_ternary() {
    // Given
    let script = r#"
    setarray .@Styles[1], 1;
    set .@Revert, 1;
    set .@Style, 1;
    set .@s,1;
    set .@menu$, ((.@Style!=1)?.@Style-1:.@Styles[.@s])+"^000000)";
    set .@menu$, " ~ Next (^0055FF"+((.@Style!=.@Styles[.@s])?.@Style+1:1)+"^000000): ~ Previous (^0055FF"+((.@Style!=1)?.@Style-1:.@Styles[.@s])+"^000000): ~ Jump to...: ~ Revert to original (^0055FF"+.@Revert+"^000000)";
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(false, result.is_err());
}

#[test]
fn undefined_label() {
    // Given
    let script = r#"#

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
_MainScript 4:12.
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
_MainScript 4:4.
l4	    .@a$[1] = 2;
	    ^^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
    assert_eq!(r#"Variable ".@b[]" is declared as an Array of number but an index is assigned with a String.
_MainScript 6:4.
l6	    .@b[1] = "2";
	    ^^^^^^
"#, result.as_ref().err().unwrap()[1].message());
    assert_eq!(r#"Variable ".@d" is declared as a Number but is assigned with a String.
_MainScript 8:4.
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
_MainScript 5:14.
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
_MainScript 5:4.
l5	    getarraysize(.@b[0], .@a$[0]);
	    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
"#, result.as_ref().err().unwrap()[0].message());
}

#[test]
fn native_input_declare_variable() {
    // Given
    let script = r#"
    input(.@choice);
    vm_dump_var("a", .@choice);
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(false, result.is_err());
}


#[test]
fn debug_opcodes() {
    // Given
    let script = r#"
    switch(select("a:b:c")) {
		case 1:
		1 + 1;
		break;
		case 2:
		input .@Style,0,29;
		break;
    }
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value()).unwrap();
    // Then
    let class_file = &result[1];
    let functions = class_file.functions.borrow();
    let main_function = functions[0].as_ref();
    for (index, op_code) in main_function.chunk.op_codes.borrow().iter().enumerate() {
        println!("[{}] {:?}", index, op_code);
    }
}

#[test]
fn nested_function_call() {
    // When
    Compiler::compile("a".to_string(), "- script _MainScript -1, {\n function a{return getarg(0) + 1;}\n
    .@a = a(a(a(a(a(a(a(a(a(a(a(1))))))))))); \n}", "native_functions_list.txt", 0);
    // Then
    assert!(true)
}

#[test]
fn declare_keyword() {
    // Given
    let script = r#"
    declare .@choice;
    vm_dump_var("a", .@choice);
    "#;
    // When
    let result = compile_script(script, compiler::DebugFlag::None.value());
    // Then
    assert_eq!(false, result.is_err());
}


#[test]
fn serialization_deserialization() {
    // Given
    let mut file_content = r#"
    goto Second;
    First:
        .@b$ = "variable in label 1";
        goto_end();
    Second:
        .@c$ = "variable in label 2";
        goto First;
    Third:
        .@d$ = "variable in label 3";
        goto First; // Hopefully this is never reach or we would have infinite loop
    End:
        .@endd$ = "the end";

    vm_dump_locals();
    function goto_end {
        goto End;
    }"#;
    let events = Arc::new(Mutex::new(HashMap::<String, Event>::new()));
    let result = compile_script(file_content, DebugFlag::None.value()).unwrap();
    // When
    let serialization_result = bitcode::serialize(&result).unwrap();
    let classes: Vec<ClassFile> = bitcode::deserialize(&serialization_result).unwrap();
    // Then
    let events_clone = events.clone();
    let vm = crate::common::setup_vm(rathena_script_lang_interpreter::lang::vm::DebugFlag::None.value());
    // When
    let vm_hook = VmHook::new( Box::new(move |e| { events_clone.lock().unwrap().insert(e.name.clone(), e); }));
    Vm::bootstrap(vm.clone(), classes, Box::new(&vm_hook));
    Vm::execute_main_script(vm, Box::new(&vm_hook), vec![]).unwrap();
    // Then
    assert_eq!(String::from("variable in label 1"), events.lock().unwrap().get("b").unwrap().value.string_value().unwrap().clone());
    assert_eq!(String::from("variable in label 2"), events.lock().unwrap().get("c").unwrap().value.string_value().unwrap().clone());
    assert_eq!(true, events.lock().unwrap().get("d").is_none());
    assert_eq!(String::from("the end"), events.lock().unwrap().get("endd").unwrap().value.string_value().unwrap().clone());
}