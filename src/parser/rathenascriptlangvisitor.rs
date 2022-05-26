#![allow(nonstandard_style)]
// Generated from RathenaScriptLang.g4 by ANTLR 4.9.3
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::rathenascriptlangparser::*;
use std::mem;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link RathenaScriptLangParser}.
 */
pub trait RathenaScriptLangVisitor<'input>: ParseTreeVisitor<'input,RathenaScriptLangParserContextType>{
	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#functionCallExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#argumentExpressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#unaryOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#castExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#multiplicativeOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeOperator(&mut self, ctx: &MultiplicativeOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#additiveOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveOperator(&mut self, ctx: &AdditiveOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#shiftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#shiftOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#relationalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#relationalOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalOperator(&mut self, ctx: &RelationalOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#equalityOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityOperator(&mut self, ctx: &EqualityOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#andExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#exclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#inclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#conditionalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#assignmentExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#assignmentLeftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentLeftExpression(&mut self, ctx: &AssignmentLeftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#assignmentOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#constantExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declaration}.
	 * @param ctx the parse tree
	 */
	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationSpecifiers(&mut self, ctx: &DeclarationSpecifiersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers2}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationSpecifiers2(&mut self, ctx: &DeclarationSpecifiers2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationSpecifier(&mut self, ctx: &DeclarationSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initDeclaratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#specifierQualifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_specifierQualifierList(&mut self, ctx: &SpecifierQualifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarator}.
	 * @param ctx the parse tree
	 */
	fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#directDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_directDeclarator(&mut self, ctx: &DirectDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#nestedParenthesesBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_nestedParenthesesBlock(&mut self, ctx: &NestedParenthesesBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#parameterTypeList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterTypeList(&mut self, ctx: &ParameterTypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#parameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#parameterDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#directAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_directAbstractDeclarator(&mut self, ctx: &DirectAbstractDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initializer}.
	 * @param ctx the parse tree
	 */
	fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initializerList}.
	 * @param ctx the parse tree
	 */
	fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#designation}.
	 * @param ctx the parse tree
	 */
	fn visit_designation(&mut self, ctx: &DesignationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#designatorList}.
	 * @param ctx the parse tree
	 */
	fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#designator}.
	 * @param ctx the parse tree
	 */
	fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#labeledStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#compoundStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#blockItemList}.
	 * @param ctx the parse tree
	 */
	fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#blockItem}.
	 * @param ctx the parse tree
	 */
	fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#expressionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#selectionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#iterationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_forCondition(&mut self, ctx: &ForConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#jumpStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_menuStatement(&mut self, ctx: &MenuStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuItem}.
	 * @param ctx the parse tree
	 */
	fn visit_menuItem(&mut self, ctx: &MenuItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#commandStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#dialogStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_dialogStatement(&mut self, ctx: &DialogStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#translationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#externalDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptInitialization}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scope_specifier}.
	 * @param ctx the parse tree
	 */
	fn visit_scope_specifier(&mut self, ctx: &Scope_specifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#variable_name}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_name(&mut self, ctx: &Variable_nameContext<'input>) { self.visit_children(ctx) }

}

pub trait RathenaScriptLangVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= RathenaScriptLangParserContextType>{
	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#functionCallExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#postfixExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#argumentExpressionList}.
	 * @param ctx the parse tree
	 */
		fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#unaryOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#castExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#multiplicativeOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeOperator(&mut self, ctx: &MultiplicativeOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#additiveOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveOperator(&mut self, ctx: &AdditiveOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#shiftExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#shiftOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#relationalExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#relationalOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_relationalOperator(&mut self, ctx: &RelationalOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#equalityExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#equalityOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityOperator(&mut self, ctx: &EqualityOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#andExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#exclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#inclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#conditionalExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#assignmentExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#assignmentLeftExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentLeftExpression(&mut self, ctx: &AssignmentLeftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#assignmentOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#constantExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declaration}.
	 * @param ctx the parse tree
	 */
		fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers}.
	 * @param ctx the parse tree
	 */
		fn visit_declarationSpecifiers(&mut self, ctx: &DeclarationSpecifiersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers2}.
	 * @param ctx the parse tree
	 */
		fn visit_declarationSpecifiers2(&mut self, ctx: &DeclarationSpecifiers2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_declarationSpecifier(&mut self, ctx: &DeclarationSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initDeclaratorList}.
	 * @param ctx the parse tree
	 */
		fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#specifierQualifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_specifierQualifierList(&mut self, ctx: &SpecifierQualifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#declarator}.
	 * @param ctx the parse tree
	 */
		fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#directDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_directDeclarator(&mut self, ctx: &DirectDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#nestedParenthesesBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_nestedParenthesesBlock(&mut self, ctx: &NestedParenthesesBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#parameterTypeList}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterTypeList(&mut self, ctx: &ParameterTypeListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#parameterList}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#parameterDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#identifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#directAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_directAbstractDeclarator(&mut self, ctx: &DirectAbstractDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initializer}.
	 * @param ctx the parse tree
	 */
		fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#initializerList}.
	 * @param ctx the parse tree
	 */
		fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#designation}.
	 * @param ctx the parse tree
	 */
		fn visit_designation(&mut self, ctx: &DesignationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#designatorList}.
	 * @param ctx the parse tree
	 */
		fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#designator}.
	 * @param ctx the parse tree
	 */
		fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#labeledStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#compoundStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#blockItemList}.
	 * @param ctx the parse tree
	 */
		fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#blockItem}.
	 * @param ctx the parse tree
	 */
		fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#expressionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#selectionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#iterationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_forCondition(&mut self, ctx: &ForConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#jumpStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_menuStatement(&mut self, ctx: &MenuStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuItem}.
	 * @param ctx the parse tree
	 */
		fn visit_menuItem(&mut self, ctx: &MenuItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#commandStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#dialogStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_dialogStatement(&mut self, ctx: &DialogStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#translationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#externalDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptInitialization}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scope_specifier}.
	 * @param ctx the parse tree
	 */
		fn visit_scope_specifier(&mut self, ctx: &Scope_specifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#variable_name}.
	 * @param ctx the parse tree
	 */
		fn visit_variable_name(&mut self, ctx: &Variable_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> RathenaScriptLangVisitor<'input> for T
where
	T: RathenaScriptLangVisitorCompat<'input>
{
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_compilationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_primaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_functionCallExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_postfixExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_argumentExpressionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_unaryOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_castExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_multiplicativeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeOperator(&mut self, ctx: &MultiplicativeOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_multiplicativeOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_additiveExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveOperator(&mut self, ctx: &AdditiveOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_additiveOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_shiftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_shiftOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_relationalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationalOperator(&mut self, ctx: &RelationalOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_relationalOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_equalityExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityOperator(&mut self, ctx: &EqualityOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_equalityOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_andExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_exclusiveOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_inclusiveOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_logicalAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_logicalOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_conditionalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_assignmentExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentLeftExpression(&mut self, ctx: &AssignmentLeftExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_assignmentLeftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_assignmentOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_constantExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_declaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarationSpecifiers(&mut self, ctx: &DeclarationSpecifiersContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_declarationSpecifiers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarationSpecifiers2(&mut self, ctx: &DeclarationSpecifiers2Context<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_declarationSpecifiers2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarationSpecifier(&mut self, ctx: &DeclarationSpecifierContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_declarationSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_initDeclaratorList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_initDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_specifierQualifierList(&mut self, ctx: &SpecifierQualifierListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_specifierQualifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_declarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_directDeclarator(&mut self, ctx: &DirectDeclaratorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_directDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nestedParenthesesBlock(&mut self, ctx: &NestedParenthesesBlockContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_nestedParenthesesBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterTypeList(&mut self, ctx: &ParameterTypeListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_parameterTypeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_parameterList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_parameterDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_identifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_directAbstractDeclarator(&mut self, ctx: &DirectAbstractDeclaratorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_directAbstractDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializer(&mut self, ctx: &InitializerContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_initializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_initializerList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_designation(&mut self, ctx: &DesignationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_designation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_designatorList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_designator(&mut self, ctx: &DesignatorContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_designator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_labeledStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_compoundStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_blockItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_blockItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_expressionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_selectionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_iterationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forCondition(&mut self, ctx: &ForConditionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_forCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_forDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_forExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_jumpStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_menuStatement(&mut self, ctx: &MenuStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_menuStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_menuItem(&mut self, ctx: &MenuItemContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_menuItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_commandStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dialogStatement(&mut self, ctx: &DialogStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_dialogStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_translationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_externalDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_functionDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptInitialization(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scope_specifier(&mut self, ctx: &Scope_specifierContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scope_specifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable_name(&mut self, ctx: &Variable_nameContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_variable_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}