use std::borrow::{BorrowMut, Cow};
use std::cell::RefCell;
use std::fmt::{Display, format, Formatter};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::{InputStream, TidExt};
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTreeVisitor};
use crate::parser::rathenascriptlangvisitor::{*};
use crate::parser::rathenascriptlanglexer::{*};
use crate::parser::rathenascriptlangparser::{*};
use crate::lang::vm::Vm;

use crate::lang::chunk::{Chunk, OpCode};
use crate::lang::chunk::OpCode::{*};
use crate::lang::compiler::CompilationErrorType::{FunctionAlreadyDefined, NativeAlreadyDefined, Type, UndefinedFunction};
use crate::lang::value::{*};

const NATIVE_METHODS: &'static [&'static str] = &[
    // Part of rathena script lang: implemented in VM.
    "getarg",
    // Part of rathena script lang: to be implemented in NativeMethodHandler
    // Not part of rathena script lang
    "print",
    // internal vm instrumentation
    "vm_dump_locals",
    "vm_dump_var"
];

pub struct Compiler {
    name: String,
    main_function: Function,
    errors: Vec<CompilationError>,
    state: State,
    script_lines: Vec<String>,
    // To check if called function exists. it can be done at the end of the compilation
    declared_functions: Vec<String>,
    called_functions: Vec<(String, CompilationErrorDetails)>,
    current_declared_function: Option<Function>,
}

#[derive(Default)]
pub struct State {
    current_assignment_types: Vec<ValueType>,
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct CompilationError {
    error_type: CompilationErrorType,
    message: String,
    file_name: String,
    details: CompilationErrorDetails,
}

#[derive(Debug, Clone)]
pub struct CompilationErrorDetails {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    text: String,
}

#[derive(Debug)]
pub enum CompilationErrorType {
    Generic,
    UndefinedVariable,
    UndefinedFunction,
    FunctionAlreadyDefined,
    NativeAlreadyDefined,
    Type,
}

impl CompilationError {
    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}:{}. {}", self.file_name, self.details.start_line, self.details.start_column, self.message).unwrap();
        writeln!(f, "l{}\t{}", self.details.start_line, self.details.text).unwrap();
        writeln!(f, "\t{}{}", " ".repeat(self.details.start_column), "^".repeat(self.details.end_column - self.details.start_column + 1))
    }
}

impl Compiler {
    fn new(name: String, script: String) -> Self {
        let main_function = Function::new(format!("{}_main", name));
        Self {
            name: name.clone(),
            main_function: main_function,
            errors: vec![],
            state: Default::default(),
            script_lines: script.split("\n").map(|l| l.to_string()).collect::<Vec<String>>(),
            declared_functions: vec![],
            called_functions: vec![],
            current_declared_function: None,
        }
    }
    pub fn compile(name: String, script: &str) -> Result<Function, Vec<CompilationError>> {
        let mut compiler = Compiler::new(name, script.to_string());
        let lexer = RathenaScriptLangLexer::new(InputStream::new(script));
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = RathenaScriptLangParser::new(token_stream);
        let tree = parser.compilationUnit();
        // println!("{}", tree.unwrap().to_string_tree(&parser));

        compiler.visit_compilationUnit(tree.as_ref().unwrap());
        for (function_name, compilationErrorDetails) in compiler.called_functions.clone().iter() {
            if !compiler.declared_functions.contains(function_name) {
                compiler.register_error_with_details(UndefinedFunction, compilationErrorDetails.clone(), format!("Function \"{}\" is not defined", function_name))
            }
        }
        if compiler.errors.is_empty() {
            Ok(compiler.main_function)
        } else {
            Err(compiler.errors)
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        if let Some(function) = self.current_declared_function.as_mut() {
            &mut function.chunk
        } else {
            &mut self.main_function.chunk
        }
    }

    fn variable_value(has_dollar: bool) -> ValueRef {
        if has_dollar { ValueRef::new_empty_string() } else { ValueRef::new_empty_number() }
    }

    fn register_error<'input>(&mut self, error_type: CompilationErrorType, node: &(dyn RathenaScriptLangParserContext<'input> + 'input), message: String) {
        let error = CompilationError {
            error_type,
            message,
            file_name: self.name.clone(),
            details: self.compilation_error_details_from_context(node),
        };
        self.errors.push(error);
    }

    fn register_error_with_details<'input>(&mut self, error_type: CompilationErrorType, details: CompilationErrorDetails, message: String) {
        let error = CompilationError {
            error_type,
            message,
            file_name: self.name.clone(),
            details,
        };
        self.errors.push(error);
    }

    fn compilation_error_details_from_context<'input>(&self, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) -> CompilationErrorDetails {
        CompilationErrorDetails {
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
        match var.value_ref.borrow().deref() {
            ValueRef::String(_) => self.add_current_assigment_type(ValueType::String),
            ValueRef::Number(_) => self.add_current_assigment_type(ValueType::Number),
        }
    }

    fn add_current_assigment_type(&mut self, value_type: ValueType) {
        self.state.current_assignment_types.push(value_type)
    }

    fn current_assignment_type_drop(&mut self) -> ValueType {
        let assignment_types = mem::replace(&mut self.state.current_assignment_types, Vec::<ValueType>::new());
        if assignment_types.iter().all(|v| v.is_number()) {
            ValueType::Number
        } else {
            ValueType::String
        }
    }

    fn current_assignment_type(&mut self) -> ValueType {
        if self.state.current_assignment_types.iter().all(|v| v.is_number()) {
            ValueType::Number
        } else {
            ValueType::String
        }
    }

    fn build_variable(ctx: &VariableContext) -> Variable {
        let scope = Self::get_variable_scope(ctx);
        let variable_name = ctx.variable_name().unwrap();
        let name = variable_name.Identifier().unwrap().symbol.text.deref().to_string();
        Variable {
            value_ref: RefCell::new(Self::variable_value(variable_name.Dollar().is_some())),
            name,
            scope,
        }
    }

    fn load_local<'input>(&mut self, variable: &Variable, node: &(dyn RathenaScriptLangParserContext<'input> + 'input)) {
        let maybe_local_variable = self.current_chunk().load_local(&variable);
        if let Ok(reference) = maybe_local_variable {
            self.current_chunk().emit_op_code(LoadLocal(reference));
        } else {
            self.register_error(
                CompilationErrorType::UndefinedVariable, node,
                format!("Variable \"{}\" is undefined.", variable.to_script_identifier()));
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
            let reference = self.current_chunk().add_constant(Constant::String(ctx.String().unwrap().symbol.text.deref().to_string()
                                                                                   .replace('\"', "") // TODO check if it can be done by antlr skip instead.
            ));
            self.add_current_assigment_type(ValueType::String);
            self.current_chunk().emit_op_code(LoadConstant(reference));
        }
        if ctx.Number().is_some() {
            let number_value = &ctx.Number().unwrap().symbol.text;
            let reference = self.current_chunk().add_constant(Constant::Number(parse_number(number_value.clone())));
            self.add_current_assigment_type(ValueType::Number);
            self.current_chunk().emit_op_code(LoadConstant(reference));
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
        let argument_count = if ctx.argumentExpressionList().is_some() {
            self.visit_argumentExpressionList(ctx.argumentExpressionList().as_ref().unwrap());
            ctx.argumentExpressionList().unwrap().assignmentExpression_all().len() as usize
        } else {
            0
        };
        let identifier = ctx.Identifier().unwrap();

        let function_or_native_name = &identifier.symbol.text;
        if NATIVE_METHODS.contains(&function_or_native_name.as_ref()) {
            self.current_chunk().emit_op_code(CallNative { reference: Vm::calculate_hash(&function_or_native_name), argument_count });
        } else {
            self.called_functions.push((String::from(function_or_native_name.clone()), self.compilation_error_details_from_context(ctx)));
            self.current_chunk().emit_op_code(CallFunction { reference: Vm::calculate_hash(&function_or_native_name), argument_count });
        }
    }

    fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) {
        self.visit_children(ctx)
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

    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) {
        self.visit_castExpression(&ctx.castExpression(ctx.castExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.Star_all().iter().enumerate().rev() {
            if i == ctx.castExpression_all().len() - 1 {
                continue;
            }
            self.visit_castExpression(&ctx.castExpression(i).unwrap());
            if self.current_assignment_type().is_string() {
                self.register_error(Type, ctx, "Multiply operator \"*\" is not allowed for String".to_string());
            }
            self.current_chunk().emit_op_code(Multiply);
        }

        for (i, _) in ctx.Percent_all().iter().enumerate().rev() {
            if i == ctx.castExpression_all().len() - 1 {
                continue;
            }
            self.visit_castExpression(&ctx.castExpression(i).unwrap());
            if self.current_assignment_type().is_string() {
                self.register_error(Type, ctx, "Modulo operator \"%\" is not allowed for String".to_string());
            }
            self.current_chunk().emit_op_code(Modulo);
        }

        for (i, _) in ctx.Slash_all().iter().enumerate().rev() {
            if i == ctx.castExpression_all().len() - 1 {
                continue;
            }
            self.visit_castExpression(&ctx.castExpression(i).unwrap());

            if self.current_assignment_type().is_string() {
                self.register_error(Type, ctx, "Divide operator \"/\" is not allowed for String".to_string());
            }
            self.current_chunk().emit_op_code(Divide);
        }
    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) {
        // self.visit_children(ctx);
        self.visit_multiplicativeExpression(&ctx.multiplicativeExpression(ctx.multiplicativeExpression_all().len() - 1).unwrap());
        for (i, _) in ctx.Plus_all().iter().enumerate().rev() {
            if i == ctx.multiplicativeExpression_all().len() - 1 {
                continue;
            }
            self.visit_multiplicativeExpression(&ctx.multiplicativeExpression(i).unwrap());
            self.current_chunk().emit_op_code(Add);
        }

        for (i, _) in ctx.Minus_all().iter().enumerate().rev() {
            if i == ctx.multiplicativeExpression_all().len() - 1 {
                continue;
            }
            self.visit_multiplicativeExpression(&ctx.multiplicativeExpression(i).unwrap());
            if self.current_assignment_type().is_string() {
                self.register_error(Type, ctx, "Subtraction operator \"-\" is not allowed for String".to_string());
            }
            self.current_chunk().emit_op_code(Subtract);
        }
    }

    fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) {
        let maybe_left = ctx.assignmentLeftExpression();
        if let Some(left) = maybe_left {
            let assignment_operator = &ctx.assignmentOperator().unwrap();
            self.visit_assignmentExpression(&ctx.assignmentExpression().unwrap());
            // Convert a += 1; into a = a + 1;
            if assignment_operator.PlusEqual().is_some() {
                if left.variable().is_some() {
                    let variable = Self::build_variable(&left.variable().unwrap());
                    self.load_local(&variable, ctx);
                }
                self.current_chunk().emit_op_code(Add);
            }
            self.visit_assignmentLeftExpression(&left);
        } else {
            self.visit_children(ctx);
        }
    }

    fn visit_assignmentLeftExpression(&mut self, ctx: &AssignmentLeftExpressionContext<'input>) {
        if ctx.Identifier().is_some() {
            // Number + char permanent variable (ie: not ending with '$', nor having any scope) match Identifier instead of variable.
            let name = ctx.Identifier().unwrap().symbol.text.deref().to_string();
            let reference = self.current_chunk().add_global(Variable {
                name,
                scope: Scope::Character,
                value_ref: RefCell::new(ValueRef::new_empty_number()),
            });
            self.current_chunk().emit_op_code(StoreGlobal(reference));
        } else if ctx.variable().is_some() {
            let variable_identifier = Self::build_variable(&ctx.variable().unwrap());
            let current_variable_type = self.current_assignment_type_drop();
            match variable_identifier.value_ref.borrow().deref() {
                ValueRef::String(_) => {
                    if current_variable_type.is_number() {
                        self.register_error(CompilationErrorType::Type, ctx,
                                            format!("Variable \"{}\" is a String but was assigned to a Number.", variable_identifier.to_script_identifier()));
                    }
                }
                ValueRef::Number(_) => {
                    if current_variable_type.is_string() {
                        self.register_error(CompilationErrorType::Type, ctx,
                                            format!("Variable \"{}\" is a Number but was assigned to a String.", variable_identifier.to_script_identifier()));
                    }
                }
            }
            match variable_identifier.scope {
                Scope::Server | Scope::Account | Scope::Character | Scope::Npc => {
                    let reference = self.current_chunk().add_global(variable_identifier);
                    self.current_chunk().emit_op_code(StoreGlobal(reference));
                }
                Scope::Local => {
                    let reference = self.current_chunk().add_local(variable_identifier);
                    self.current_chunk().emit_op_code(StoreLocal(reference));
                }
                Scope::Instance => {
                    let reference = self.current_chunk().add_instance(variable_identifier);
                    self.current_chunk().emit_op_code(StoreInstance(reference));
                }
            }
        }
    }


    fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) {
        self.visit_children(ctx);
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_declarationSpecifiers(&mut self, ctx: &DeclarationSpecifiersContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_declarationSpecifiers2(&mut self, ctx: &DeclarationSpecifiers2Context<'input>) {
        self.visit_children(ctx)
    }

    fn visit_declarationSpecifier(&mut self, ctx: &DeclarationSpecifierContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>) {
        self.visit_children(ctx)
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

    fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_directDeclarator(&mut self, ctx: &DirectDeclaratorContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_nestedParenthesesBlock(&mut self, ctx: &NestedParenthesesBlockContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_parameterTypeList(&mut self, ctx: &ParameterTypeListContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_directAbstractDeclarator(&mut self, ctx: &DirectAbstractDeclaratorContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_designation(&mut self, ctx: &DesignationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>) {
        // TODO push nested function to current function,
        self.visit_children(ctx)
    }

    fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_forCondition(&mut self, ctx: &ForConditionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) {
        self.visit_children(ctx);
        if ctx.Return().is_some() {
            self.current_chunk().emit_op_code(OpCode::Return);
        }
    }

    fn visit_menuStatement(&mut self, ctx: &MenuStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_menuItem(&mut self, ctx: &MenuItemContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_dialogStatement(&mut self, ctx: &DialogStatementContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) {
        let function_name = &ctx.Identifier().unwrap().symbol.text;
        let function_name = function_name.clone().to_string();
        if self.declared_functions.contains(&function_name) {
            self.register_error(FunctionAlreadyDefined, ctx, format!("A function with name \"{}\" already exists.", function_name));
            return;
        }
        if NATIVE_METHODS.contains(&function_name.as_str()) {
            self.register_error(NativeAlreadyDefined, ctx, format!("A native function with name \"{}\" already exists.", function_name));
            return;
        }
        let function = Function {
            name: function_name.clone(),
            arity: 0,
            chunk: Default::default()
        };
        self.declared_functions.push(function_name);
        self.current_declared_function = Some(function);
        self.visit_children(ctx);
        let current_declared_function = mem::replace(&mut self.current_declared_function, None);
        self.current_chunk().add_function(current_declared_function.unwrap());
    }

    fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_scope_specifier(&mut self, ctx: &Scope_specifierContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        let variable = Self::build_variable(ctx);
        self.add_current_assignment_type_from_variable(&variable);
        self.load_local(&variable, ctx);
    }

    fn visit_variable_name(&mut self, ctx: &Variable_nameContext<'input>) {
        self.visit_children(ctx)
    }
}

fn parse_number(num: Cow<str>) -> i32 {
    let maybe_i32 = num.parse::<i32>();
    if maybe_i32.is_err() {
        panic!("Expected number to be u32, but was {}", num);
    }
    maybe_i32.unwrap()
}