use std::borrow::{Borrow, Cow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;

use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Tree};
use serde::{Deserialize, Serialize};

use crate::lang::chunk::{Chunk, ClassFile, FunctionDefinition, Label, NumericOperation, OpCode, Relational};
use crate::lang::chunk::OpCode::{*};
use crate::lang::chunk::OpCode::{Add, CallFunction, CallNative, LoadConstant, LoadLocal, StoreInstance, StoreLocal};
use crate::lang::error::{CompilationError, CompilationErrorType};
use crate::lang::error::CompilationErrorType::{FunctionAlreadyDefined, LabelNotInMain, NativeAlreadyDefined, NativeArgumentCount, Type, UndefinedFunction, UndefinedLabel};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::type_checker::TypeChecker;
use crate::lang::value::{*};
use crate::lang::vm::{NATIVE_FUNCTIONS, NativeFunction, Vm};
use crate::parser::rathenascriptlanglexer::{*};
use crate::parser::rathenascriptlangparser::{*};
use crate::parser::rathenascriptlangvisitor::{*};

// Labels below will be turned into functions
const HOOK_LABEL: &[&str] = &[
    "OnInit",
    "OnInstanceInit",
    "OnInstanceDestroy",
];

pub enum DebugFlag {
    None,
    All,
    TypeChecker,
}

impl DebugFlag {
    pub fn value(&self) -> u64 {
        match self {
            DebugFlag::None => 0,
            DebugFlag::All => 0xFFFF,
            DebugFlag::TypeChecker => 2,
        }
    }
}

impl Default for DebugFlag {
    fn default() -> Self {
        DebugFlag::None
    }
}

#[allow(dead_code)]
pub struct Compiler {
    pub file_name: String,
    native_functions: Vec<NativeFunction>,
    hook_labels: Vec<String>,
    classes: Vec<ClassFile>,
    errors: RefCell<HashMap<String, Vec<CompilationError>>>,
    state: State,
    script_lines: Vec<String>,
    debug_flag: u64,
    type_checker: TypeChecker,
}

#[derive(Default)]
pub struct State {
    current_declared_class: usize,
}

pub enum VariableScope {
    Default,
    Temporary,
    GlobalPermanent,
    GlobalTemporary,
    Npc,
    InstanceLocal,
    Instance,
    PermanentAccount,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationDetail {
    pub file_name: String,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub text: String,
}

impl CompilationDetail {
    pub fn new_empty() -> Self {
        Self {
            file_name: "".to_string(),
            start_line: 0,
            start_column: 0,
            end_line: 0,
            end_column: 0,
            text: "".to_string(),
        }
    }
    pub fn single_line(&self) -> String {
        format!("{} {}:{}.  {}", self.file_name, self.start_line, self.start_column, self.text.trim())
    }
}

impl Display for CompilationDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}:{}.", self.file_name, self.start_line, self.start_column).unwrap();
        writeln!(f, "l{}\t{}", self.start_line, self.text).unwrap();
        writeln!(f, "\t{}{}", " ".repeat(self.start_column), "^".repeat(self.end_column - self.start_column + 1))
    }
}

impl Compiler {
    pub fn new(file_name: String, script: String, native_function_list_file_path: &str, debug_flag: u64) -> Self {
        let mut native_functions: Vec<NativeFunction> = NATIVE_FUNCTIONS.to_vec().iter()
            .map(NativeFunction::from_vm_native)
            .collect();
        native_functions.extend(Vm::collect_native_functions(native_function_list_file_path));
        let script_lines = script.split('\n').map(|l| l.to_string()).collect::<Vec<String>>();
        Self {
            file_name,
            native_functions,
            hook_labels: vec![],
            classes: vec![ClassFile::new("_Global".to_string(), "_globa_class_".to_string(), 0)],
            errors: RefCell::new(HashMap::new()),
            state: Default::default(),
            script_lines: script_lines.clone(),
            debug_flag,
            type_checker: TypeChecker::new(debug_flag & DebugFlag::TypeChecker.value() == DebugFlag::TypeChecker.value(), script_lines),
        }
    }

    pub fn compile_script(name: String, script: &str, native_function_list_file_path: &str, debug_flag: u64) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
        let script_declaration = format!("- script {} -1,{{ \n{}\n }}", name, script);
        Self::compile(name, script_declaration.as_str(), native_function_list_file_path, debug_flag)
    }
    pub fn compile_script_into_binary(name: String, script: &str, native_function_list_file_path: &str, debug_flag: u64) -> Result<Vec<u8>, Vec<CompilationError>> {
        let script_declaration = format!("- script {} -1,{{ \n{}\n }}", name, script);
        Self::compile(name, script_declaration.as_str(), native_function_list_file_path, debug_flag)
            .map(|result| bitcode::serialize(&result).unwrap())
    }

    pub fn from_binary(serialized_script: &Vec<u8>) -> Result<Vec<ClassFile>, String> {
        let result = bitcode::deserialize::<Vec<ClassFile>>(serialized_script);
        if let Ok(classes) = result {
            Ok(classes)
        } else {
            Err("Failed to deserialize script".to_string())
        }
    }

    pub fn compile_file_and_keep_state(&mut self, script_path: &Path) {
        let file = File::open(script_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut file_content = String::new();
        reader.read_to_string(&mut file_content).unwrap();
        self.file_name = script_path.to_str().unwrap().to_string();
        self.compile_content_and_keep_state(file_content)
    }

    pub fn compile_content_and_keep_state(&mut self, file_content: String) {
        self.script_lines = file_content.split('\n').map(|l| l.to_string()).collect::<Vec<String>>();
        let lexer = RathenaScriptLangLexer::new(InputStream::new(file_content.as_str()));
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = RathenaScriptLangParser::new(token_stream);
        let tree = parser.compilationUnit();
        self.visit_compilationUnit(tree.as_ref().unwrap());
    }

    pub fn end_compilation(&mut self) -> (Vec<ClassFile>, HashMap<String, Vec<CompilationError>>) {
        for class in self.classes.iter() {
            Self::add_hook_functions(class);
            Self::check_called_function_are_defined(self, class);
            for function in class.functions().iter() {
                Self::update_goto_jump_index(self, class, function.as_ref());
            }
        }

        (mem::take(&mut self.classes), mem::take(&mut self.errors.take()))
    }

    pub fn compile(name: String, script: &str, native_function_list_file_path: &str, debug_flag: u64) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
        let mut compiler = Compiler::new(name, script.to_string(), native_function_list_file_path, debug_flag);
        let lexer = RathenaScriptLangLexer::new(InputStream::new(script));
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = RathenaScriptLangParser::new(token_stream);
        let tree = parser.compilationUnit();
        // println!("{}", tree.unwrap().to_string_tree(&parser));
        compiler.visit_compilationUnit(tree.as_ref().unwrap());

        for class in compiler.classes.iter() {
            Self::add_hook_functions(class);
            Self::check_called_function_are_defined(&compiler, class);
            for function in class.functions().iter() {
                Self::update_goto_jump_index(&compiler, class, function.as_ref());
            }
        }

        if compiler.errors.borrow().is_empty() {
            Ok(mem::take(&mut compiler.classes))
        } else {
            let errors_ref_cell = mem::take(&mut compiler.errors);
            let mut errors: Vec<CompilationError> = vec![];
            errors_ref_cell.take().drain().for_each(|(_, errs)| errors.extend(errs));
            Err(mem::take(&mut errors))
        }
    }

    fn check_called_function_are_defined(compiler: &Compiler, class: &ClassFile) {
        for rc in class.called_functions().iter() {
            let rc = rc.clone();
            let (function_name, compilation_error_details) = rc.borrow();
            if !class.functions().iter().map(|func| func.name.clone()).any(|f| &f == function_name) && !compiler.classes[0].functions.borrow().iter().map(|func| func.name.clone()).any(|f| &f == function_name) {
                compiler.register_error_with_details_class(class, UndefinedFunction, compilation_error_details.clone(),
                                                           format!("Function \"{}\" is not defined", function_name))
            }
        }
    }

    fn add_hook_functions(class: &ClassFile) {
        if class.name == "_Global" {
            return;
        }
        let functions = class.functions();
        let main_function: &FunctionDefinition = functions.get(0).unwrap().borrow();
        for hook_label in main_function.declared_labels().iter().filter(|label| HOOK_LABEL.contains(&label.name.as_str())
            || main_function.callsub_contains(&label.name)) {
            let mut function_definition = FunctionDefinition::new(format!("_{}", hook_label.name.clone()));
            let mut chunk = Chunk::default();
            let mut declared_local_variable_references: HashMap<u64, Variable, NoopHasher> = Default::default();
            let start_index = hook_label.first_op_code_index;
            let mut index = hook_label.first_op_code_index;
            let mut condition_depth = 0;
            let mut end_if_indexes: Vec<usize> = vec![];
            loop {
                if index >= main_function.chunk.op_codes.borrow().len() { break; }
                let op_code = main_function.chunk.op_codes.borrow()[index].clone();
                let compilation_details = main_function.chunk.compilation_details.borrow()[index].clone();
                if let StoreLocal(reference) = op_code {
                    if let Some(variable) = main_function.chunk.locals.borrow().get(&reference) {
                        declared_local_variable_references.insert(reference, variable.clone());
                    }
                }
                // Update OpCode jump index
                match op_code {
                    Jump(i) => chunk.emit_op_code(OpCode::Jump(i - start_index), compilation_details),
                    OpCode::If(i) => chunk.emit_op_code(OpCode::If(i - start_index), compilation_details),
                    _ => chunk.emit_op_code(op_code.clone(), compilation_details),
                };
                // To know if we are inside a if or not
                match op_code {
                    OpCode::If(i) => {
                        if mem::discriminant(&main_function.chunk.op_codes.borrow()[i]) == mem::discriminant(&OpCode::Else) {
                            if let OpCode::Jump(j) = main_function.chunk.op_codes.borrow()[i - 1].clone() {
                                end_if_indexes.push(j);
                            }
                        } else {
                            end_if_indexes.push(i);
                        }
                        condition_depth += 1;
                    }
                    _ => {}
                };
                if let Some(if_index) = end_if_indexes.last() {
                    if *if_index <= index {
                        condition_depth -= 1;
                        end_if_indexes.pop();
                    }
                }
                if condition_depth == 0 && mem::discriminant(&op_code) == mem::discriminant(&OpCode::End) {
                    break;
                }
                index += 1;
            }
            chunk.locals = RefCell::new(declared_local_variable_references);
            function_definition.chunk = Rc::new(chunk);
            class.add_function(function_definition);
        }
    }

    fn update_goto_jump_index(compiler: &Compiler, class: &ClassFile, function: &FunctionDefinition) {
        let label_gotos_op_code = function.chunk.drop_goto_indices();
        for (label_name, indices) in label_gotos_op_code.iter() {
            let maybe_label = class.get_label(label_name);
            if let Some(label) = maybe_label {
                for (index, _) in indices {
                    function.chunk.set_op_code_at(*index, OpCode::Goto(label.first_op_code_index));
                }
            } else {
                for (_, compilation_detail) in indices {
                    compiler.register_error_with_details_class(class, UndefinedLabel, compilation_detail.clone(),
                                                               format!("Undefined label \"{}\"", label_name))
                }
            }
        }
    }

    fn add_function_to_current_class(&self, function: FunctionDefinition) -> usize {
        self.current_class().add_function(function)
    }

    fn current_declared_function(&self) -> Rc<FunctionDefinition> {
        self.current_class().current_declared_function()
    }

    fn current_class(&self) -> &ClassFile {
        self.classes.get(self.state.current_declared_class).as_ref().unwrap()
    }

    fn current_chunk(&self) -> Rc<Chunk> {
        self.current_class().current_declared_function().chunk.clone()
    }

    fn is_inside_a_main_function(&self) -> bool {
        self.current_class().is_inside_a_main_function()
    }

    fn visible_functions(&self) -> Vec<String> {
        self.current_class().functions().iter().map(|func| func.name.clone()).collect::<Vec<String>>()
    }
    fn global_class(&self) -> &ClassFile {
        self.classes.get(0).as_ref().unwrap()
    }
    fn function_returned_type(&self, function_name: &String) -> Option<ValueType> {
        let maybe_returned_type = self.current_class().get_function_returned_type(function_name);
        if maybe_returned_type.is_some() {
            return maybe_returned_type;
        }
        self.global_class().get_function_returned_type(function_name)
    }

    fn register_error<'input>(&self, error_type: CompilationErrorType, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), message: String) {
        let error = CompilationError {
            error_type,
            message,
            details: self.compilation_details_from_context(node),
        };
        self.insert_error(self.current_class(), error);
    }

    fn register_error_with_details_class(&self, class: &ClassFile, error_type: CompilationErrorType, details: CompilationDetail, message: String) {
        let error = CompilationError {
            error_type,
            message,
            details,
        };
        self.insert_error(class, error);
    }

    fn insert_error(&self, class: &ClassFile, error: CompilationError) {
        let mut errors_ref_mut = self.errors.borrow_mut();
        if !errors_ref_mut.contains_key(&class.name) {
            errors_ref_mut.insert(class.name.clone(), vec![]);
        }
        errors_ref_mut.get_mut(&class.name).unwrap().push(error);
    }

    fn compilation_details_from_context<'input>(&self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) -> CompilationDetail {
        CompilationDetail {
            file_name: self.file_name.to_string(),
            start_line: node.start().get_line() as usize,
            start_column: node.start().get_column() as usize,
            end_line: node.stop().get_line() as usize,
            end_column: node.stop().get_column() as usize,
            text: self.script_lines[node.start().get_line() as usize - 1].clone(),
        }
    }

    fn get_variable_scope(ctx: &VariableContext) -> Scope {
        let scope = if ctx.scope_specifier().is_some() {
            let scope_specifier = ctx.scope_specifier().unwrap();
            if scope_specifier.At().is_some() {
                Scope::CharacterTemporary
            } else if scope_specifier.Dollar().is_some() {
                Scope::Server
            } else if scope_specifier.DollarAt().is_some() {
                Scope::ServerTemporary
            } else if scope_specifier.Dot().is_some() {
                Scope::Npc
            } else if scope_specifier.DotAt().is_some() {
                Scope::Local
            } else if scope_specifier.Quote().is_some() {
                Scope::Instance
            } else if scope_specifier.Sharp().is_some() || scope_specifier.DoubleSharp().is_some() {
                Scope::Account
            } else {
                Scope::Local
            }
        } else {
            Scope::Character
        };
        scope
    }

    fn build_variable(ctx: &VariableContext) -> Variable {
        let scope = Self::get_variable_scope(ctx);
        let variable_name = ctx.variable_name().unwrap();
        let name = variable_name.get_child(0).as_ref().unwrap().get_text();
        Variable {
            value_ref: Variable::variable_value(ctx.Dollar().is_some(), ctx.LeftBracket().is_some()),
            name,
            scope,
        }
    }

    fn load_variable<'input>(&mut self, variable: &Variable, variable_ctx: &VariableContext, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        match variable.scope {
            Scope::Npc => {
                if let Ok(reference) = self.current_class().load_variable(variable, Scope::Npc) {
                    self.current_chunk().emit_op_code(LoadStatic(reference), self.compilation_details_from_context(node));
                } else {
                    // TODO fix
                    self.current_chunk().emit_op_code(LoadStatic(Vm::calculate_hash(variable)), self.compilation_details_from_context(node));
                    // self.register_error(CompilationErrorType::UndefinedVariable, node, format!("Static variable \"{}\" is undefined.", variable.to_script_identifier()));
                }
            }
            Scope::Instance => {
                if let Ok(reference) = self.current_class().load_variable(variable, Scope::Instance) {
                    self.current_chunk().emit_op_code(LoadInstance(reference), self.compilation_details_from_context(node));
                } else {
                    // TODO fix
                    self.current_chunk().emit_op_code(LoadInstance(Vm::calculate_hash(variable)), self.compilation_details_from_context(node));
                    // self.register_error(CompilationErrorType::UndefinedVariable, node, format!("Instance variable \"{}\" is undefined.", variable.to_script_identifier()));
                }
            }
            Scope::Local => {
                if let Ok(reference) = self.current_chunk().load_local(variable) {
                    self.current_chunk().emit_op_code(LoadLocal(reference), self.compilation_details_from_context(node));
                } else {
                    let variable_ref = Vm::calculate_hash(variable);
                    self.current_chunk().add_undefined_variable(variable_ref);
                    // When using setd for local scope we have no way to know if variable is defined or not without evaluating the setd expression, which is done at runtime.
                    // So we disable variable existence check in this case
                    if self.current_chunk().local_setd_len() > 0 || self.current_chunk().dynamically_defined_variable_contains(variable_ref) {
                        self.current_chunk().emit_op_code(LoadLocal(variable_ref), self.compilation_details_from_context(node));
                    } else {
                        self.register_error(CompilationErrorType::UndefinedVariable, node, format!("Variable \"{}\" is undefined.", variable.to_script_identifier()));
                    }
                }
            }
            Scope::Server | Scope::ServerTemporary | Scope::Account | Scope::CharacterTemporary | Scope::Character => {
                let variable_name = variable_ctx.get_text();
                let reference = self.current_chunk().add_constant(Constant::String(variable_name));
                self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(node));
                self.current_chunk().emit_op_code(LoadGlobal, self.compilation_details_from_context(node));
            }
        }

        if variable.value_ref.borrow().is_array() {
            self.visit_conditionalExpression(variable_ctx.conditionalExpression().as_ref().unwrap());
            self.type_checker.pop_expression_type();
            self.current_chunk().emit_op_code(ArrayLoad, self.compilation_details_from_context(node));
        }
    }

    // Unfortunately some native function support un-standard behavior:
    // 1. some(input, getarraysize, getvariableofnpc) accept variable reference
    // 2. some(setvariableofnpc, setd, etc..) set value to variable reference
    // 3. some terminate execution (close)
    // We need function below to handle these edge cases.
    fn visit_native_function<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), argumentsList: Option<Rc<ArgumentExpressionListContextAll<'input>>>, function_or_native_name: &String, first_argument_op_code_index: usize, argument_count: usize, native: NativeFunction) {
        if argument_count < native.min_arguments || argument_count > native.max_arguments {
            self.register_error(NativeArgumentCount, node,
                                format!("Wrong arguments: {} accept at least {} argument(s) and at most {} argument(s) but received {} argument(s)",
                                        native.name, native.min_arguments, native.max_arguments, argument_count
                                ));
            return;
        }

        if native.name == "input" {
            for (i, expr) in argumentsList.as_ref().unwrap().conditionalExpression_all().iter().enumerate() {
                if i == 0 {
                    let variable_identifier = argumentsList.as_ref().unwrap().conditionalExpression_all().get(0).unwrap().get_text();
                    let constant_reference = self.current_chunk().add_constant(Constant::String(variable_identifier));
                    self.current_chunk().emit_op_code(OpCode::LoadConstant(constant_reference), self.compilation_details_from_context(node));
                } else {
                    self.visit_conditionalExpression(expr);
                }
            }
        } else if native.name == "bonus" || native.name == "bonus2" || native.name == "bonus3" || native.name == "bonus4" || native.name == "bonus5" || native.name == "readparam" {
            // First argument of bonus* methods is not a string but a "constant", using same syntax as permanent character integer variable
            // We convert the expression into a LoadConstant, basically "bonus bStr, 10;" becomes "bonus "bStr", 10;"
            // In the native method handler we then perform string matching on the first argument to determine what to do.
            for (i, expr) in argumentsList.as_ref().unwrap().conditionalExpression_all().iter().enumerate() {
                if i == 0 {
                    let variable_identifier = argumentsList.as_ref().unwrap().conditionalExpression_all().get(0).unwrap().get_text();
                    let constant_reference = self.current_chunk().add_constant(Constant::String(variable_identifier));
                    self.current_chunk().emit_op_code(OpCode::LoadConstant(constant_reference), self.compilation_details_from_context(node));
                } else {
                    self.visit_conditionalExpression(expr);
                }
            }
        } else if argumentsList.is_some() {
            self.visit_argumentExpressionList(argumentsList.as_ref().unwrap());
        }

        if native.name == "setd" {
            let setd_variable_expression = argumentsList.as_ref().unwrap().conditionalExpression_all().get(0).unwrap().get_text();
            if setd_variable_expression.starts_with(&format!("\"{}", &Scope::Local.prefix())) {
                self.current_chunk().add_local_setd(Vm::calculate_hash(&setd_variable_expression))
            }
        } else if native.name == "getglobalarrayref" {
            // custom "native" method which just load a global array refrence on the stack
            let last_code_op = self.current_chunk().last_op_code_index();
            if mem::discriminant(&self.current_chunk().get_op_code_at(last_code_op)) == mem::discriminant(&LoadGlobal) {
                let mut array_name = argumentsList.as_ref().unwrap().conditionalExpression(0).as_ref().unwrap().get_text();
                if !array_name.contains('[') {
                    array_name = format!("{}[0]", array_name);
                }
                let reference = self.current_chunk().add_constant(Constant::String(array_name));
                self.current_chunk().set_op_code_at(last_code_op - 1, LoadConstant(reference))
            }
            return;
        } else if native.name == "getarraysize" || native.name == "implode" {
            // getarraysize accept the array name without index.
            // In case of loadglobal we use the variable string then build a variable from it. But if it does not contains bracket
            // we can't determine it is an array.
            // Code below transform array$ into array$[0] for load global.
            let last_code_op = self.current_chunk().last_op_code_index();
            if mem::discriminant(&self.current_chunk().get_op_code_at(last_code_op)) == mem::discriminant(&LoadGlobal) {
                let mut array_name = argumentsList.as_ref().unwrap().conditionalExpression(0).as_ref().unwrap().get_text();
                if !array_name.contains('[') {
                    array_name = format!("{}[0]", array_name);
                }
                let reference = self.current_chunk().add_constant(Constant::String(array_name));
                self.current_chunk().set_op_code_at(last_code_op - 1, LoadConstant(reference))
            }
        } else if native.name == "getvariableofnpc" {
            // Replacing first argument LoadStatic with a LoadConstant instead.
            // Syntax want first argument to be the variable, instead of a string with the variable name.
            // In this implementation variable will be interpreted and its value will be pushed in the stack instead of its name..
            let static_variable_identifier = argumentsList.as_ref().unwrap().conditionalExpression_all().get(0).unwrap().get_text();
            if !static_variable_identifier.starts_with("getd(") { // we can use getd to use reference of another variable containing the variable identifier
                let constant_reference = self.current_chunk().add_constant(Constant::String(static_variable_identifier));
                let npc_name_load_constant_op_code_index = self.current_chunk().last_op_code_index();
                self.current_chunk().set_op_code_at(first_argument_op_code_index, OpCode::LoadConstant(constant_reference));
                for i in (first_argument_op_code_index + 1)..npc_name_load_constant_op_code_index {
                    self.current_chunk().set_op_code_at(i, OpCode::Noop);
                }
            }
        }
        if native.name == "getarg" && argument_count > 1 {
            // do not remove default value type, so we can check at compile time that default type match variable type
            let index = self.type_checker.type_len() - 2;
            self.type_checker.remove_type_at(index);
        } else {
            self.type_checker.remove_type(argument_count);
        }
        self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&function_or_native_name), argument_count }, self.compilation_details_from_context(node));
        if let Some(returned_type) = native.return_type.as_ref() {
            self.type_checker.add_current_assigment_type(returned_type.clone(), node);
        }
        if native.name == "close" {
            self.current_chunk().emit_op_code(OpCode::End, self.compilation_details_from_context(node));
        } else if native.name == "input" {
            let variable_identifier = argumentsList.as_ref().unwrap().conditionalExpression_all().get(0).unwrap().get_text();
            let constant_reference = self.current_chunk().add_constant(Constant::String(variable_identifier.clone()));
            let variable = Variable::from_string(&variable_identifier);
            self.current_chunk().add_dynamically_defined_variable(Vm::calculate_hash(&variable));
            self.current_chunk().emit_op_code(OpCode::AssignVariable(constant_reference), self.compilation_details_from_context(node));
        }
    }

    fn goto<'input>(&mut self, label: String, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        let index = self.current_chunk().emit_op_code(OpCode::Goto(0), self.compilation_details_from_context(node));
        let detail = self.compilation_details_from_context(node);
        self.current_chunk().push_goto_index(label, index, detail);
    }

    fn store_variable<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), variable: Variable, enable_type_checking: bool) {
        if enable_type_checking {
            if let Some(current_value_type) = self.type_checker.current_type(node) {
                if variable.value_ref.borrow().is_string() && current_value_type.is_number() {
                    self.type_checker.debug_type_checking(node, "store_variable");
                    self.register_error(CompilationErrorType::Type, node,
                                        format!("Variable \"{}\" is declared as a String but is assigned with a Number.", variable.to_script_identifier()));
                }
                if variable.value_ref.borrow().is_string_array() && current_value_type.is_number() {
                    self.type_checker.debug_type_checking(node, "store_variable");
                    self.register_error(CompilationErrorType::Type, node,
                                        format!("Variable \"{}\" is declared as an Array of string but an index is assigned with a Number.", variable.to_script_identifier()));
                }
                if variable.value_ref.borrow().is_number() && current_value_type.is_string() {
                    self.type_checker.debug_type_checking(node, "store_variable");
                    self.register_error(CompilationErrorType::Type, node,
                                        format!("Variable \"{}\" is declared as a Number but is assigned with a String.", variable.to_script_identifier()));
                }

                if variable.value_ref.borrow().is_number_array() && current_value_type.is_string() {
                    self.type_checker.debug_type_checking(node, "store_variable");
                    self.register_error(CompilationErrorType::Type, node,
                                        format!("Variable \"{}\" is declared as an Array of number but an index is assigned with a String.", variable.to_script_identifier()));
                }
            }
            self.type_checker.drop_current_type(node);
        }
        match variable.scope {
            Scope::Server | Scope::Account | Scope::Character | Scope::ServerTemporary | Scope::CharacterTemporary => {
                let variable_name = variable.to_script_identifier();
                let reference = self.current_chunk().add_constant(Constant::String(variable_name));
                self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(node));
                self.current_chunk().emit_op_code(StoreGlobal, self.compilation_details_from_context(node));
            }
            Scope::Local => {
                let reference = self.current_chunk().add_local(variable);
                self.current_chunk().emit_op_code(StoreLocal(reference), self.compilation_details_from_context(node));
            }
            Scope::Instance => {
                let reference = self.current_class().add_instance_variable(variable);
                self.current_chunk().emit_op_code(StoreInstance(reference), self.compilation_details_from_context(node));
            }
            Scope::Npc => {
                let reference = self.current_class().add_static_variable(variable);
                self.current_chunk().emit_op_code(StoreStatic(reference), self.compilation_details_from_context(node));
            }
        }
    }

    fn visit_functionCall<'input>(&mut self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), argumentsList: Option<Rc<ArgumentExpressionListContextAll<'input>>>, function_or_native_name: &String, functionCallCtx: Option<&FunctionCallExpressionContext<'input>>) {
        let mut function_or_native_name = function_or_native_name.clone();
        let first_argument_op_code_index = self.current_chunk().last_op_code_index() + 1;
        let argument_count = if argumentsList.is_some() {
            argumentsList.as_ref().unwrap().conditionalExpression_all().len() as usize
        } else {
            0
        };
        if let Some(native) = self.native_functions.iter().find(|native| native.name == function_or_native_name).cloned() {
            self.visit_native_function(node, argumentsList, &function_or_native_name, first_argument_op_code_index, argument_count, native)
        } else {
            if argumentsList.is_some() {
                self.visit_argumentExpressionList(argumentsList.as_ref().unwrap());
            }
            if functionCallCtx.is_some() && functionCallCtx.as_ref().unwrap().Underscore().is_some() {
                return;
            }
            if functionCallCtx.is_some() && functionCallCtx.as_ref().unwrap().Callsub().is_some() {
                self.current_class().insert_callsub(function_or_native_name.clone());
                function_or_native_name = format!("_{}", function_or_native_name);
            }
            self.type_checker.remove_type(argument_count);
            self.current_class().add_called_function((function_or_native_name.clone(), self.compilation_details_from_context(node)));
            if let Some(returned_type) = self.function_returned_type(&function_or_native_name) {
                self.type_checker.add_current_assigment_type(returned_type, node);
            }
            self.current_chunk().emit_op_code(CallFunction { reference: Vm::calculate_hash(&function_or_native_name), argument_count }, self.compilation_details_from_context(node));
        }
    }
}

impl<'input> ParseTreeVisitor<'input, RathenaScriptLangParserContextType> for Compiler {}

impl<'input> RathenaScriptLangVisitor<'input> for Compiler {
    fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) {
        if ctx.String().is_some() {
            let reference = self.current_chunk().add_constant(
                Constant::String(
                    remove_quote_from_string(ctx.String().as_ref().unwrap().symbol.text.as_ref()) // TODO check if it can be done by antlr skip instead.
                ));
            self.type_checker.add_current_assigment_type(ValueType::String, ctx);
            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(ctx));
        }
        if ctx.Number().is_some() {
            let number_value = &ctx.Number().unwrap().symbol.text;
            let mut value: i32 = parse_number(number_value.clone());
            if ctx.Minus().is_some() {
                value *= -1;
            }
            let reference = self.current_chunk().add_constant(Constant::Number(value));
            self.type_checker.add_current_assigment_type(ValueType::Number, ctx);
            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(ctx));

        }
        if ctx.True().is_some() {
            let reference = self.current_chunk().add_constant(Constant::Number(1));
            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(ctx));
        }
        if ctx.False().is_some() {
            let reference = self.current_chunk().add_constant(Constant::Number(0));
            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(ctx));
        }
        if ctx.Identifier().is_some() {
            let reference = self.current_chunk().add_constant(
                Constant::String(
                    ctx.Identifier().unwrap().symbol.text.deref().to_string()
                ));
            self.current_chunk().emit_op_code(OpCode::LoadConstant(reference), self.compilation_details_from_context(ctx));
            self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&"loadconstant"), argument_count: 1 }, self.compilation_details_from_context(ctx));
        }
        if ctx.variable().is_some() {
            self.visit_variable(ctx.variable().as_ref().unwrap());
        }
    }

    fn visit_incrementThenLoad(&mut self, ctx: &IncrementThenLoadContext<'input>) {
        let variable = Self::build_variable(&ctx.variable().unwrap());
        if !variable.value_ref.value_type.is_number() {
            self.register_error(Type, ctx, format!("Operator \"{}\" is not allowed for String", if ctx.IncrementOp().is_some() { "++" } else { "--" }));
            return;
        }
        let one = self.current_chunk().add_constant(Constant::Number(1));
        if ctx.IncrementOp().is_some() {
            self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
            self.current_chunk().emit_op_code(LoadConstant(one), self.compilation_details_from_context(ctx));
            self.current_chunk().emit_op_code(Add, self.compilation_details_from_context(ctx));
        } else {
            self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
            self.current_chunk().emit_op_code(LoadConstant(one), self.compilation_details_from_context(ctx));
            self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Subtract), self.compilation_details_from_context(ctx));
        }
        self.store_variable(ctx, variable.clone(), false);
        self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
    }

    fn visit_loadThenIncrement(&mut self, ctx: &LoadThenIncrementContext<'input>) {
        let variable = Self::build_variable(&ctx.variable().unwrap());
        if !variable.value_ref.value_type.is_number() {
            self.register_error(Type, ctx, format!("Operator \"{}\" is not allowed for String", if ctx.IncrementOp().is_some() { "++" } else { "--" }));
            return;
        }
        self.type_checker.inc_current_expression_index(ctx, "visit_loadThenIncrement");
        let one = self.current_chunk().add_constant(Constant::Number(1));
        if ctx.IncrementOp().is_some() {
            self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
            self.current_chunk().emit_op_code(LoadValue, self.compilation_details_from_context(ctx));
            self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
            self.current_chunk().emit_op_code(LoadConstant(one), self.compilation_details_from_context(ctx));
            self.current_chunk().emit_op_code(Add, self.compilation_details_from_context(ctx));
            self.store_variable(ctx, variable, false);
        } else {
            self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
            self.current_chunk().emit_op_code(LoadValue, self.compilation_details_from_context(ctx));
            self.load_variable(&variable, &ctx.variable().unwrap(), ctx);
            self.current_chunk().emit_op_code(LoadConstant(one), self.compilation_details_from_context(ctx));
            self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Subtract), self.compilation_details_from_context(ctx));
            self.store_variable(ctx, variable, false);
        }
        self.type_checker.add_current_assigment_type(ValueType::Number, ctx);
        self.type_checker.dec_current_expression_index(true, ctx, "visit_loadThenIncrement");
    }

    fn visit_functionCallExpressionWithoutParentheses(&mut self, ctx: &FunctionCallExpressionWithoutParenthesesContext<'input>) {
        let mut function_or_native_name = if ctx.Identifier().is_some() {
            ctx.Identifier().unwrap().symbol.text.to_string()
        } else {
            return;
        };
        self.visit_functionCall(ctx, ctx.argumentExpressionList(), &mut function_or_native_name, None);
    }

    fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) {
        let mut function_or_native_name = if ctx.Identifier().is_some() {
            ctx.Identifier().unwrap().symbol.text.to_string()
        } else if ctx.String().is_some() {
            remove_quote_from_string(ctx.String().as_ref().unwrap().symbol.text.as_ref())
        } else if ctx.Underscore().is_some() {
            String::from("_")
        } else {
            return;
        };

        self.visit_functionCall(ctx, ctx.argumentExpressionList(), &mut function_or_native_name, Some(ctx));
    }

    fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>) {
        let native_name = ctx.get_child(0).as_ref().unwrap().get_text();

        // example: menu "String", Label, "A:B", Label,...;
        // we will transform this command above into:
        // 1. call to native "menu" function to wait player answer
        // 2. generate if(option_number) + goto Label
        if native_name == "menu" {
            let argument_count = ctx.menuOptionText_all().len() as usize;
            ctx.menuOptionText_all().iter().for_each(|option_text_node| {
                if option_text_node.String().is_some() {
                    let constant_ref = self.current_chunk().add_constant(Constant::String(remove_quote_from_string(&option_text_node.String().as_ref().unwrap().symbol.text)));
                    self.current_chunk().emit_op_code(LoadConstant(constant_ref), self.compilation_details_from_context(ctx));
                } else {
                    self.visit_children(option_text_node.as_ref());
                }
            });
            // 1. Call native menu function
            self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&native_name), argument_count }, self.compilation_details_from_context(ctx));

            // 2. iterate over all String, to generate if + goto. A string can contain multiple options
            let mut option_number = 0;
            // When label is "-" we want to jump right after the menu
            let mut goto_op_code_indicies_to_update: Vec<usize> = vec![];
            for (i, options) in ctx.menuOptionText_all().iter().enumerate() {
                let iter_count = if options.String().is_some() {
                    options.String().as_ref().unwrap().symbol.text.split(':').count()
                } else {
                    1 // In case option text is an expression, we consider that returned String does not contains ":"
                };
                for _option in 0..iter_count {
                    option_number += 1;
                    let constant_ref = self.current_chunk().add_constant(Constant::Number(option_number));
                    self.current_chunk().emit_op_code(LoadConstant(constant_ref), self.compilation_details_from_context(options.as_ref()));
                    let current_op_code_index = self.current_chunk().emit_op_code(OpCode::SwitchCompare, self.compilation_details_from_context(options.as_ref()));
                    let next_if_index = current_op_code_index + 3; // jump to next "if", when "if" is false. + 3, because current op code is "SwitchCompare" next op code is "if" then "goto".
                    self.current_chunk().emit_op_code(OpCode::If(next_if_index), self.compilation_details_from_context(options.as_ref()));
                    let menu_label = ctx.menuLabel(i).unwrap();
                    if menu_label.Identifier().is_some() {
                        self.goto(menu_label.Identifier().as_ref().unwrap().symbol.text.to_string(), menu_label.as_ref());
                    } else {
                        // meaning that Label is "-"
                        goto_op_code_indicies_to_update.push(self.current_chunk().emit_op_code(OpCode::Goto(0), self.compilation_details_from_context(menu_label.as_ref())));
                    }
                }
            }
            for i in goto_op_code_indicies_to_update.iter() {
                self.current_chunk().set_op_code_at(*i, OpCode::Goto(self.current_chunk().last_op_code_index() + 1));
            }
            return;
        }
        self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&native_name), argument_count: 0 }, self.compilation_details_from_context(ctx));
        if native_name == "close" {
            self.current_chunk().emit_op_code(OpCode::End, self.compilation_details_from_context(ctx));
        }
    }


    fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>) {
        self.visit_children(ctx);
    }

    fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) {
        self.visit_unaryExpression(&ctx.unaryExpression(ctx.unaryExpression_all().len() - 1).unwrap());

        for (i, _) in ctx.unaryExpression_all().iter().enumerate().rev() {
            if i == ctx.unaryExpression_all().len() - 1 {
                continue;
            }
            self.visit_unaryExpression(&ctx.unaryExpression(i).unwrap());
            let operator = ctx.multiplicativeOperator(i).unwrap();

            if operator.Star().is_some() {
                if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                    self.type_checker.debug_type_checking(ctx, "visit_multiplicativeExpression");
                    self.register_error(Type, ctx, "Multiply operator \"*\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Multiply), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.Slash().is_some() {
                self.type_checker.debug_type_checking(ctx, "visit_multiplicativeExpression");
                if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                    self.register_error(Type, ctx, "Divide operator \"/\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Divide), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.Percent().is_some() {
                self.type_checker.debug_type_checking(ctx, "visit_multiplicativeExpression");
                if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                    self.register_error(Type, ctx, "Modulo operator \"%\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Modulo), self.compilation_details_from_context(operator.as_ref()));
            }
        }
    }

    fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) {
        self.visit_additiveExpression(&ctx.additiveExpression(ctx.additiveExpression_all().len() - 1).unwrap());

        for (i, _) in ctx.additiveExpression_all().iter().enumerate().rev() {
            if i == ctx.additiveExpression_all().len() - 1 {
                continue;
            }
            self.visit_additiveExpression(&ctx.additiveExpression(i).unwrap());
            let operator = ctx.shiftOperator(i).unwrap();

            if operator.DoubleLeftCaret().is_some() {
                if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                    self.type_checker.debug_type_checking(ctx, "visit_shiftExpression");
                    self.register_error(Type, ctx, "Shift operator \"<<\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::LeftShift), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.DoubleRightCaret().is_some() {
                if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                    self.type_checker.debug_type_checking(ctx, "visit_shiftExpression");
                    self.register_error(Type, ctx, "Shift operator \">>\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::RightShift), self.compilation_details_from_context(operator.as_ref()));
            }
        }
    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) {
        // self.visit_children(ctx);
        self.visit_multiplicativeExpression(&ctx.multiplicativeExpression(ctx.multiplicativeExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.multiplicativeExpression_all().iter().enumerate().rev() {
            if i == ctx.multiplicativeExpression_all().len() - 1 {
                continue;
            }
            self.visit_multiplicativeExpression(&ctx.multiplicativeExpression(i).unwrap());
            let operator = ctx.additiveOperator(i).unwrap();
            if operator.Plus().is_some() {
                self.current_chunk().emit_op_code(Add, self.compilation_details_from_context(operator.as_ref()));
            } else if operator.Minus().is_some() {
                if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                    self.type_checker.debug_type_checking(ctx, "visit_additiveExpression");
                    self.register_error(Type, ctx, "Subtraction operator \"-\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Subtract), self.compilation_details_from_context(operator.as_ref()));
            }
        }
    }

    fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) {
        self.visit_shiftExpression(&ctx.shiftExpression(ctx.shiftExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.shiftExpression_all().iter().enumerate().rev() {
            if i == ctx.shiftExpression_all().len() - 1 {
                continue;
            }
            self.visit_shiftExpression(&ctx.shiftExpression(i).unwrap());
            if !self.type_checker.current_types_are_same_type() {
                self.type_checker.debug_type_checking(ctx, "visit_relationalExpression");
                self.register_error(Type, ctx, "Can't perform comparison when left and right are not same types".to_string());
            }
            let operator = ctx.relationalOperator(i).unwrap();
            if operator.RightCaret().is_some() {
                self.current_chunk().emit_op_code(OpCode::Relational(Relational::GT), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.RightCaretEqual().is_some() {
                self.current_chunk().emit_op_code(OpCode::Relational(Relational::GTE), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.LeftCaret().is_some() {
                self.current_chunk().emit_op_code(OpCode::Relational(Relational::LT), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.LeftCaretEqual().is_some() {
                self.current_chunk().emit_op_code(OpCode::Relational(Relational::LTE), self.compilation_details_from_context(operator.as_ref()));
            }
            self.type_checker.clear_current_expression_types();
            self.type_checker.add_current_assigment_type(ValueType::Number, ctx);
        }
    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) {
        self.visit_relationalExpression(&ctx.relationalExpression(ctx.relationalExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.relationalExpression_all().iter().enumerate().rev() {
            if i == ctx.relationalExpression_all().len() - 1 {
                continue;
            }
            self.type_checker.inc_current_expression_index(ctx, "visit_equalityExpression");
            self.visit_relationalExpression(&ctx.relationalExpression(i).unwrap());
            self.type_checker.dec_current_expression_index(false, ctx, "visit_equalityExpression");
            if !self.type_checker.current_types_are_same_type() {
                self.type_checker.debug_type_checking(ctx, "visit_equalityExpression");
                self.register_error(Type, ctx, "Can't perform comparison when left and right are not same types".to_string());
            }
            self.type_checker.clear_current_expression_types();
            self.type_checker.add_current_assigment_type(ValueType::Number, ctx);
            let operator = ctx.equalityOperator(i).unwrap();
            if operator.DoubleEqual().is_some() {
                self.current_chunk().emit_op_code(OpCode::Equal, self.compilation_details_from_context(operator.as_ref()));
            } else if operator.BangEqual().is_some() {
                self.current_chunk().emit_op_code(OpCode::NotEqual, self.compilation_details_from_context(operator.as_ref()));
            }
        }
    }


    fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) {
        self.visit_inclusiveOrExpression(&ctx.inclusiveOrExpression(ctx.inclusiveOrExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.inclusiveOrExpression_all().iter().enumerate().rev() {
            if i == ctx.inclusiveOrExpression_all().len() - 1 {
                continue;
            }
            self.type_checker.inc_current_expression_index(ctx, "visit_logicalAndExpression");
            self.visit_inclusiveOrExpression(&ctx.inclusiveOrExpression(i).unwrap());
            self.type_checker.dec_current_expression_index(false, ctx, "visit_logicalAndExpression");
            if !self.type_checker.current_types_are_same_type() {
                self.type_checker.debug_type_checking(ctx, "visit_logicalAndExpression");
                self.register_error(Type, ctx, "Can't perform logical and (&&) when left and right are not same types".to_string());
            }
            self.type_checker.clear_current_expression_types();
            self.type_checker.add_current_assigment_type(ValueType::Number, ctx);
            self.current_chunk().emit_op_code(OpCode::LogicalAnd, self.compilation_details_from_context(ctx));
        }
    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) {
        self.visit_logicalAndExpression(&ctx.logicalAndExpression(ctx.logicalAndExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.logicalAndExpression_all().iter().enumerate().rev() {
            if i == ctx.logicalAndExpression_all().len() - 1 {
                continue;
            }
            self.type_checker.inc_current_expression_index(ctx, "visit_logicalOrExpression");
            self.visit_logicalAndExpression(&ctx.logicalAndExpression(i).unwrap());
            self.type_checker.dec_current_expression_index(false, ctx, "visit_logicalOrExpression");
            if !self.type_checker.current_types_are_same_type() {
                self.type_checker.debug_type_checking(ctx, "visit_logicalOrExpression");
                self.register_error(Type, ctx, "Can't perform logical or (||) when left and right are not same types".to_string());
            }
            self.type_checker.clear_current_expression_types();
            self.type_checker.add_current_assigment_type(ValueType::Number, ctx);
            self.current_chunk().emit_op_code(OpCode::LogicalOr, self.compilation_details_from_context(ctx));
        }
    }


    fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) {
        if !ctx.assignmentLeftExpression_all().is_empty() {
            if ctx.Setarray().is_some() {
                if let Some(left) = ctx.assignmentLeftExpression(0) {
                    // Array can be assigned using setarray too
                    // *setarray <array name>[<first value>],<value>{,<value>...<value>};
                    self.visit_conditionalExpression(&ctx.conditionalExpression().unwrap()); // <value>. In this language declaration require a value to assign
                    self.visit_assignmentLeftExpression(&left); // <array name>[<first value>]. Declare array variable.
                    let argument_count = if ctx.argumentExpressionList().is_none() {
                        0
                    } else { // {,<value>...<value>}
                        ctx.argumentExpressionList().unwrap().conditionalExpression_all().len() as usize
                    };
                    if argument_count > 0 {
                        self.visit_variable(left.variable().as_ref().unwrap());
                        self.visit_argumentExpressionList(&ctx.argumentExpressionList().unwrap());
                        self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&"setarray"), argument_count: argument_count + 1 }, self.compilation_details_from_context(ctx));
                    }
                    return;
                }

            } else if ctx.Copyarray().is_some() {
                let left = ctx.assignmentLeftExpression(0).unwrap();
                // Array can be assigned using copyarray too
                // *copyarray <destination array>[<first value>],<source array>[<first value>],<amount of data to copy>
                self.visit_conditionalExpression(&ctx.conditionalExpression().unwrap()); // <source array>[<first value>]. Declare array variable.
                self.visit_assignmentLeftExpression(&left); // <destination array>[<first value>]. Declare array variable.
                self.visit_variable(left.variable().as_ref().unwrap()); // Retrieve declared destination array
                self.visit_conditionalExpression(&ctx.conditionalExpression().unwrap()); // Retrieve source array
                self.type_checker.drop_current_type(ctx);
                // TODO ensure that argument list contains only 1 element.
                self.visit_argumentExpressionList(&ctx.argumentExpressionList().unwrap()); // <amount of data to copy>
                self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&"copyarray"), argument_count: 3 }, self.compilation_details_from_context(ctx));
            } else {
                for (i, left) in ctx.assignmentLeftExpression_all().iter().enumerate() {
                    self.visit_conditionalExpression(&ctx.conditionalExpression().unwrap());
                    if ctx.assignmentOperator(i).is_some() {
                        let assignment_operator = &ctx.assignmentOperator(i).unwrap();
                        // Convert a += 1; into a = a + 1;
                        if assignment_operator.PlusEqual().is_some() {
                            if left.variable().is_some() {
                                let variable = Self::build_variable(&left.variable().unwrap());
                                self.load_variable(&variable, &left.variable().unwrap(), ctx);
                            }
                            self.current_chunk().emit_op_code(Add, self.compilation_details_from_context(ctx));
                        } else if assignment_operator.MinusEqual().is_some() {
                            if left.variable().is_some() {
                                let variable = Self::build_variable(&left.variable().unwrap());
                                self.load_variable(&variable, &left.variable().unwrap(), ctx);
                            }
                            self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Subtract), self.compilation_details_from_context(ctx));
                        }
                    }
                    self.visit_assignmentLeftExpression(left);
                }
            }
        } else if ctx.Setarray().is_some() && ctx.functionCallExpression().is_some() { // edge case handling
            if let Some(function_call) = ctx.functionCallExpression() {
                if function_call.Identifier().unwrap().get_text() == "getelementofarray" {
                    if let Some(getlementofarray_arguments) = function_call.argumentExpressionList() {
                        if getlementofarray_arguments.conditionalExpression_all().len() != 2 {
                            self.register_error(CompilationErrorType::Generic, function_call.as_ref(), "getelementofarray only accept 2 arguments".to_string());
                            return;
                        }
                        self.visit_conditionalExpression(&ctx.conditionalExpression().unwrap()); // <value>
                        self.visit_conditionalExpression(getlementofarray_arguments.conditionalExpression_all().get(0).as_ref().unwrap());
                        self.visit_conditionalExpression(getlementofarray_arguments.conditionalExpression_all().get(1).as_ref().unwrap());


                        self.current_chunk().emit_op_code(ArrayStore, self.compilation_details_from_context(ctx));
                    }
                }
            }
        } else if ctx.Set().is_some() && ctx.functionCallExpression().is_some() { // edge case handling
            // handle set(getd(expr), assignement_expr); -> transform into setd(expr, assigment_expr);
            let function_call = ctx.functionCallExpression().unwrap();
            if function_call.Identifier().unwrap().get_text() != "getd" {
                self.register_error(CompilationErrorType::Generic, function_call.as_ref(), "Only \"getd\" function allowed here".to_string());
                return;
            }
            if function_call.argumentExpressionList().unwrap().conditionalExpression_all().len() != 1 {
                self.register_error(CompilationErrorType::Generic, function_call.as_ref(), "\"getd\" accept only 1 argument".to_string());
                return;
            }
            self.visit_argumentExpressionList(function_call.argumentExpressionList().as_ref().unwrap());
            self.visit_conditionalExpression(&ctx.conditionalExpression().unwrap());
            let setd_variable_expression = function_call.argumentExpressionList().unwrap().conditionalExpression_all().get(0).unwrap().get_text();
            if setd_variable_expression.starts_with(&format!("\"{}", &Scope::Local.prefix())) {
                self.current_chunk().add_local_setd(Vm::calculate_hash(&setd_variable_expression))
            }
            let native_name = "setd";
            self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&native_name), argument_count: 2 }, self.compilation_details_from_context(ctx));
        } else {
            self.visit_children(ctx);
        }
    }

    fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) {
        self.type_checker.inc_current_expression_index(ctx, "visit_conditionalExpression");
        if ctx.QuestionMark().is_some() {
            self.visit_logicalOrExpression(ctx.logicalOrExpression().as_ref().unwrap());
            let if_index = self.current_chunk().emit_op_code(OpCode::If(0), self.compilation_details_from_context(ctx));
            self.visit_conditionalExpression(ctx.conditionalExpression(0).as_ref().unwrap());
            let jump_to_index = self.current_chunk().last_op_code_index() + 2;
            self.current_chunk().set_op_code_at(if_index, OpCode::If(jump_to_index));
            let then_jump_index = self.current_chunk().emit_op_code(OpCode::Jump(jump_to_index - 1), self.compilation_details_from_context(ctx));
            self.current_chunk().emit_op_code(OpCode::Else, self.compilation_details_from_context(ctx));
            self.visit_conditionalExpression(ctx.conditionalExpression(1).as_ref().unwrap());
            let jump_to_index = self.current_chunk().last_op_code_index() + 1;
            self.current_chunk().set_op_code_at(then_jump_index, OpCode::Jump(jump_to_index));
        } else {
            self.visit_children(ctx);
        }
        self.type_checker.dec_current_expression_index(true, ctx, "visit_conditionalExpression");
    }

    fn visit_assignmentLeftExpression(&mut self, ctx: &AssignmentLeftExpressionContext<'input>) {
        if ctx.variable().is_some() {
            let variable = Self::build_variable(&ctx.variable().unwrap().clone());
            let is_array = variable.value_ref.borrow().is_array();
            self.store_variable(ctx, variable, true);

            if is_array {
                self.visit_conditionalExpression(ctx.variable().as_ref().unwrap().conditionalExpression().as_ref().unwrap());
                self.type_checker.pop_current_expression_types();
                self.current_chunk().emit_op_code(ArrayStore, self.compilation_details_from_context(ctx));
            }
        }
    }

    fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) {
        if ctx.Label().is_some() {
            let label_name = ctx.Label().unwrap().symbol.text.clone();
            let label_name = label_name[0..label_name.len() - 1].to_string(); // remove ':' in label name
            if !self.is_inside_a_main_function() {
                self.register_error(LabelNotInMain, ctx,
                                    format!("Label \"{}\" is declared in \"{}\" function scope but label should be declared in script scope only.",
                                            label_name, self.current_declared_function().name));
                return;
            }
            let label_start_index = if self.current_chunk().last_op_code_index() > 0 { self.current_chunk().last_op_code_index() + 1 } else { 0 };
            self.visit_children(ctx);
            let label_end_index = self.current_chunk().last_op_code_index() + 1;
            self.current_declared_function().insert_label(Label {
                name: label_name,
                first_op_code_index: label_start_index,
                last_op_code_index: label_end_index,
            });
        }
    }

    fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) {
        self.type_checker.reset_state();
        self.visit_children(ctx);
    }

    fn visit_declarationStatement(&mut self, ctx: &DeclarationStatementContext<'input>) {
        let mut variable = Self::build_variable(&ctx.variable().unwrap().clone());
        let is_array = variable.value_ref.borrow().is_array();
        self.store_variable(ctx, variable, true);

        if is_array {
            self.visit_conditionalExpression(ctx.variable().as_ref().unwrap().conditionalExpression().as_ref().unwrap());
            self.current_chunk().emit_op_code(ArrayStore, self.compilation_details_from_context(ctx));
        }
    }

    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) {
        self.visit_children(ctx);
        if let Some(operator) = ctx.unaryOperator() {
            if operator.Bang().is_some() {
                let constant_ref = self.current_chunk().add_constant(Constant::Number(0));
                self.current_chunk().emit_op_code(OpCode::LoadConstant(constant_ref), self.compilation_details_from_context(ctx));
                self.current_chunk().emit_op_code(OpCode::Equal, self.compilation_details_from_context(ctx));
            } else if operator.Tilde().is_some() {
                self.current_chunk().emit_op_code(OpCode::BitNot, self.compilation_details_from_context(ctx));
            }
        }
    }

    fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) {
        if ctx.If().is_some() {
            self.visit_conditionalExpression(ctx.conditionalExpression().as_ref().unwrap());
            let if_index = self.current_chunk().emit_op_code(OpCode::If(0), self.compilation_details_from_context(ctx));
            self.visit_statement(ctx.statement(0).as_ref().unwrap());
            let jump_to_index = self.current_chunk().last_op_code_index() + 2;
            self.current_chunk().set_op_code_at(if_index, OpCode::If(jump_to_index));

            if ctx.Else().is_some() {
                let then_jump_index = self.current_chunk().emit_op_code(OpCode::Jump(jump_to_index - 1), self.compilation_details_from_context(ctx));
                self.current_chunk().emit_op_code(OpCode::Else, self.compilation_details_from_context(ctx));
                self.visit_statement(ctx.statement(1).as_ref().unwrap());
                let jump_to_index = self.current_chunk().last_op_code_index() + 1;
                self.current_chunk().set_op_code_at(then_jump_index, OpCode::Jump(jump_to_index));
            } else {
                self.current_chunk().set_op_code_at(if_index, OpCode::If(jump_to_index - 1));
            }
        } else {
            self.visit_children(ctx);
        }
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        self.type_checker.reset_state();
        self.visit_children(ctx);
    }

    fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>) {
        self.current_chunk().add_new_block_state();
        self.visit_conditionalExpression(ctx.conditionalExpression().as_ref().unwrap());
        let switch_expression_index = self.current_chunk().last_op_code_index();
        let switch_block = ctx.switchBlock();
        let switch_block = switch_block.as_ref().unwrap();
        /*
         * 1. we collect all case statement and generate an if/else if/else block, with goto in their body.
         *   goto index will be case statement block.
         */
        let mut if_op_code_indices_to_update: Vec<usize> = vec![];
        let mut case_condition_op_code_count: Vec<usize> = vec![];
        let mut default_index: Option<usize> = None;
        let mut goto_op_code_indices_to_update: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, switch_block_group) in switch_block.switchBlockStatementGroup_all().iter().enumerate() {
            goto_op_code_indices_to_update.insert(i, vec![]);
            for switch_labels in switch_block_group.switchLabels().iter() {
                for (_j, label) in switch_labels.switchLabel_all().iter().enumerate() {
                    let (_switch_expression_op_code, _) = self.current_chunk().clone_op_code_at(switch_expression_index);
                    let last_op_code_index_before_case = self.current_chunk().last_op_code_index();
                    if label.Case().is_some() {
                        if let Some(constant_expression) = label.constantExpression().as_ref() {
                            self.visit_constantExpression(constant_expression);
                        } else if let Some(label_expression) = label.Label() {
                            // Load constant e.g: Job_novice
                            let reference = self.current_chunk().add_constant(Constant::String(label_expression.get_text().replace(":", "")));
                            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(label.as_ref()));
                            self.current_chunk().emit_op_code(LoadGlobal, self.compilation_details_from_context(label.as_ref()));
                        }
                        case_condition_op_code_count.push(self.current_chunk().last_op_code_index() - last_op_code_index_before_case);
                        self.type_checker.drop_current_type(ctx);
                        self.current_chunk().emit_op_code(OpCode::SwitchCompare, self.compilation_details_from_context(label.as_ref()));
                        let if_index = self.current_chunk().emit_op_code(OpCode::If(0), self.compilation_details_from_context(label.as_ref()));
                        let goto_case_statement_index = self.current_chunk().emit_op_code(OpCode::Jump(0), self.compilation_details_from_context(label.as_ref()));
                        if_op_code_indices_to_update.push(if_index);
                        goto_op_code_indices_to_update.get_mut(&i).unwrap().push(goto_case_statement_index);
                    } else if label.Default().is_some() {
                        default_index = Some(
                            self.current_chunk().emit_op_code(OpCode::Else, self.compilation_details_from_context(ctx))
                        );
                        let goto_case_statement_index = self.current_chunk().emit_op_code(OpCode::Jump(0), self.compilation_details_from_context(label.as_ref()));
                        goto_op_code_indices_to_update.get_mut(&i).unwrap().push(goto_case_statement_index);
                    }
                }
            }
        }
        /*
         * 2. we iterate over all case statement block, collect their first code op index
         */
        for (i, switch_block_group) in switch_block.switchBlockStatementGroup_all().iter().enumerate() {
            self.type_checker.drop_current_type(ctx);
            for goto_index in goto_op_code_indices_to_update.get(&i).unwrap().iter() {
                self.current_chunk().set_op_code_at(*goto_index, OpCode::Jump(self.current_chunk().last_op_code_index() + 1));
            }
            let block_item_list = switch_block_group.blockItemList();
            let block_item_list = block_item_list.as_ref().unwrap();
            self.visit_blockItemList(block_item_list);
        }
        /*
         * 3.Update all case "if" op_code to jump after to next case when not match
        */
        let end_of_switch_op_code = self.current_chunk().last_op_code_index();
        let mut i = 0;
        loop {
            if i >= if_op_code_indices_to_update.len() - 1 {
                if let Some(default_index) = default_index {
                    // Last if to be updated to jump to default label
                    self.current_chunk().set_op_code_at(if_op_code_indices_to_update[i], OpCode::If(default_index));
                } else {
                    // Last if to be updated to jump to end of switch
                    self.current_chunk().set_op_code_at(if_op_code_indices_to_update[i], OpCode::If(end_of_switch_op_code));
                }
                break;
            }
            // "If" is updated to jump to next if, when not match
            self.current_chunk().set_op_code_at(if_op_code_indices_to_update[i], OpCode::If(if_op_code_indices_to_update[i + 1] - case_condition_op_code_count[i + 1] - 1));
            i += 1;
        }
        let block_state = self.current_chunk().drop_block_state();

        /*
         * 4.Update all "break" op_code to jump after the switch statement
        */
        block_state.break_op_code_indices.borrow().iter().for_each(|index| {
            self.current_chunk().set_op_code_at(*index, OpCode::Jump(self.current_chunk().last_op_code_index() + 1));
        });
    }


    fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) {
        if ctx.For().is_some() {
            let option = ctx.forCondition();
            let for_condition = option.as_ref().unwrap();
            self.visit_forDeclaration(for_condition.forDeclaration().as_ref().unwrap());
            let for_expression_index = self.current_chunk().last_op_code_index();
            let mut for_if_index = 0;
            if for_condition.forStopExpression().is_some() {
                self.visit_forStopExpression(for_condition.forStopExpression().as_ref().unwrap());
                for_if_index = self.current_chunk().emit_op_code(OpCode::If(0), self.compilation_details_from_context(ctx));
            }

            self.current_chunk().add_new_block_state();
            self.visit_statement(ctx.statement().as_ref().unwrap());

            if for_condition.forExpression().is_some() {
                self.visit_forExpression(for_condition.forExpression().as_ref().unwrap());
            }
            let block_state = self.current_chunk().drop_block_state();
            let for_statement_end = self.current_chunk().emit_op_code(OpCode::Jump(for_expression_index + 1), self.compilation_details_from_context(ctx));
            if for_condition.forStopExpression().is_some() {
                self.current_chunk().set_op_code_at(for_if_index, OpCode::If(for_statement_end + 1));
            }
            block_state.break_op_code_indices.borrow().iter().for_each(|index| {
                self.current_chunk().set_op_code_at(*index, OpCode::Jump(for_statement_end + 1));
            });
        } else if ctx.While().is_some() {
            let do_jump_index = if ctx.Do().is_some() {
                Some(self.current_chunk().emit_op_code(OpCode::Jump(0), self.compilation_details_from_context(ctx)))
            } else {
                None
            };
            let while_start_index = self.current_chunk().last_op_code_index();
            self.visit_conditionalExpression(ctx.conditionalExpression().as_ref().unwrap());
            let while_if_index = self.current_chunk().emit_op_code(OpCode::If(0), self.compilation_details_from_context(ctx));
            if let Some(do_jump_index) = do_jump_index {
                self.current_chunk().set_op_code_at(do_jump_index, OpCode::Jump(self.current_chunk().last_op_code_index() + 1));
            }
            self.visit_statement(ctx.statement().as_ref().unwrap());
            let while_statement_end = self.current_chunk().emit_op_code(OpCode::Jump(while_start_index + 1), self.compilation_details_from_context(ctx));
            self.current_chunk().set_op_code_at(while_if_index, OpCode::If(while_statement_end + 1));
        } else {
            self.visit_children(ctx)
        }
    }

    fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) {
        self.visit_children(ctx);
        if ctx.Return().is_some() {
            let not_empty_return = ctx.conditionalExpression().is_some();
            self.current_chunk().emit_op_code(OpCode::Return(not_empty_return), self.compilation_details_from_context(ctx));
        } else if ctx.Break().is_some() {
            let index = self.current_chunk().emit_op_code(OpCode::Jump(0), self.compilation_details_from_context(ctx));
            self.current_chunk().push_block_break_index(index);
        } else if ctx.Goto().is_some() {
            let label = ctx.Identifier().unwrap().symbol.text.clone();
            self.goto(label.to_string(), ctx);
        } else if ctx.End().is_some() {
            self.current_chunk().emit_op_code(OpCode::End, self.compilation_details_from_context(ctx));
        }
    }

    fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) {
        self.visit_equalityExpression(&ctx.equalityExpression(ctx.equalityExpression_all().len() - 1).unwrap());

        for (i, _) in ctx.equalityExpression_all().iter().enumerate().rev() {
            if i == ctx.equalityExpression_all().len() - 1 {
                continue;
            }
            self.visit_equalityExpression(&ctx.equalityExpression(i).unwrap());
            if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                self.type_checker.debug_type_checking(ctx, "visit_andExpression");
                self.register_error(Type, ctx, "And operator \"&\" is not allowed for String".to_string());
            }
            self.current_chunk().emit_op_code(OpCode::BitAnd, self.compilation_details_from_context(ctx.equalityExpression(i).unwrap().as_ref()));
        }
    }

    fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) {
        self.visit_exclusiveOrExpression(&ctx.exclusiveOrExpression(ctx.exclusiveOrExpression_all().len() - 1).unwrap());

        for (i, _) in ctx.exclusiveOrExpression_all().iter().enumerate().rev() {
            if i == ctx.exclusiveOrExpression_all().len() - 1 {
                continue;
            }
            self.visit_exclusiveOrExpression(&ctx.exclusiveOrExpression(i).unwrap());
            if self.type_checker.current_sub_expression_type().is_some() && self.type_checker.current_sub_expression_type().unwrap().is_string() {
                self.type_checker.debug_type_checking(ctx, "visit_andExpression");
                self.register_error(Type, ctx, "Or operator \"|\" is not allowed for String".to_string());
            }
            self.current_chunk().emit_op_code(OpCode::BitOr, self.compilation_details_from_context(ctx.exclusiveOrExpression(i).unwrap().as_ref()));
        }
    }

    fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>) {
        self.visit_children(ctx);
        self.type_checker.reset_state();
    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) {
        if ctx.SemiColon().is_some() {
            // We don't care of function declared like this: function fn_name;
            return;
        }
        let function_name = &ctx.Identifier().unwrap().symbol.text;
        let function_name = function_name.clone().to_string();
        if self.visible_functions().contains(&function_name) {
            self.register_error(FunctionAlreadyDefined, ctx, format!("A function with name \"{}\" already exists.", function_name));
            return;
        }
        if self.native_functions.iter().any(|native| native.name == function_name) {
            self.register_error(NativeAlreadyDefined, ctx, format!("A native function with name \"{}\" already exists.", function_name));
            return;
        }
        let function = FunctionDefinition::new(function_name);
        if ctx.Script().is_some() {
            self.classes[0].add_function(function); // Add global function
        } else {
            self.add_function_to_current_class(function);
        }
        self.visit_children(ctx);
        // TODO: it won't work for all cases, to refactor
        self.current_declared_function().set_returned_type(self.type_checker.drop_current_type(ctx));
        self.current_class().set_current_declared_function_index(0);
    }

    fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) {
        let mut name = String::from("");
        for child in ctx.scriptName().as_ref().unwrap().get_children() {
            name = format!("{}{}", name, child.get_text());
        }
        if let Some(class) = self.classes.iter().find(|c| c.name == name) {
            self.register_error(CompilationErrorType::ClassAlreadyDefined, ctx, format!("Class {} is already defined in file \"{}\" at line \"l{}\"", name, class.defined_in_file_name, class.defined_at_line));
        } else {
            let detail = self.compilation_details_from_context(ctx);
            self.classes.push(ClassFile::new_with_main_function(name, self.file_name.to_string(), detail.start_line));
        }
        self.state.current_declared_class += 1;
        self.visit_compoundStatement(ctx.compoundStatement().as_ref().unwrap());
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        let variable = Self::build_variable(ctx);
        self.type_checker.add_current_assignment_type_from_variable(&variable, ctx);
        self.load_variable(&variable, ctx, ctx);
    }

    fn visit_scriptName(&mut self, ctx: &ScriptNameContext<'input>) {
        let mut name = String::from("");
        for child in ctx.get_children() {
            name = format!("{}{}", name, child.get_text());
        }
    }
}

pub fn parse_number(num: Cow<str>) -> i32 {
    let maybe_i32 = if num.starts_with("0x") || num.starts_with("0X") {
        i32::from_str_radix(&num.as_ref()[2..], 16)
    } else {
        num.parse::<i32>()
    };
    if maybe_i32.is_err() {
        panic!("Expected number to be i32, but was {} due to {}", num, maybe_i32.err().unwrap());
    }
    maybe_i32.unwrap()
}

fn remove_quote_from_string(string: &str) -> String {
    string[1..string.len() - 1].to_string()
}