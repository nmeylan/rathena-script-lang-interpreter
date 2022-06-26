use std::borrow::{Borrow, Cow};
use std::default::Default;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::{mem};

use std::ops::Deref;

use std::rc::Rc;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::{InputStream};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{NodeText, ParseTree, ParseTreeVisitor, Tree};
use crate::parser::rathenascriptlangvisitor::{*};
use crate::parser::rathenascriptlanglexer::{*};
use crate::parser::rathenascriptlangparser::{*};
use crate::lang::vm::{NATIVE_FUNCTIONS, Vm};

use crate::lang::chunk::OpCode::{*};
use crate::lang::chunk::{Chunk, NumericOperation, OpCode, Relational, ClassFile, FunctionDefinition, Label};
use crate::lang::chunk::OpCode::{Add, CallFunction, CallNative, LoadConstant, LoadLocal, StoreInstance, StoreLocal};
use crate::lang::error::{CompilationError, CompilationErrorType};
use crate::lang::error::CompilationErrorType::{FunctionAlreadyDefined, Generic, LabelNotInMain, NativeAlreadyDefined, NativeArgumentCount, Type, UndefinedFunction, UndefinedLabel};
use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{*};
use crate::util::file::read_lines;

// Labels below will be turned into functions
const HOOK_LABEL: &[&str] = &[
    "OnInit",
    "OnInstanceInit",
    "OnInstanceDestroy",
];

#[derive(Clone)]
struct NativeFunction {
    pub name: String,
    pub return_type: Option<ValueType>,
    pub min_arguments: usize,
    pub max_arguments: usize,
}

impl NativeFunction {
    fn from_vm_native(vm_native: &crate::lang::vm::NativeFunction) -> Self {
        NativeFunction::new(
            vm_native.name.to_string(),
            vm_native.return_type.clone(),
            vm_native.min_arguments,
            vm_native.max_arguments,
        )
    }

    fn new(name: String, return_type: Option<ValueType>, min_arguments: usize, max_arguments: usize) -> Self {
        Self {
            name,
            return_type,
            min_arguments,
            max_arguments,
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Compiler {
    file_name: String,
    native_functions: Vec<NativeFunction>,
    hook_labels: Vec<String>,
    classes: Vec<ClassFile>,
    errors: RefCell<Vec<CompilationError>>,
    state: State,
    script_lines: Vec<String>,
}

#[derive(Default)]
pub struct State {
    current_assignment_types: Vec<ValueType>,
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
#[derive(Debug, Clone)]
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
    fn new(file_name: String, script: String, native_function_list_file_path: &str) -> Self {
        let native_functions = Self::collect_native_functions(native_function_list_file_path);
        Self {
            file_name,
            native_functions,
            hook_labels: vec![],
            classes: vec![ClassFile::new("_Global".to_string(), "_globa_class_".to_string(), 0)],
            errors: RefCell::new(vec![]),
            state: Default::default(),
            script_lines: script.split('\n').map(|l| l.to_string()).collect::<Vec<String>>(),
        }
    }

    fn collect_native_functions(native_function_list_file_path: &str) -> Vec<NativeFunction> {
        let mut native_functions: Vec<NativeFunction> = NATIVE_FUNCTIONS.to_vec().iter()
            .map(|native_function| NativeFunction::from_vm_native(&native_function))
            .collect();
        let result = read_lines(native_function_list_file_path);
        if result.is_err() {
            panic!("{}", result.err().unwrap());
        }
        if let Ok(lines) = result {
            for line in lines.flatten() {
                let line = line.trim();
                if line.starts_with('/') {
                    continue;
                }
                let split = line.split(',');
                let split: Vec<&str> = split.map(|item| item.trim()).collect();
                let min_arguments = split[1].parse::<usize>().unwrap();
                let max_arguments = split[2].parse::<usize>().unwrap();
                let return_type = if split.len() > 3 {
                    match split[3] {
                        "Number" | "number" => Some(ValueType::Number),
                        "String" | "string" => Some(ValueType::String),
                        _ => None
                    }
                } else {
                    None
                };
                native_functions.push(NativeFunction { name: split[0].to_string(), return_type, min_arguments, max_arguments});
            }
        } else {
            panic!()
        }
        native_functions
    }
    pub fn compile_script(name: String, script: &str, native_function_list_file_path: &str) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
        Self::compile(name, format!("- script _MainScript -1,{{ \n{}\n }}", script).as_str(), native_function_list_file_path)
    }

    pub fn compile(name: String, script: &str, native_function_list_file_path: &str) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
        let mut compiler = Compiler::new(name, script.to_string(), native_function_list_file_path);
        let lexer = RathenaScriptLangLexer::new(InputStream::new(script));
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = RathenaScriptLangParser::new(token_stream);
        let tree = parser.compilationUnit();
        // println!("{}", tree.unwrap().to_string_tree(&parser));
        compiler.visit_compilationUnit(tree.as_ref().unwrap());

        for class in compiler.classes.iter() {
            Self::check_called_function_are_defined(&compiler, class);
            Self::add_hook_functions(class);
            for function in class.functions().iter() {
                Self::update_goto_jump_index(&compiler, class, function.as_ref());
            }
        }

        if compiler.errors.borrow().is_empty() {
            Ok(mem::take(&mut compiler.classes))
        } else {
            let errors_ref_cell = mem::replace(&mut compiler.errors, RefCell::new(vec![]));
            Err(errors_ref_cell.take())
        }
    }

    fn check_called_function_are_defined(compiler: &Compiler, class: &ClassFile) {
        for rc in class.called_functions().iter() {
            let rc = rc.clone();
            let (function_name, compilation_error_details) = rc.borrow();
            if !class.functions().iter().map(|func| func.name.clone()).any(|f| &f == function_name) {
                compiler.register_error_with_details(UndefinedFunction, compilation_error_details.clone(),
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
        for hook_label in main_function.declared_labels().iter().filter(|label| HOOK_LABEL.contains(&label.name.as_str())) {
            let mut function_definition = FunctionDefinition::new(format!("_{}", hook_label.name.clone()));
            let mut chunk = Chunk::default();
            let mut declared_local_variable_references: HashMap<u64, Variable, NoopHasher> = Default::default();
            for index in hook_label.first_op_code_index..hook_label.last_op_code_index {
                let op_code = main_function.chunk.op_codes.borrow()[index].clone();
                let compilation_details = main_function.chunk.compilation_details.borrow()[index].clone();
                if let StoreLocal(reference) = op_code {
                    if let Some(variable) = main_function.chunk.locals.borrow().get(&reference) {
                        declared_local_variable_references.insert(reference, variable.clone());
                    }
                }
                chunk.emit_op_code(op_code.clone(), compilation_details);
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
                    compiler.register_error_with_details(UndefinedLabel, compilation_detail.clone(),
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
        self.errors.borrow_mut().push(error);
    }

    fn register_error_with_details(&self, error_type: CompilationErrorType, details: CompilationDetail, message: String) {
        let error = CompilationError {
            error_type,
            message,
            details,
        };
        self.errors.borrow_mut().push(error);
    }

    fn compilation_details_from_context<'input>(&self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) -> CompilationDetail {
        CompilationDetail {
            file_name: self.file_name.clone(),
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
                Scope::Character // TODO: Temporary
            } else if scope_specifier.Dollar().is_some() || scope_specifier.DollarAt().is_some() {
                Scope::Server
                // } else if scope_specifier.DollarAt().is_some() {
                //     Scope::Server // TODO: Temporary
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
            Scope::Local
        };
        scope
    }

    fn add_current_assignment_type_from_variable(&mut self, var: &Variable) {
        match var.value_ref.borrow().deref().value_type {
            ValueType::String => self.add_current_assigment_type(ValueType::String),
            ValueType::Number => self.add_current_assigment_type(ValueType::Number),
            ValueType::Array(_) => {
                if var.value_ref.borrow().is_string_array() {
                    self.add_current_assigment_type(ValueType::String)
                } else {
                    self.add_current_assigment_type(ValueType::Number)
                }
            }
        }
    }

    fn add_current_assigment_type(&mut self, value_type: ValueType) {
        self.state.current_assignment_types.push(value_type)
    }

    fn remove_current_assigment_type(&mut self, count: usize) {
        for _i in 0..count {
            self.state.current_assignment_types.pop();
        }
    }

    fn current_assignment_type_drop(&mut self) -> Option<ValueType> {
        let assignment_types = mem::take(&mut self.state.current_assignment_types);
        if assignment_types.is_empty() {
            return None;
        }
        if assignment_types.iter().all(|v| v.is_number()) {
            Some(ValueType::Number)
        } else {
            Some(ValueType::String)
        }
    }

    fn current_assignment_type(&mut self) -> ValueType {
        if self.state.current_assignment_types.iter().all(|v| v.is_number()) {
            ValueType::Number
        } else {
            ValueType::String
        }
    }

    fn current_assignment_types_are_same_type(&self) -> bool {
        self.state.current_assignment_types.iter().all(|v| v.is_number())
            || self.state.current_assignment_types.iter().all(|v| v.is_string())
    }

    fn build_variable(ctx: &VariableContext) -> (Variable, Option<usize>) {
        let scope = Self::get_variable_scope(ctx);
        let variable_name = ctx.variable_name().unwrap();
        let name = variable_name.Identifier().unwrap().symbol.text.deref().to_string();
        let variable = Variable {
            value_ref: RefCell::new(Variable::variable_value(variable_name.Dollar().is_some(), variable_name.LeftBracket().is_some())),
            name,
            scope,
        };
        let index = if variable.value_ref.borrow().is_array() {
            let index_str = variable_name.Number().unwrap().symbol.text.clone();
            Some(parse_number(index_str) as usize)
        } else {
            None
        };
        (variable, index)
    }

    fn load_variable<'input>(&mut self, variable: &Variable, index: Option<usize>, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        match variable.scope {
            Scope::Server => {}
            Scope::Account => {}
            Scope::Character => {}
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
                    if self.current_chunk().local_setd_len() > 0 {
                        self.current_chunk().emit_op_code(LoadLocal(variable_ref), self.compilation_details_from_context(node));
                    } else {
                        self.register_error(CompilationErrorType::UndefinedVariable, node, format!("Variable \"{}\" is undefined.", variable.to_script_identifier()));
                    }
                }
            }
        }

        if variable.value_ref.borrow().is_array() {
            self.current_chunk().emit_op_code(ArrayLoad(index.unwrap()), self.compilation_details_from_context(node));
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
            self.add_current_assigment_type(ValueType::String);
            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(ctx));
        }
        if ctx.Number().is_some() {
            let number_value = &ctx.Number().unwrap().symbol.text;
            let reference = self.current_chunk().add_constant(Constant::Number(parse_number(number_value.clone())));
            self.add_current_assigment_type(ValueType::Number);
            self.current_chunk().emit_op_code(LoadConstant(reference), self.compilation_details_from_context(ctx));
        }
        if ctx.Identifier().is_some() {
            // TODO ensure it is a native, otherwise is is a function
            // let reference = self.current_chunk().add_native(Native{name: ctx.Identifier().unwrap().symbol.text.deref().to_string()});
            // self.current_chunk().emit_op_code(OpCode::LoadConstant(reference));
            // self.current_chunk().emit_reference(reference);
        }
        if ctx.variable().is_some() {
            self.visit_variable(ctx.variable().as_ref().unwrap());
        }
        if ctx.expression().is_some() {
            self.visit_expression(ctx.expression().as_ref().unwrap());
        }
        // self.visit_children(ctx)
    }

    fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) {
        let first_argument_op_code_index = self.current_chunk().last_op_code_index() + 1;
        let argument_count = if ctx.argumentExpressionList().is_some() {
            self.visit_argumentExpressionList(ctx.argumentExpressionList().as_ref().unwrap());
            ctx.argumentExpressionList().unwrap().assignmentExpression_all().len() as usize
        } else {
            0
        };
        let function_or_native_name = if ctx.Identifier().is_some() {
            ctx.Identifier().unwrap().symbol.text.to_string()
        } else if ctx.String().is_some() {
            remove_quote_from_string(ctx.String().as_ref().unwrap().symbol.text.as_ref())
        } else {
            return;
        };

        if let Some(native) = self.native_functions.iter().find(|native| native.name == function_or_native_name).cloned() {
            if argument_count < native.min_arguments || argument_count > native.max_arguments {
                self.register_error(NativeArgumentCount, ctx,
                                    format!("Wrong arguments: {} accept at least {} argument(s) and at most {} argument(s) but received {} argument(s)",
                                            native.name, native.min_arguments, native.max_arguments, argument_count
                                    ));
                return;
            }
            if native.name == "getarg" && argument_count > 1 {
                // do not remove default value type, so we can check at compile time that default type match variable type
                self.state.current_assignment_types.remove(self.state.current_assignment_types.len() - 2);
            } else if native.name == "setd" {
                let setd_variable_expression = ctx.argumentExpressionList().unwrap().assignmentExpression_all().get(0).unwrap().get_text();
                if setd_variable_expression.starts_with(&format!("\"{}", &Scope::Local.prefix())) {
                    self.current_chunk().add_local_setd(Vm::calculate_hash(&setd_variable_expression))
                }
            } else if native.name == "getvariableofnpc" {
                // Replacing first argument LoadStatic with a LoadConstant instead.
                // Syntax want first argument to be the variable, instead of a string with the variable name.
                // In this implementation variable will be interpreted and its value will be pushed in the stack instead of its name..
                let static_variable_identifier = ctx.argumentExpressionList().unwrap().assignmentExpression_all().get(0).unwrap().get_text();
                if !static_variable_identifier.starts_with("getd(") { // we can use getd to use reference of another variable containing the variable identifier
                    let constant_reference = self.current_chunk().add_constant(Constant::String(static_variable_identifier));
                    let npc_name_load_constant_op_code_index = self.current_chunk().last_op_code_index();
                    self.current_chunk().set_op_code_at(first_argument_op_code_index, OpCode::LoadConstant(constant_reference));
                    for i in (first_argument_op_code_index + 1)..npc_name_load_constant_op_code_index {
                        self.current_chunk().set_op_code_at(i, OpCode::Noop);
                    }
                }
            } else {
                self.remove_current_assigment_type(argument_count);
            }
            self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&function_or_native_name), argument_count }, self.compilation_details_from_context(ctx));
            if let Some(returned_type) = native.return_type.as_ref() {
                self.add_current_assigment_type(returned_type.clone());
            }
        } else {
            self.remove_current_assigment_type(argument_count);
            self.current_class().add_called_function((function_or_native_name.clone(), self.compilation_details_from_context(ctx)));
            if let Some(returned_type) = self.function_returned_type(&function_or_native_name) {
                self.add_current_assigment_type(returned_type);
            }
            self.current_chunk().emit_op_code(CallFunction { reference: Vm::calculate_hash(&function_or_native_name), argument_count }, self.compilation_details_from_context(ctx));
        }
    }

    fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>) {
        for expression in ctx.assignmentExpression_all().iter() {
            if expression.Number().is_some() {
                let number_value = &expression.Number().unwrap().symbol.text;
                self.current_chunk().add_constant(Constant::Number(parse_number(number_value.clone())));
            }
        }
        self.visit_children(ctx);
    }

    fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) {
        self.visit_castExpression(&ctx.castExpression(ctx.castExpression_all().len() - 1).unwrap());

        for (i, _) in ctx.castExpression_all().iter().enumerate().rev() {
            if i == ctx.castExpression_all().len() - 1 {
                continue;
            }
            self.visit_castExpression(&ctx.castExpression(i).unwrap());
            let operator = ctx.multiplicativeOperator(i).unwrap();

            if operator.Star().is_some() {
                if self.current_assignment_type().is_string() {
                    self.register_error(Type, ctx, "Multiply operator \"*\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Multiply), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.Slash().is_some() {
                if self.current_assignment_type().is_string() {
                    self.register_error(Type, ctx, "Divide operator \"/\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Divide), self.compilation_details_from_context(operator.as_ref()));
            } else if operator.Percent().is_some() {
                if self.current_assignment_type().is_string() {
                    self.register_error(Type, ctx, "Modulo operator \"%\" is not allowed for String".to_string());
                }
                self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Modulo), self.compilation_details_from_context(operator.as_ref()));
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
                if self.current_assignment_type().is_string() {
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
            if !self.current_assignment_types_are_same_type() {
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
            self.current_assignment_type_drop();
            self.add_current_assigment_type(ValueType::Number);
        }
    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) {
        self.visit_relationalExpression(&ctx.relationalExpression(ctx.relationalExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.relationalExpression_all().iter().enumerate().rev() {
            if i == ctx.relationalExpression_all().len() - 1 {
                continue;
            }
            self.visit_relationalExpression(&ctx.relationalExpression(i).unwrap());
            if !self.current_assignment_types_are_same_type() {
                self.register_error(Type, ctx, "Can't perform comparison when left and right are not same types".to_string());
            }
            let operator = ctx.equalityOperator(i).unwrap();
            if operator.DoubleEqual().is_some() {
                self.current_chunk().emit_op_code(OpCode::Equal, self.compilation_details_from_context(operator.as_ref()));
            } else if operator.BangEqual().is_some() {
                self.current_chunk().emit_op_code(OpCode::NotEqual, self.compilation_details_from_context(operator.as_ref()));
            }
            self.current_assignment_type_drop();
            self.add_current_assigment_type(ValueType::Number);
        }
    }

    fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) {
        self.visit_inclusiveOrExpression(&ctx.inclusiveOrExpression(ctx.inclusiveOrExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.inclusiveOrExpression_all().iter().enumerate().rev() {
            if i == ctx.inclusiveOrExpression_all().len() - 1 {
                continue;
            }
            self.visit_inclusiveOrExpression(&ctx.inclusiveOrExpression(i).unwrap());
            if !self.current_assignment_types_are_same_type() {
                self.register_error(Type, ctx, "Can't perform logical and (&&) when left and right are not same types".to_string());
            }
            self.current_chunk().emit_op_code(OpCode::LogicalAnd, self.compilation_details_from_context(ctx));
            self.current_assignment_type_drop();
            self.add_current_assigment_type(ValueType::Number);
        }
    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) {
        self.visit_logicalAndExpression(&ctx.logicalAndExpression(ctx.logicalAndExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.logicalAndExpression_all().iter().enumerate().rev() {
            if i == ctx.logicalAndExpression_all().len() - 1 {
                continue;
            }
            self.visit_logicalAndExpression(&ctx.logicalAndExpression(i).unwrap());
            if !self.current_assignment_types_are_same_type() {
                self.register_error(Type, ctx, "Can't perform logical or (||) when left and right are not same types".to_string());
            }
            self.current_chunk().emit_op_code(OpCode::LogicalOr, self.compilation_details_from_context(ctx));
            self.current_assignment_type_drop();
            self.add_current_assigment_type(ValueType::Number);
        }
    }

    fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) {
        let maybe_left = ctx.assignmentLeftExpression();
        if let Some(left) = maybe_left {
            if ctx.Setarray().is_some() {
                // Array can be assigned using setarray too
                // *setarray <array name>[<first value>],<value>{,<value>...<value>};
                self.visit_assignmentExpression(&ctx.assignmentExpression().unwrap()); // <value>. In this language declaration require a value to assign
                self.visit_assignmentLeftExpression(&left); // <array name>[<first value>]. Declare array variable.
                let argument_count = ctx.argumentExpressionList().unwrap().assignmentExpression_all().len() as usize; // {,<value>...<value>}
                if argument_count > 0 {
                    self.visit_variable(left.variable().as_ref().unwrap());
                    self.visit_argumentExpressionList(&ctx.argumentExpressionList().unwrap());
                    self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&"setarray"), argument_count: argument_count + 1 }, self.compilation_details_from_context(ctx));
                }
            } else if ctx.Copyarray().is_some() {
                // Array can be assigned using copyarray too
                // *copyarray <destination array>[<first value>],<source array>[<first value>],<amount of data to copy>
                self.visit_assignmentExpression(&ctx.assignmentExpression().unwrap()); // <source array>[<first value>]. Declare array variable.
                self.visit_assignmentLeftExpression(&left); // <destination array>[<first value>]. Declare array variable.
                self.visit_variable(left.variable().as_ref().unwrap()); // Retrieve declared destination array
                self.visit_assignmentExpression(&ctx.assignmentExpression().unwrap()); // Retrieve source array
                self.current_assignment_type_drop();
                // TODO ensure that argument list contains only 1 element.
                self.visit_argumentExpressionList(&ctx.argumentExpressionList().unwrap()); // <amount of data to copy>
                self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&"copyarray"), argument_count: 3 }, self.compilation_details_from_context(ctx));
            } else {
                self.visit_assignmentExpression(&ctx.assignmentExpression().unwrap());
                if ctx.assignmentOperator().is_some() {
                    let assignment_operator = &ctx.assignmentOperator().unwrap();
                    // Convert a += 1; into a = a + 1;
                    if assignment_operator.PlusEqual().is_some() {
                        if left.variable().is_some() {
                            let (variable, index) = Self::build_variable(&left.variable().unwrap());
                            self.load_variable(&variable, index, ctx);
                        }
                        self.current_chunk().emit_op_code(Add, self.compilation_details_from_context(ctx));
                    } else if assignment_operator.MinusEqual().is_some() {
                        if left.variable().is_some() {
                            let (variable, index) = Self::build_variable(&left.variable().unwrap());
                            self.load_variable(&variable, index, ctx);
                        }
                        self.current_chunk().emit_op_code(NumericOperation(NumericOperation::Subtract), self.compilation_details_from_context(ctx));
                    }
                }
                self.visit_assignmentLeftExpression(&left);
            }
        } else if ctx.Set().is_some() && ctx.functionCallExpression().is_some() { // edge case handling
            // handle set(getd(expr), assignement_expr); -> transform into setd(expr, assigment_expr);
            let function_call = ctx.functionCallExpression().unwrap();
            if function_call.Identifier().unwrap().get_text() != "getd" {
                self.register_error(CompilationErrorType::Generic, function_call.as_ref(), "Only \"getd\" function allowed here".to_string());
                return;
            }
            if function_call.argumentExpressionList().unwrap().assignmentExpression_all().len() != 1 {
                self.register_error(CompilationErrorType::Generic, function_call.as_ref(), "\"getd\" accept only 1 argument".to_string());
                return;
            }
            self.visit_argumentExpressionList(function_call.argumentExpressionList().as_ref().unwrap());
            self.visit_assignmentExpression(&ctx.assignmentExpression().unwrap());
            let setd_variable_expression = function_call.argumentExpressionList().unwrap().assignmentExpression_all().get(0).unwrap().get_text();
            if setd_variable_expression.starts_with(&format!("\"{}", &Scope::Local.prefix())) {
                self.current_chunk().add_local_setd(Vm::calculate_hash(&setd_variable_expression))
            }
            let native_name = "setd";
            self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&native_name), argument_count: 2 }, self.compilation_details_from_context(ctx));
        } else {
            self.visit_children(ctx);
        }
    }

    fn visit_assignmentLeftExpression(&mut self, ctx: &AssignmentLeftExpressionContext<'input>) {
        if ctx.Identifier().is_some() {
            // Number + char permanent variable (ie: not ending with '$', nor having any scope) match Identifier instead of variable.
            let _name = ctx.Identifier().unwrap().symbol.text.deref().to_string();
            // TODO
        } else if ctx.variable().is_some() {
            let (variable, index) = Self::build_variable(&ctx.variable().unwrap());
            if let Some(current_value_type) = self.current_assignment_type_drop() {
                if variable.value_ref.borrow().is_string() && current_value_type.is_number() {
                    self.register_error(CompilationErrorType::Type, ctx,
                                        format!("Variable \"{}\" is declared as a String but is assigned with a Number.", variable.to_script_identifier()));
                }
                if variable.value_ref.borrow().is_string_array() && current_value_type.is_number() {
                    self.register_error(CompilationErrorType::Type, ctx,
                                        format!("Variable \"{}\" is declared as an Array of string but index {} is assigned with a Number.", variable.to_script_identifier(), index.unwrap()));
                }
                if variable.value_ref.borrow().is_number() && current_value_type.is_string() {
                    self.register_error(CompilationErrorType::Type, ctx,
                                        format!("Variable \"{}\" is declared as a Number but is assigned with a String.", variable.to_script_identifier()));
                }

                if variable.value_ref.borrow().is_number_array() && current_value_type.is_string() {
                    self.register_error(CompilationErrorType::Type, ctx,
                                        format!("Variable \"{}\" is declared as an Array of number but index {} is assigned with a String.", variable.to_script_identifier(), index.unwrap()));
                }
            }
            let is_array = variable.value_ref.borrow().is_array();
            match variable.scope {
                Scope::Server | Scope::Account | Scope::Character => {
                    // TODO
                }
                Scope::Local => {
                    let reference = self.current_chunk().add_local(variable);
                    self.current_chunk().emit_op_code(StoreLocal(reference), self.compilation_details_from_context(ctx));
                }
                Scope::Instance => {
                    let reference = self.current_class().add_instance_variable(variable);
                    self.current_chunk().emit_op_code(StoreInstance(reference), self.compilation_details_from_context(ctx));
                }
                Scope::Npc => {
                    let reference = self.current_class().add_static_variable(variable);
                    self.current_chunk().emit_op_code(StoreStatic(reference), self.compilation_details_from_context(ctx));
                }
            }
            if is_array {
                let number_value = ctx.variable().as_ref().unwrap().variable_name().as_ref().unwrap().Number().as_ref().unwrap().symbol.text.clone();
                self.current_chunk().emit_op_code(ArrayStore(parse_number(number_value) as usize), self.compilation_details_from_context(ctx));
            }
        }
    }

    fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) {
        self.visit_children(ctx);
        self.current_assignment_type_drop();
    }

    fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>) {
        if ctx.variable().is_some() {
            self.visit_variable(ctx.variable().as_ref().unwrap());
        } else {
            self.visit_children(ctx)
        }
    }

    fn visit_specifierQualifierList(&mut self, ctx: &SpecifierQualifierListContext<'input>) {
        self.visit_children(ctx)
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
        self.visit_children(ctx);
        self.current_assignment_type_drop();
    }

    fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) {
        if ctx.If().is_some() {
            self.visit_expression(ctx.expression().as_ref().unwrap());
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

    fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>) {
        self.current_chunk().add_new_block_state();
        self.visit_expression(ctx.expression().as_ref().unwrap());
        let switch_expression_index = self.current_chunk().last_op_code_index();
        let switch_block = ctx.switchBlock();
        let switch_block = switch_block.as_ref().unwrap();
        /*
         * 1. we collect all case statement and generate an if/else if/else block, with goto in their body.
         *   goto index will be case statement block.
         */
        let mut if_op_code_indices_to_update: Vec<usize> = vec![];
        let mut default_index: Option<usize> = None;
        let mut goto_op_code_indices_to_update: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, switch_block_group) in switch_block.switchBlockStatementGroup_all().iter().enumerate() {
            goto_op_code_indices_to_update.insert(i, vec![]);
            for switch_labels in switch_block_group.switchLabels().iter() {
                for label in switch_labels.switchLabel_all().iter() {
                    let (switch_expression_op_code, _) = self.current_chunk().clone_op_code_at(switch_expression_index);
                    if label.Case().is_some() {
                        self.current_chunk().emit_op_code(switch_expression_op_code, self.compilation_details_from_context(ctx));
                        self.visit_constantExpression(label.constantExpression().as_ref().unwrap());
                        self.current_assignment_type_drop();
                        self.current_chunk().emit_op_code(OpCode::Equal, self.compilation_details_from_context(label.as_ref()));
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
            self.current_assignment_type_drop();
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
        let end_of_switch_op_code = self.current_chunk().last_op_code_index() + 1;
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
            // If is updated to jump to next if, when not match
            self.current_chunk().set_op_code_at(if_op_code_indices_to_update[i], OpCode::If(if_op_code_indices_to_update[i + 1] - 3));
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
            self.visit_expression(ctx.expression().as_ref().unwrap());
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
            let not_empty_return = ctx.expression().is_some();
            self.current_chunk().emit_op_code(OpCode::Return(not_empty_return), self.compilation_details_from_context(ctx));
        } else if ctx.Break().is_some() {
            let index = self.current_chunk().emit_op_code(OpCode::Jump(0), self.compilation_details_from_context(ctx));
            self.current_chunk().push_block_break_index(index);
        } else if ctx.Goto().is_some() {
            let index = self.current_chunk().emit_op_code(OpCode::Goto(0), self.compilation_details_from_context(ctx));
            let label = ctx.Identifier().unwrap().symbol.text.clone();
            let detail = self.compilation_details_from_context(ctx);
            self.current_chunk().push_goto_index(label.to_string(), index, detail);
        } else if ctx.End().is_some() {
            self.current_chunk().emit_op_code(OpCode::End, self.compilation_details_from_context(ctx));
        }
    }

    fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>) {
        self.visit_children(ctx);
        self.current_assignment_type_drop();
    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) {
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
        self.add_function_to_current_class(function);
        self.visit_children(ctx);
        // TODO: it won't work for all cases, to refactor
        self.current_declared_function().set_returned_type(self.current_assignment_type_drop());
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
            self.classes.push(ClassFile::new_with_main_function(name, self.file_name.clone(), detail.start_line));
        }
        self.state.current_declared_class += 1;
        self.visit_compoundStatement(ctx.compoundStatement().as_ref().unwrap());
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        let (variable, index) = Self::build_variable(ctx);
        self.add_current_assignment_type_from_variable(&variable);
        self.load_variable(&variable, index, ctx);
    }

    fn visit_scriptName(&mut self, ctx: &ScriptNameContext<'input>) {
        let mut name = String::from("");
        for child in ctx.get_children() {
            name = format!("{}{}", name, child.get_text());
        }
    }
}

fn parse_number(num: Cow<str>) -> i32 {
    let maybe_i32 = num.parse::<i32>();
    if maybe_i32.is_err() {
        panic!("Expected number to be u32, but was {}", num);
    }
    maybe_i32.unwrap()
}

fn remove_quote_from_string(string: &str) -> String {
    (&string[1..string.len() - 1]).to_string()
}