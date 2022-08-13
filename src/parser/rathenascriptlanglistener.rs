#![allow(nonstandard_style)]
// Generated from RathenaScriptLang.g4 by ANTLR 4.9.3
use antlr_rust::tree::ParseTreeListener;
use super::rathenascriptlangparser::*;

pub trait RathenaScriptLangListener<'input> : ParseTreeListener<'input,RathenaScriptLangParserContextType>{
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#functionCallExpression}.
 * @param ctx the parse tree
 */
fn enter_functionCallExpression(&mut self, _ctx: &FunctionCallExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#functionCallExpression}.
 * @param ctx the parse tree
 */
fn exit_functionCallExpression(&mut self, _ctx: &FunctionCallExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#postfixExpression}.
 * @param ctx the parse tree
 */
fn enter_postfixExpression(&mut self, _ctx: &PostfixExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#postfixExpression}.
 * @param ctx the parse tree
 */
fn exit_postfixExpression(&mut self, _ctx: &PostfixExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#incrementThenLoad}.
 * @param ctx the parse tree
 */
fn enter_incrementThenLoad(&mut self, _ctx: &IncrementThenLoadContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#incrementThenLoad}.
 * @param ctx the parse tree
 */
fn exit_incrementThenLoad(&mut self, _ctx: &IncrementThenLoadContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#loadThenIncrement}.
 * @param ctx the parse tree
 */
fn enter_loadThenIncrement(&mut self, _ctx: &LoadThenIncrementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#loadThenIncrement}.
 * @param ctx the parse tree
 */
fn exit_loadThenIncrement(&mut self, _ctx: &LoadThenIncrementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#argumentExpressionList}.
 * @param ctx the parse tree
 */
fn enter_argumentExpressionList(&mut self, _ctx: &ArgumentExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#argumentExpressionList}.
 * @param ctx the parse tree
 */
fn exit_argumentExpressionList(&mut self, _ctx: &ArgumentExpressionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#unaryOperator}.
 * @param ctx the parse tree
 */
fn enter_unaryOperator(&mut self, _ctx: &UnaryOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#unaryOperator}.
 * @param ctx the parse tree
 */
fn exit_unaryOperator(&mut self, _ctx: &UnaryOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#multiplicativeOperator}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeOperator(&mut self, _ctx: &MultiplicativeOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#multiplicativeOperator}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeOperator(&mut self, _ctx: &MultiplicativeOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#additiveOperator}.
 * @param ctx the parse tree
 */
fn enter_additiveOperator(&mut self, _ctx: &AdditiveOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#additiveOperator}.
 * @param ctx the parse tree
 */
fn exit_additiveOperator(&mut self, _ctx: &AdditiveOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#shiftExpression}.
 * @param ctx the parse tree
 */
fn enter_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#shiftExpression}.
 * @param ctx the parse tree
 */
fn exit_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#shiftOperator}.
 * @param ctx the parse tree
 */
fn enter_shiftOperator(&mut self, _ctx: &ShiftOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#shiftOperator}.
 * @param ctx the parse tree
 */
fn exit_shiftOperator(&mut self, _ctx: &ShiftOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#relationalExpression}.
 * @param ctx the parse tree
 */
fn enter_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#relationalExpression}.
 * @param ctx the parse tree
 */
fn exit_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#relationalOperator}.
 * @param ctx the parse tree
 */
fn enter_relationalOperator(&mut self, _ctx: &RelationalOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#relationalOperator}.
 * @param ctx the parse tree
 */
fn exit_relationalOperator(&mut self, _ctx: &RelationalOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#equalityExpression}.
 * @param ctx the parse tree
 */
fn enter_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#equalityExpression}.
 * @param ctx the parse tree
 */
fn exit_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#equalityOperator}.
 * @param ctx the parse tree
 */
fn enter_equalityOperator(&mut self, _ctx: &EqualityOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#equalityOperator}.
 * @param ctx the parse tree
 */
fn exit_equalityOperator(&mut self, _ctx: &EqualityOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#andExpression}.
 * @param ctx the parse tree
 */
fn enter_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#andExpression}.
 * @param ctx the parse tree
 */
fn exit_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#exclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn enter_exclusiveOrExpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#exclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn exit_exclusiveOrExpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#inclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn enter_inclusiveOrExpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#inclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn exit_inclusiveOrExpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#conditionalExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalExpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#conditionalExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalExpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#assignmentExpression}.
 * @param ctx the parse tree
 */
fn enter_assignmentExpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#assignmentExpression}.
 * @param ctx the parse tree
 */
fn exit_assignmentExpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#assignmentLeftExpression}.
 * @param ctx the parse tree
 */
fn enter_assignmentLeftExpression(&mut self, _ctx: &AssignmentLeftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#assignmentLeftExpression}.
 * @param ctx the parse tree
 */
fn exit_assignmentLeftExpression(&mut self, _ctx: &AssignmentLeftExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#assignmentOperator}.
 * @param ctx the parse tree
 */
fn enter_assignmentOperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#assignmentOperator}.
 * @param ctx the parse tree
 */
fn exit_assignmentOperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#constantExpression}.
 * @param ctx the parse tree
 */
fn enter_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#constantExpression}.
 * @param ctx the parse tree
 */
fn exit_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#commandStatement}.
 * @param ctx the parse tree
 */
fn enter_commandStatement(&mut self, _ctx: &CommandStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#commandStatement}.
 * @param ctx the parse tree
 */
fn exit_commandStatement(&mut self, _ctx: &CommandStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#menuOptionText}.
 * @param ctx the parse tree
 */
fn enter_menuOptionText(&mut self, _ctx: &MenuOptionTextContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#menuOptionText}.
 * @param ctx the parse tree
 */
fn exit_menuOptionText(&mut self, _ctx: &MenuOptionTextContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#menuLabel}.
 * @param ctx the parse tree
 */
fn enter_menuLabel(&mut self, _ctx: &MenuLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#menuLabel}.
 * @param ctx the parse tree
 */
fn exit_menuLabel(&mut self, _ctx: &MenuLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#labeledStatement}.
 * @param ctx the parse tree
 */
fn enter_labeledStatement(&mut self, _ctx: &LabeledStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#labeledStatement}.
 * @param ctx the parse tree
 */
fn exit_labeledStatement(&mut self, _ctx: &LabeledStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#compoundStatement}.
 * @param ctx the parse tree
 */
fn enter_compoundStatement(&mut self, _ctx: &CompoundStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#compoundStatement}.
 * @param ctx the parse tree
 */
fn exit_compoundStatement(&mut self, _ctx: &CompoundStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#blockItemList}.
 * @param ctx the parse tree
 */
fn enter_blockItemList(&mut self, _ctx: &BlockItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#blockItemList}.
 * @param ctx the parse tree
 */
fn exit_blockItemList(&mut self, _ctx: &BlockItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#blockItem}.
 * @param ctx the parse tree
 */
fn enter_blockItem(&mut self, _ctx: &BlockItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#blockItem}.
 * @param ctx the parse tree
 */
fn exit_blockItem(&mut self, _ctx: &BlockItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#expressionStatement}.
 * @param ctx the parse tree
 */
fn enter_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#expressionStatement}.
 * @param ctx the parse tree
 */
fn exit_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#selectionStatement}.
 * @param ctx the parse tree
 */
fn enter_selectionStatement(&mut self, _ctx: &SelectionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#selectionStatement}.
 * @param ctx the parse tree
 */
fn exit_selectionStatement(&mut self, _ctx: &SelectionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#switchStatement}.
 * @param ctx the parse tree
 */
fn enter_switchStatement(&mut self, _ctx: &SwitchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#switchStatement}.
 * @param ctx the parse tree
 */
fn exit_switchStatement(&mut self, _ctx: &SwitchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#switchBlock}.
 * @param ctx the parse tree
 */
fn enter_switchBlock(&mut self, _ctx: &SwitchBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#switchBlock}.
 * @param ctx the parse tree
 */
fn exit_switchBlock(&mut self, _ctx: &SwitchBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#switchBlockStatementGroup}.
 * @param ctx the parse tree
 */
fn enter_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#switchBlockStatementGroup}.
 * @param ctx the parse tree
 */
fn exit_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#switchLabels}.
 * @param ctx the parse tree
 */
fn enter_switchLabels(&mut self, _ctx: &SwitchLabelsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#switchLabels}.
 * @param ctx the parse tree
 */
fn exit_switchLabels(&mut self, _ctx: &SwitchLabelsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#switchLabel}.
 * @param ctx the parse tree
 */
fn enter_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#switchLabel}.
 * @param ctx the parse tree
 */
fn exit_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#iterationStatement}.
 * @param ctx the parse tree
 */
fn enter_iterationStatement(&mut self, _ctx: &IterationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#iterationStatement}.
 * @param ctx the parse tree
 */
fn exit_iterationStatement(&mut self, _ctx: &IterationStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#forCondition}.
 * @param ctx the parse tree
 */
fn enter_forCondition(&mut self, _ctx: &ForConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#forCondition}.
 * @param ctx the parse tree
 */
fn exit_forCondition(&mut self, _ctx: &ForConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#forDeclaration}.
 * @param ctx the parse tree
 */
fn enter_forDeclaration(&mut self, _ctx: &ForDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#forDeclaration}.
 * @param ctx the parse tree
 */
fn exit_forDeclaration(&mut self, _ctx: &ForDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#forStopExpression}.
 * @param ctx the parse tree
 */
fn enter_forStopExpression(&mut self, _ctx: &ForStopExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#forStopExpression}.
 * @param ctx the parse tree
 */
fn exit_forStopExpression(&mut self, _ctx: &ForStopExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#forExpression}.
 * @param ctx the parse tree
 */
fn enter_forExpression(&mut self, _ctx: &ForExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#forExpression}.
 * @param ctx the parse tree
 */
fn exit_forExpression(&mut self, _ctx: &ForExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#jumpStatement}.
 * @param ctx the parse tree
 */
fn enter_jumpStatement(&mut self, _ctx: &JumpStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#jumpStatement}.
 * @param ctx the parse tree
 */
fn exit_jumpStatement(&mut self, _ctx: &JumpStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#translationUnit}.
 * @param ctx the parse tree
 */
fn enter_translationUnit(&mut self, _ctx: &TranslationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#translationUnit}.
 * @param ctx the parse tree
 */
fn exit_translationUnit(&mut self, _ctx: &TranslationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#externalDeclaration}.
 * @param ctx the parse tree
 */
fn enter_externalDeclaration(&mut self, _ctx: &ExternalDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#externalDeclaration}.
 * @param ctx the parse tree
 */
fn exit_externalDeclaration(&mut self, _ctx: &ExternalDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn enter_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn exit_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptInitialization}.
 * @param ctx the parse tree
 */
fn enter_scriptInitialization(&mut self, _ctx: &ScriptInitializationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptInitialization}.
 * @param ctx the parse tree
 */
fn exit_scriptInitialization(&mut self, _ctx: &ScriptInitializationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptLocation}.
 * @param ctx the parse tree
 */
fn enter_scriptLocation(&mut self, _ctx: &ScriptLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptLocation}.
 * @param ctx the parse tree
 */
fn exit_scriptLocation(&mut self, _ctx: &ScriptLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptXPos}.
 * @param ctx the parse tree
 */
fn enter_scriptXPos(&mut self, _ctx: &ScriptXPosContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptXPos}.
 * @param ctx the parse tree
 */
fn exit_scriptXPos(&mut self, _ctx: &ScriptXPosContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptYPos}.
 * @param ctx the parse tree
 */
fn enter_scriptYPos(&mut self, _ctx: &ScriptYPosContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptYPos}.
 * @param ctx the parse tree
 */
fn exit_scriptYPos(&mut self, _ctx: &ScriptYPosContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptDir}.
 * @param ctx the parse tree
 */
fn enter_scriptDir(&mut self, _ctx: &ScriptDirContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptDir}.
 * @param ctx the parse tree
 */
fn exit_scriptDir(&mut self, _ctx: &ScriptDirContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptSprite}.
 * @param ctx the parse tree
 */
fn enter_scriptSprite(&mut self, _ctx: &ScriptSpriteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptSprite}.
 * @param ctx the parse tree
 */
fn exit_scriptSprite(&mut self, _ctx: &ScriptSpriteContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#npcInitialization}.
 * @param ctx the parse tree
 */
fn enter_npcInitialization(&mut self, _ctx: &NpcInitializationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#npcInitialization}.
 * @param ctx the parse tree
 */
fn exit_npcInitialization(&mut self, _ctx: &NpcInitializationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scriptName}.
 * @param ctx the parse tree
 */
fn enter_scriptName(&mut self, _ctx: &ScriptNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scriptName}.
 * @param ctx the parse tree
 */
fn exit_scriptName(&mut self, _ctx: &ScriptNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#npcShopDiscount}.
 * @param ctx the parse tree
 */
fn enter_npcShopDiscount(&mut self, _ctx: &NpcShopDiscountContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#npcShopDiscount}.
 * @param ctx the parse tree
 */
fn exit_npcShopDiscount(&mut self, _ctx: &NpcShopDiscountContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#npcShopItem}.
 * @param ctx the parse tree
 */
fn enter_npcShopItem(&mut self, _ctx: &NpcShopItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#npcShopItem}.
 * @param ctx the parse tree
 */
fn exit_npcShopItem(&mut self, _ctx: &NpcShopItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#npcShopPrice}.
 * @param ctx the parse tree
 */
fn enter_npcShopPrice(&mut self, _ctx: &NpcShopPriceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#npcShopPrice}.
 * @param ctx the parse tree
 */
fn exit_npcShopPrice(&mut self, _ctx: &NpcShopPriceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#scope_specifier}.
 * @param ctx the parse tree
 */
fn enter_scope_specifier(&mut self, _ctx: &Scope_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#scope_specifier}.
 * @param ctx the parse tree
 */
fn exit_scope_specifier(&mut self, _ctx: &Scope_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#variable_name}.
 * @param ctx the parse tree
 */
fn enter_variable_name(&mut self, _ctx: &Variable_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#variable_name}.
 * @param ctx the parse tree
 */
fn exit_variable_name(&mut self, _ctx: &Variable_nameContext<'input>) { }

}
