#![allow(nonstandard_style)]
// Generated from RathenaScriptLang.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::rathenascriptlangparser::*;

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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#shiftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#relationalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

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