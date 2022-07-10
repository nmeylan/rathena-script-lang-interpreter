grammar RathenaScriptLang;

compilationUnit
    :   translationUnit EOF
    ;

primaryExpression
    : Identifier
    | variable
    |   Number
    |   String
    | '(' expression ')'
    |   '-' // it is a special arguments for command, lets see if it can cause weird parse issue
    ;

functionCallExpression
    : Identifier '('? argumentExpressionList? ')'?
    | 'callfunc' '('? String ( ',' argumentExpressionList)? ')'
    | 'callfunc' String ( ',' argumentExpressionList)?
    ;

postfixExpression
    : functionCallExpression | primaryExpression ('[' expression ']' | ('++' | '--') )*
    ;

argumentExpressionList
    :   assignmentExpression (',' assignmentExpression)*
    ;

unaryExpression
    :
    ('++' |  '--')*
    (postfixExpression
    |   unaryOperator castExpression
    )
    ;

unaryOperator
    :   '&' | '*' | '+' | '-' | '~' | '!'
    ;

castExpression
    :   unaryExpression
    |   Number // for
    ;

multiplicativeExpression
    :   castExpression (multiplicativeOperator castExpression)*
    ;

multiplicativeOperator
    : ('*'|'/'|'%');

additiveExpression
    :   multiplicativeExpression (additiveOperator multiplicativeExpression)*
    ;

additiveOperator
    : (Plus | Minus);

shiftExpression
    :   additiveExpression (shiftOperator additiveExpression)*
    ;

shiftOperator
    : ('<<'|'>>');

relationalExpression
    :   shiftExpression (relationalOperator shiftExpression)*
    ;
relationalOperator
    : ('<'|'>'|'<='|'>=');

equalityExpression
    :   relationalExpression (equalityOperator relationalExpression)*
    ;
equalityOperator
    : ('=='| '!=');

andExpression
    :   equalityExpression ( '&' equalityExpression)*
    ;

exclusiveOrExpression
    :   andExpression ('^' andExpression)*
    ;

inclusiveOrExpression
    :   exclusiveOrExpression ('|' exclusiveOrExpression)*
    ;

logicalAndExpression
    :   inclusiveOrExpression ('&&' inclusiveOrExpression)*
    ;

logicalOrExpression
    :   logicalAndExpression ( '||' logicalAndExpression)*
    ;

conditionalExpression
    :   logicalOrExpression ('?' expression ':' conditionalExpression)?
    ;

assignmentExpression
    :   conditionalExpression
    |   assignmentLeftExpression assignmentOperator assignmentExpression
    |   'set' '('? functionCallExpression ',' assignmentExpression ')'? // only set(getd()) will be allowed by compiler, not any function call
    |   'set' '('? assignmentLeftExpression ',' assignmentExpression ')'?
    |   'setarray' '('? assignmentLeftExpression ',' assignmentExpression (',' argumentExpressionList)? ')'?
    |   'copyarray' '('? assignmentLeftExpression ',' assignmentExpression ',' argumentExpressionList ')'?
    |   Number // for
    ;
assignmentLeftExpression
    : Identifier | variable;

assignmentOperator
    :   '=' | '*=' | '/=' | '%=' | '+=' | '-=' | '<<=' | '>>=' | '&=' | '^=' | '|='
    ;

expression
    :   assignmentExpression (',' assignmentExpression)*
    ;

constantExpression
    :   conditionalExpression
    ;

declaration
    :   initDeclaratorList? ';'
    ;

declarationSpecifiers
    :   declarationSpecifier+
    ;

declarationSpecifiers2
    :   scope_specifier
    ;

declarationSpecifier
    :   scope_specifier
    ;

initDeclaratorList
    :   initDeclarator (',' initDeclarator)*
    ;

initDeclarator
    :  variable ('=' initializer)?
    | Function Identifier
    ;

specifierQualifierList
    :   (scope_specifier) specifierQualifierList?
    ;

declarator
    :   directDeclarator
    ;

directDeclarator
    :   variable
    |   '(' declarator ')'
    |   directDeclarator '[' '*' ']'
    |   directDeclarator '(' parameterTypeList ')'
    |   directDeclarator '(' identifierList? ')'
    |   '('  directDeclarator ')'
    ;

nestedParenthesesBlock
    :   (   ~('(' | ')')
        |   '(' nestedParenthesesBlock ')'
        )*
    ;


parameterTypeList
    :   parameterList (',' '...')?
    ;

parameterList
    :   parameterDeclaration (',' parameterDeclaration)*
    ;

parameterDeclaration
    :   declarationSpecifiers declarator
    |   declarationSpecifiers2 directAbstractDeclarator?
    ;

identifierList
    :   Identifier (',' Identifier)*
    ;

directAbstractDeclarator
    :   '[' '*' ']'
    |   directAbstractDeclarator '[' '*' ']'
    ;

initializer
    :   assignmentExpression
    |   '{' initializerList ','? '}'
    ;

initializerList
    :   designation? initializer (',' designation? initializer)*
    ;

designation
    :   designatorList '='
    ;

designatorList
    :   designator+
    ;

designator
    :   '[' constantExpression ']'
    |   '.' Identifier
    ;


statement
    :   compoundStatement
    |   expressionStatement
    |   selectionStatement
    |   iterationStatement
    |   jumpStatement
    ;

labeledStatement
    : Label statement*
    ;

compoundStatement
    :   '{' blockItemList? '}'
    ;

blockItemList
    :   blockItem+
    ;

blockItem
    :   statement
    |   labeledStatement
    |   functionDefinition
    |   declaration
    ;

expressionStatement
    :   expression? ';'
    ;

selectionStatement
    :   'if' '(' expression ')' statement ('else' statement)?
    |   switchStatement
//    |   'switch' '(' expression ')' statement
    ;

switchStatement
	:	'switch' '(' expression ')' switchBlock
	;

switchBlock
	:	'{' switchBlockStatementGroup* '}'
	;

switchBlockStatementGroup
	:	switchLabels blockItemList
	;

switchLabels
	:	switchLabel switchLabel*
	;

switchLabel
	:	'case' constantExpression ':'
	|	'default:'
	;

iterationStatement
    :   While '(' expression ')' statement
    |   Do statement While '(' expression ')' ';'
    |   For '(' forCondition ')' statement
    ;

//    |   'for' '(' expression? ';' expression?  ';' forUpdate? ')' statement
//    |   For '(' declaration  expression? ';' expression? ')' statement

forCondition
	:  forDeclaration ';' forStopExpression? ';' forExpression?
	;

forDeclaration
    :  expression?
    ;

forStopExpression
    :   assignmentExpression (',' assignmentExpression)*
    ;
forExpression
    :   assignmentExpression (',' assignmentExpression)*
    ;

jumpStatement
    :   ('goto' Identifier
    |   ('continue'| 'break' | 'end')
    |   'return' expression?
    )
    ';'
    ;

translationUnit
    : externalDeclaration+
    ;

externalDeclaration
    : functionDefinition
    | scriptInitialization
    | npcInitialization
    |   ';' // stray ;
    ;

functionDefinition
    :  Function Identifier  compoundStatement?
    ;
scriptInitialization
    : '-' 'script' scriptName (scriptSprite) (',' (scriptSprite))* ',' compoundStatement
    | scriptLocation ',' scriptXPos ',' scriptYPos ',' scriptDir  'script'  scriptName  (scriptSprite) (',' (scriptSprite))* ',' compoundStatement
    ;
scriptLocation : Identifier;
scriptXPos : Number;
scriptYPos : Number;
scriptDir : Number;
scriptSprite : (Minus? Number | Identifier);

npcInitialization
    : Identifier (',' Number?)+  Identifier  scriptName (Number | Identifier) (',' (Number | Identifier))*
    | scriptLocation ',' scriptXPos ',' scriptYPos ',' scriptDir  'duplicate' '(' scriptName ')' scriptName (scriptSprite) (',' (scriptSprite))*
    ;
scriptName
    : (Identifier | ':' | '#' | Colon | Label | Number)*
    ;

scope_specifier
  :  '@' | '$' | '$@' | '.' | '.@' | '\'' | '#' | '##';
variable
  : scope_specifier variable_name | variable_name;
variable_name
  : Identifier '$'? ('[' Number ']')?;

// Tokens
LeftParen : '(';
RightParen : ')';
LeftBrace : '{';
RightBrace : '}';
LeftBracket : '[';
RightBracket : ']';
Comma : ',';
At : '@';
Colon : ':';
SemiColon : ';';
Percent : '%';
Star : '*';
Tilde : '~';
QuestionMark : '?';
Quote : '\'';
DoubleQuote : '"';
// one or two char token
LogicalOr : '|';
OrOp : '||';
LogicalAnd : '&';
AndOp : '&&';
Slash : '/';
SlashStar : '/*';
StarSlash : '*/';
DoubleSlash : '//';
Sharp : '#';
DoubleSharp : '##';
Minus : '-';
DecrementOp : '--';
Plus : '+';
IncrementOp : '++';
Dot : '.';
DotAt : '.@';
Dollar : '$';
DollarAt : '$@';
Bang : '!';
BangEqual : '!=';
Equal : '=';
DoubleEqual : '==';
LeftCaret : '<';
DoubleLeftCaret : '<<';
LeftCaretEqual : '<=';
RightCaret : '>';
DoubleRightCaret : '>>';
RightCaretEqual : '>=';
PlusEqual : '+=';
MinusEqual : '-=';
MultiplyEqual : '*=';
DivideEqual : '/=';
PercentEqual : '%=';

// Keywords
If : 'if';
Else : 'else';
End : 'end';
Set : 'set';
For : 'for';
While : 'while';
Do : 'do';
Goto : 'goto';
Return : 'return';
Switch : 'switch';
Case : 'case';
Default : 'default:';
Function : 'function';
Break : 'break';
Callfunc: 'callfunc';
Eof : 'eof';
Setarray: 'setarray';
Copyarray: 'copyarray';

// Literal
Identifier : Letter (Letter | Digit)*;

Label : Identifier ':';

String : '"' ( '\\"' | . )*? '"' ;
fragment Escape : '\\' ( '\'' | '\\' );
fragment Digit : [0-9];
fragment Letter : [A-Za-z_];

Number : Digit+;

Whitespace
    :   [ \t]+
        -> skip
    ;

Newline
    :   (   '\r' '\n'?
        |   '\n'
        )
        -> skip
    ;

BlockComment
    :   '/*' .*? '*/'
        -> skip
    ;

LineComment
    :
        ('//' Newline
    |    '//' ~[\r\n]*
    )
        -> skip
    ;