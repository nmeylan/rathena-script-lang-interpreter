#![allow(nonstandard_style)]
// Generated from RathenaScriptLang.g4 by ANTLR 4.8
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
 * Enter a parse tree produced by {@link RathenaScriptLangParser#castExpression}.
 * @param ctx the parse tree
 */
fn enter_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#castExpression}.
 * @param ctx the parse tree
 */
fn exit_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }

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
 * Enter a parse tree produced by {@link RathenaScriptLangParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }

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
 * Enter a parse tree produced by {@link RathenaScriptLangParser#declaration}.
 * @param ctx the parse tree
 */
fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#declaration}.
 * @param ctx the parse tree
 */
fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers}.
 * @param ctx the parse tree
 */
fn enter_declarationSpecifiers(&mut self, _ctx: &DeclarationSpecifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers}.
 * @param ctx the parse tree
 */
fn exit_declarationSpecifiers(&mut self, _ctx: &DeclarationSpecifiersContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers2}.
 * @param ctx the parse tree
 */
fn enter_declarationSpecifiers2(&mut self, _ctx: &DeclarationSpecifiers2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifiers2}.
 * @param ctx the parse tree
 */
fn exit_declarationSpecifiers2(&mut self, _ctx: &DeclarationSpecifiers2Context<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifier}.
 * @param ctx the parse tree
 */
fn enter_declarationSpecifier(&mut self, _ctx: &DeclarationSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#declarationSpecifier}.
 * @param ctx the parse tree
 */
fn exit_declarationSpecifier(&mut self, _ctx: &DeclarationSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#initDeclaratorList}.
 * @param ctx the parse tree
 */
fn enter_initDeclaratorList(&mut self, _ctx: &InitDeclaratorListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#initDeclaratorList}.
 * @param ctx the parse tree
 */
fn exit_initDeclaratorList(&mut self, _ctx: &InitDeclaratorListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#initDeclarator}.
 * @param ctx the parse tree
 */
fn enter_initDeclarator(&mut self, _ctx: &InitDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#initDeclarator}.
 * @param ctx the parse tree
 */
fn exit_initDeclarator(&mut self, _ctx: &InitDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#specifierQualifierList}.
 * @param ctx the parse tree
 */
fn enter_specifierQualifierList(&mut self, _ctx: &SpecifierQualifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#specifierQualifierList}.
 * @param ctx the parse tree
 */
fn exit_specifierQualifierList(&mut self, _ctx: &SpecifierQualifierListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#declarator}.
 * @param ctx the parse tree
 */
fn enter_declarator(&mut self, _ctx: &DeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#declarator}.
 * @param ctx the parse tree
 */
fn exit_declarator(&mut self, _ctx: &DeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#directDeclarator}.
 * @param ctx the parse tree
 */
fn enter_directDeclarator(&mut self, _ctx: &DirectDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#directDeclarator}.
 * @param ctx the parse tree
 */
fn exit_directDeclarator(&mut self, _ctx: &DirectDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#nestedParenthesesBlock}.
 * @param ctx the parse tree
 */
fn enter_nestedParenthesesBlock(&mut self, _ctx: &NestedParenthesesBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#nestedParenthesesBlock}.
 * @param ctx the parse tree
 */
fn exit_nestedParenthesesBlock(&mut self, _ctx: &NestedParenthesesBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#parameterTypeList}.
 * @param ctx the parse tree
 */
fn enter_parameterTypeList(&mut self, _ctx: &ParameterTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#parameterTypeList}.
 * @param ctx the parse tree
 */
fn exit_parameterTypeList(&mut self, _ctx: &ParameterTypeListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#parameterList}.
 * @param ctx the parse tree
 */
fn enter_parameterList(&mut self, _ctx: &ParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#parameterList}.
 * @param ctx the parse tree
 */
fn exit_parameterList(&mut self, _ctx: &ParameterListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#parameterDeclaration}.
 * @param ctx the parse tree
 */
fn enter_parameterDeclaration(&mut self, _ctx: &ParameterDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#parameterDeclaration}.
 * @param ctx the parse tree
 */
fn exit_parameterDeclaration(&mut self, _ctx: &ParameterDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#directAbstractDeclarator}.
 * @param ctx the parse tree
 */
fn enter_directAbstractDeclarator(&mut self, _ctx: &DirectAbstractDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#directAbstractDeclarator}.
 * @param ctx the parse tree
 */
fn exit_directAbstractDeclarator(&mut self, _ctx: &DirectAbstractDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#initializer}.
 * @param ctx the parse tree
 */
fn enter_initializer(&mut self, _ctx: &InitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#initializer}.
 * @param ctx the parse tree
 */
fn exit_initializer(&mut self, _ctx: &InitializerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#initializerList}.
 * @param ctx the parse tree
 */
fn enter_initializerList(&mut self, _ctx: &InitializerListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#initializerList}.
 * @param ctx the parse tree
 */
fn exit_initializerList(&mut self, _ctx: &InitializerListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#designation}.
 * @param ctx the parse tree
 */
fn enter_designation(&mut self, _ctx: &DesignationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#designation}.
 * @param ctx the parse tree
 */
fn exit_designation(&mut self, _ctx: &DesignationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#designatorList}.
 * @param ctx the parse tree
 */
fn enter_designatorList(&mut self, _ctx: &DesignatorListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#designatorList}.
 * @param ctx the parse tree
 */
fn exit_designatorList(&mut self, _ctx: &DesignatorListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#designator}.
 * @param ctx the parse tree
 */
fn enter_designator(&mut self, _ctx: &DesignatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#designator}.
 * @param ctx the parse tree
 */
fn exit_designator(&mut self, _ctx: &DesignatorContext<'input>) { }

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
 * Enter a parse tree produced by {@link RathenaScriptLangParser#menuStatement}.
 * @param ctx the parse tree
 */
fn enter_menuStatement(&mut self, _ctx: &MenuStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#menuStatement}.
 * @param ctx the parse tree
 */
fn exit_menuStatement(&mut self, _ctx: &MenuStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link RathenaScriptLangParser#menuItem}.
 * @param ctx the parse tree
 */
fn enter_menuItem(&mut self, _ctx: &MenuItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#menuItem}.
 * @param ctx the parse tree
 */
fn exit_menuItem(&mut self, _ctx: &MenuItemContext<'input>) { }

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
 * Enter a parse tree produced by {@link RathenaScriptLangParser#dialogStatement}.
 * @param ctx the parse tree
 */
fn enter_dialogStatement(&mut self, _ctx: &DialogStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RathenaScriptLangParser#dialogStatement}.
 * @param ctx the parse tree
 */
fn exit_dialogStatement(&mut self, _ctx: &DialogStatementContext<'input>) { }

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
