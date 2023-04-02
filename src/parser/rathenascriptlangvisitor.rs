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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#functionCallExpressionWithoutParentheses}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCallExpressionWithoutParentheses(&mut self, ctx: &FunctionCallExpressionWithoutParenthesesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#incrementThenLoad}.
	 * @param ctx the parse tree
	 */
	fn visit_incrementThenLoad(&mut self, ctx: &IncrementThenLoadContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#loadThenIncrement}.
	 * @param ctx the parse tree
	 */
	fn visit_loadThenIncrement(&mut self, ctx: &LoadThenIncrementContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#constantExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#commandStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuOptionText}.
	 * @param ctx the parse tree
	 */
	fn visit_menuOptionText(&mut self, ctx: &MenuOptionTextContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_menuLabel(&mut self, ctx: &MenuLabelContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_switchBlock(&mut self, ctx: &SwitchBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchBlockStatementGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchLabels}.
	 * @param ctx the parse tree
	 */
	fn visit_switchLabels(&mut self, ctx: &SwitchLabelsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forStopExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_forStopExpression(&mut self, ctx: &ForStopExpressionContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptLocation}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptLocation(&mut self, ctx: &ScriptLocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptXPos}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptXPos(&mut self, ctx: &ScriptXPosContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptYPos}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptYPos(&mut self, ctx: &ScriptYPosContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptDir}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptDir(&mut self, ctx: &ScriptDirContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptSprite}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptSprite(&mut self, ctx: &ScriptSpriteContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcInitialization}.
	 * @param ctx the parse tree
	 */
	fn visit_npcInitialization(&mut self, ctx: &NpcInitializationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptName}.
	 * @param ctx the parse tree
	 */
	fn visit_scriptName(&mut self, ctx: &ScriptNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcShopDiscount}.
	 * @param ctx the parse tree
	 */
	fn visit_npcShopDiscount(&mut self, ctx: &NpcShopDiscountContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcShopItem}.
	 * @param ctx the parse tree
	 */
	fn visit_npcShopItem(&mut self, ctx: &NpcShopItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcShopPrice}.
	 * @param ctx the parse tree
	 */
	fn visit_npcShopPrice(&mut self, ctx: &NpcShopPriceContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#functionCallExpressionWithoutParentheses}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCallExpressionWithoutParentheses(&mut self, ctx: &FunctionCallExpressionWithoutParenthesesContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#incrementThenLoad}.
	 * @param ctx the parse tree
	 */
		fn visit_incrementThenLoad(&mut self, ctx: &IncrementThenLoadContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#loadThenIncrement}.
	 * @param ctx the parse tree
	 */
		fn visit_loadThenIncrement(&mut self, ctx: &LoadThenIncrementContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#constantExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#commandStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuOptionText}.
	 * @param ctx the parse tree
	 */
		fn visit_menuOptionText(&mut self, ctx: &MenuOptionTextContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#menuLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_menuLabel(&mut self, ctx: &MenuLabelContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_switchBlock(&mut self, ctx: &SwitchBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchBlockStatementGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchLabels}.
	 * @param ctx the parse tree
	 */
		fn visit_switchLabels(&mut self, ctx: &SwitchLabelsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#switchLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#forStopExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_forStopExpression(&mut self, ctx: &ForStopExpressionContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptLocation}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptLocation(&mut self, ctx: &ScriptLocationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptXPos}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptXPos(&mut self, ctx: &ScriptXPosContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptYPos}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptYPos(&mut self, ctx: &ScriptYPosContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptDir}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptDir(&mut self, ctx: &ScriptDirContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptSprite}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptSprite(&mut self, ctx: &ScriptSpriteContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcInitialization}.
	 * @param ctx the parse tree
	 */
		fn visit_npcInitialization(&mut self, ctx: &NpcInitializationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#scriptName}.
	 * @param ctx the parse tree
	 */
		fn visit_scriptName(&mut self, ctx: &ScriptNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcShopDiscount}.
	 * @param ctx the parse tree
	 */
		fn visit_npcShopDiscount(&mut self, ctx: &NpcShopDiscountContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcShopItem}.
	 * @param ctx the parse tree
	 */
		fn visit_npcShopItem(&mut self, ctx: &NpcShopItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RathenaScriptLangParser#npcShopPrice}.
	 * @param ctx the parse tree
	 */
		fn visit_npcShopPrice(&mut self, ctx: &NpcShopPriceContext<'input>) -> Self::Return {
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

	fn visit_functionCallExpressionWithoutParentheses(&mut self, ctx: &FunctionCallExpressionWithoutParenthesesContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_functionCallExpressionWithoutParentheses(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_postfixExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_incrementThenLoad(&mut self, ctx: &IncrementThenLoadContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_incrementThenLoad(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_loadThenIncrement(&mut self, ctx: &LoadThenIncrementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_loadThenIncrement(self, ctx);
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

	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_constantExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commandStatement(&mut self, ctx: &CommandStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_commandStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_menuOptionText(&mut self, ctx: &MenuOptionTextContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_menuOptionText(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_menuLabel(&mut self, ctx: &MenuLabelContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_menuLabel(self, ctx);
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

	fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_switchStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchBlock(&mut self, ctx: &SwitchBlockContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_switchBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_switchBlockStatementGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchLabels(&mut self, ctx: &SwitchLabelsContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_switchLabels(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_switchLabel(self, ctx);
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

	fn visit_forStopExpression(&mut self, ctx: &ForStopExpressionContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_forStopExpression(self, ctx);
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

	fn visit_scriptLocation(&mut self, ctx: &ScriptLocationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptLocation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scriptXPos(&mut self, ctx: &ScriptXPosContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptXPos(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scriptYPos(&mut self, ctx: &ScriptYPosContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptYPos(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scriptDir(&mut self, ctx: &ScriptDirContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptDir(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scriptSprite(&mut self, ctx: &ScriptSpriteContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptSprite(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_npcInitialization(&mut self, ctx: &NpcInitializationContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_npcInitialization(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scriptName(&mut self, ctx: &ScriptNameContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_scriptName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_npcShopDiscount(&mut self, ctx: &NpcShopDiscountContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_npcShopDiscount(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_npcShopItem(&mut self, ctx: &NpcShopItemContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_npcShopItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_npcShopPrice(&mut self, ctx: &NpcShopPriceContext<'input>){
		let result = <Self as RathenaScriptLangVisitorCompat>::visit_npcShopPrice(self, ctx);
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