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
    : Identifier '(' argumentExpressionList? ')' | 'callfunc' '('? String ( ',' argumentExpressionList)? ')' | 'callfunc' String ( ',' argumentExpressionList)?;

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
    :   labeledStatement
    |   compoundStatement
    |   expressionStatement
    |   selectionStatement
    |   iterationStatement
    |   jumpStatement
    |   menuStatement
    |   commandStatement
    |   dialogStatement
    ;

labeledStatement
    :   Identifier ':' statement
    |   'case' constantExpression ':' statement
    |   'default' ':' statement
    ;

compoundStatement
    :   '{' blockItemList? '}'
    ;

blockItemList
    :   blockItem+
    ;

blockItem
    :   statement
    |   functionDefinition
    |   declaration
    ;

expressionStatement
    :   expression? ';'
    ;

selectionStatement
    :   'if' '(' expression ')' statement ('else' statement)?
    |   'switch' '(' expression ')' statement
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

menuStatement
    : Menu menuItem (',' menuItem)*;

menuItem
    : expression ',' (Identifier | '-');

commandStatement
    : Identifier expression (',' expression)*;

dialogStatement
    : (Close | Close2 | Next) ';';


translationUnit
    : externalDeclaration+
    ;

externalDeclaration
    : functionDefinition
    | blockItem
    | scriptInitialization
    |   ';' // stray ;
    ;

functionDefinition
    :  Function Identifier  compoundStatement?
    ;
scriptInitialization
    : '-' (Identifier | '::')* ','? compoundStatement? ;

scope_specifier
  :  '@' | '$' | '$@' | '.' | '.@' | '\'' | '#' | '##';
variable
  : scope_specifier variable_name | variable_name;
variable_name
  : (Identifier | Menu) '$'?;

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
Function : 'function';
Break : 'break';
SetArray: 'setarray';
Callfunc: 'callfunc';
GetArraySize: 'getarraysize';
Close: 'close';
Close2: 'close2';
Next: 'next';
Menu: 'menu';
Eof : 'eof';

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
    :   '//' ~[\r\n]*
        -> skip
    ;