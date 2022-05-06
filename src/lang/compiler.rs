use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Visitable, VisitChildren};
use crate::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;
use crate::{RathenaScriptLangLexer, RathenaScriptLangParser};
use crate::lang::chunk::{Chunk, OpCode, Value};
use crate::parser::rathenascriptlangparser::{*};

#[derive(Default)]
pub struct Compiler {
    chunks: Vec<Chunk>,
    current_chunk_id: Option<usize>,
}

pub enum State {
    Variable
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


impl Compiler {
    pub fn compile(&mut self, script: InputStream<&str>) -> &mut Vec<Chunk> {
        let lexer = RathenaScriptLangLexer::new(script);
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = RathenaScriptLangParser::new(token_stream);
        let tree = parser.compilationUnit();
        // println!("{}", tree.unwrap().to_string_tree(&parser));
        self.visit_compilationUnit(tree.as_ref().unwrap());

        &mut self.chunks
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunks[self.current_chunk_id.unwrap()]
    }
}


impl ParseTreeVisitor<'input, RathenaScriptLangParserContextType> for Compiler {}

impl RathenaScriptLangVisitor<'input> for Compiler {
    fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) {
        if ctx.String().is_some() {
            self.current_chunk().emit_op_code(OpCode::Constant);
            self.current_chunk().add_constant(Value::String(ctx.String().unwrap().symbol.text.deref().to_string()));
        }
        if ctx.Number().is_some() {
            self.current_chunk().emit_op_code(OpCode::Constant);
            let number_value = &ctx.Number().unwrap().symbol.text;
            self.current_chunk().add_constant(Value::Number(parse_number(number_value)))
        }
        if ctx.Identifier().is_some() {
            self.current_chunk().emit_op_code(OpCode::GetIdentifier);
            self.current_chunk().add_identifiers(ctx.Identifier().unwrap().symbol.text.deref().to_string());
        }
        if ctx.variable().is_some() {
            self.visit_variable(ctx.variable().as_ref().unwrap());
        }
        // self.visit_children(ctx)
    }

    fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) {
        self.visit_children(ctx);
        if ctx.argumentExpressionList().is_some() {
            self.current_chunk().emit_bytes(ctx.argumentExpressionList().unwrap().assignmentExpression_all().len() as u64)
        }
        self.current_chunk().emit_op_code(OpCode::Call);
    }

    fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>) {
        for expression in ctx.assignmentExpression_all().iter() {
            if expression.Number().is_some() {
                let number_value = &expression.Number().unwrap().symbol.text;
                self.current_chunk().add_constant(Value::Number(parse_number(number_value)));
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
        self.visit_children(ctx)
    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) {
        self.visit_children(ctx)
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
        self.visit_children(ctx)
    }

    fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) {
        self.visit_children(ctx)
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
        let parent = if self.current_chunk_id.is_none() {
            self.current_chunk_id = Some(0);
            None
        } else {
            let previous_id = self.current_chunk_id.unwrap();
            self.current_chunk_id = Some(previous_id + 1);
            Some(previous_id)
        };
        if self.chunks.get(self.current_chunk_id.unwrap()).is_none() {
            self.chunks.push(Chunk::new(self.current_chunk_id.unwrap(), parent));
        }
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
        self.visit_children(ctx)
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
        self.visit_children(ctx)
    }

    fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) {
        self.visit_children(ctx)
    }

    fn visit_scope_specifier(&mut self, ctx: &Scope_specifierContext<'input>) {
        let scope = if ctx.At().is_some() {
            VariableScope::Temporary
        } else if ctx.Dollar().is_some() {
            VariableScope::GlobalPermanent
        } else if ctx.DollarAt().is_some() {
            VariableScope::GlobalTemporary
        } else if ctx.Dot().is_some() {
            VariableScope::Npc
        } else if ctx.DotAt().is_some() {
            VariableScope::InstanceLocal
        } else if ctx.Quote().is_some() {
            VariableScope::Instance
        } else if ctx.Sharp().is_some() || ctx.DoubleSharp().is_some() {
            VariableScope::PermanentAccount
        } else {
            VariableScope::Default
        };
        self.visit_children(ctx)
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        if ctx.scope_specifier().is_some() {
            let scope_specifier = ctx.scope_specifier().unwrap();
            // if scope_specifier.At().is_some() {
            //     VariableScope::Temporary
            // } else if scope_specifier.Dollar().is_some() {
            //     VariableScope::GlobalPermanent
            // } else if scope_specifier.DollarAt().is_some() {
            //     VariableScope::GlobalTemporary
            // } else if scope_specifier.Dot().is_some() {
            //     VariableScope::Npc
            // } else if scope_specifier.DotAt().is_some() {
            //     VariableScope::InstanceLocal
            // } else if scope_specifier.Quote().is_some() {
            //     VariableScope::Instance
            // } else if scope_specifier.Sharp().is_some() || scope_specifier.DoubleSharp().is_some() {
            //     VariableScope::PermanentAccount
            // } else {
            //     VariableScope::Default
            // }
        } else {}
        println!("Visit variable {}", ctx.variable_name().unwrap().Identifier().unwrap().symbol.text);
    }

    fn visit_variable_name(&mut self, ctx: &Variable_nameContext<'input>) {
        // let var_type =if ctx.Dollar().is_some() {
        //
        // } else {
        //     VariableType::Integer
        // };
        self.visit_children(ctx)
    }

}

fn parse_number(num: &Cow<str>) -> u32 {
    let maybe_u32 = num.parse::<u32>();
    if maybe_u32.is_err() {
        panic!("Expected number to be u32, but was {}", num);
    }
    maybe_u32.unwrap()
}