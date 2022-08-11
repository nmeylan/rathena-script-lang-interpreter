grammar RathenaScriptLang;

compilationUnit
    :   translationUnit EOF
    ;

primaryExpression
    : variable
    | Identifier
    | True
    | False
    | Number
    | String
    | '-' // it is a special arguments for command, lets see if it can cause weird parse issue
    ;

functionCallExpression
    : Identifier '(' argumentExpressionList? ')'
    | Underscore '(' argumentExpressionList? ')'
    | Identifier argumentExpressionList
    | Callfunc '('? String ( ',' argumentExpressionList)? ')'
    | Callfunc String ( ',' argumentExpressionList)?
    | Callsub '('? Identifier ( ',' argumentExpressionList)? ')'
    | Callsub Identifier ( ',' argumentExpressionList)?
    ;

postfixExpression
    : primaryExpression
    | incrementThenLoad
    | loadThenIncrement
    | '(' conditionalExpression ')'
    | functionCallExpression
    ;

incrementThenLoad
    : ('++' | '--') variable
    ;
loadThenIncrement
    : variable ('++' | '--')
    ;

argumentExpressionList
    :   conditionalExpression (',' conditionalExpression)*
    ;

unaryExpression
    :
    postfixExpression
    |   unaryOperator unaryExpression
    ;

unaryOperator
    :   '&' | '*' | '+' | '-' | '~' | '!'
    ;

multiplicativeExpression
    :   unaryExpression (multiplicativeOperator unaryExpression)*
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
    :   logicalOrExpression ('?' conditionalExpression ':' conditionalExpression)?
    ;

assignmentExpression
    : (assignmentLeftExpression assignmentOperator)+ conditionalExpression
    |   'set' '('? functionCallExpression ',' conditionalExpression ')'? // only set(getd()) will be allowed by compiler, not any function call
    |   'set' '('? assignmentLeftExpression ',' conditionalExpression ')'?
    |   'setarray' '('? assignmentLeftExpression ',' conditionalExpression (',' argumentExpressionList)? ')'?
    |   'copyarray' '('? assignmentLeftExpression ',' conditionalExpression ',' argumentExpressionList ')'?
    ;

assignmentLeftExpression
    : variable;

assignmentOperator
    :   '=' | '*=' | '/=' | '%=' | '+=' | '-=' | '<<=' | '>>=' | '&=' | '^=' | '|='
    ;


constantExpression
    :   conditionalExpression
    ;


statement
    :   compoundStatement
    |   commandStatement ';'
    |   expressionStatement
    |   selectionStatement
    |   iterationStatement
    |   jumpStatement
    |   labeledStatement
    ;
commandStatement
    : Menu menuOptionText ',' menuLabel  (',' menuOptionText ',' menuLabel)*
    | Close | Close2 | Next;
menuOptionText
    :  (String | conditionalExpression);
menuLabel
    :  (Identifier | '-');
labeledStatement
    : Label;

compoundStatement
    :   '{' blockItemList? '}'
    ;

blockItemList
    :   blockItem+
    ;

blockItem
    :   statement
    |   functionDefinition
    ;

expressionStatement
    : assignmentExpression ';'
    | conditionalExpression ';'
    ;

selectionStatement
    :   'if' '(' conditionalExpression ')' statement ('else' statement)?
    |   switchStatement
    ;

switchStatement
	:	'switch' '(' conditionalExpression ')' switchBlock
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
    :   While '(' conditionalExpression ')' statement
    |   Do statement While '(' conditionalExpression ')' ';'
    |   For '(' forCondition ')' statement
    ;

forCondition
	:  forDeclaration ';' forStopExpression? ';' forExpression?
	;

forDeclaration
    :  assignmentExpression?
    ;

forStopExpression
    :   assignmentExpression | conditionalExpression
    ;
forExpression
    :   assignmentExpression | conditionalExpression
    ;

jumpStatement
    :   ('goto' Identifier
    |   ('continue'| 'break' | 'end')
    |   'return' conditionalExpression?
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
    :  Function 'script' Identifier compoundStatement
    |  Function Identifier compoundStatement
    |  Function Identifier ';'
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
  : scope_specifier? variable_name '$'? ('[' conditionalExpression ']')?
  ;
variable_name
  : (Identifier | Menu | Next);

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
Underscore: '_';
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
Callsub: 'callsub';
Eof : 'eof';
Setarray: 'setarray';
Copyarray: 'copyarray';
True: 'true';
False: 'false';
// Functions without args
Menu: 'menu';
Close: 'close';
Close2: 'close2';
Next: 'next';
Script: 'script';

// Literal
Identifier : (Letter | Digit '_' Letter) (Letter | Digit)*;

Label : Identifier ':';

String : '"' ( '\\"' | . )*? '"' ;
fragment Escape : '\\' ( '\'' | '\\' );
fragment Digit : [0-9];
fragment Letter : [A-Za-z_];

Number : Digit+ | '0'[xX][a-fA-F]+;

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