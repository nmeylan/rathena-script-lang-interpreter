// Generated from RathenaScriptLang.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::rathenascriptlanglistener::*;
use super::rathenascriptlangvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const LeftParen:isize=16; 
		pub const RightParen:isize=17; 
		pub const LeftBrace:isize=18; 
		pub const RightBrace:isize=19; 
		pub const LeftBracket:isize=20; 
		pub const RightBracket:isize=21; 
		pub const Comma:isize=22; 
		pub const At:isize=23; 
		pub const Colon:isize=24; 
		pub const SemiColon:isize=25; 
		pub const Percent:isize=26; 
		pub const Star:isize=27; 
		pub const Tilde:isize=28; 
		pub const QuestionMark:isize=29; 
		pub const Quote:isize=30; 
		pub const DoubleQuote:isize=31; 
		pub const LogicalOr:isize=32; 
		pub const OrOp:isize=33; 
		pub const LogicalAnd:isize=34; 
		pub const AndOp:isize=35; 
		pub const Slash:isize=36; 
		pub const SlashStar:isize=37; 
		pub const StarSlash:isize=38; 
		pub const DoubleSlash:isize=39; 
		pub const Sharp:isize=40; 
		pub const DoubleSharp:isize=41; 
		pub const Minus:isize=42; 
		pub const DecrementOp:isize=43; 
		pub const Plus:isize=44; 
		pub const IncrementOp:isize=45; 
		pub const Dot:isize=46; 
		pub const DotAt:isize=47; 
		pub const Dollar:isize=48; 
		pub const DollarAt:isize=49; 
		pub const Bang:isize=50; 
		pub const BangEqual:isize=51; 
		pub const Equal:isize=52; 
		pub const DoubleEqual:isize=53; 
		pub const LeftCaret:isize=54; 
		pub const DoubleLeftCaret:isize=55; 
		pub const LeftCaretEqual:isize=56; 
		pub const RightCaret:isize=57; 
		pub const DoubleRightCaret:isize=58; 
		pub const RightCaretEqual:isize=59; 
		pub const If:isize=60; 
		pub const Else:isize=61; 
		pub const End:isize=62; 
		pub const Set:isize=63; 
		pub const For:isize=64; 
		pub const While:isize=65; 
		pub const Do:isize=66; 
		pub const Goto:isize=67; 
		pub const Return:isize=68; 
		pub const Switch:isize=69; 
		pub const Case:isize=70; 
		pub const Function:isize=71; 
		pub const Break:isize=72; 
		pub const SetArray:isize=73; 
		pub const GetArraySize:isize=74; 
		pub const Close:isize=75; 
		pub const Close2:isize=76; 
		pub const Next:isize=77; 
		pub const Menu:isize=78; 
		pub const Eof:isize=79; 
		pub const Identifier:isize=80; 
		pub const Label:isize=81; 
		pub const String:isize=82; 
		pub const Number:isize=83; 
		pub const Whitespace:isize=84; 
		pub const Newline:isize=85; 
		pub const BlockComment:isize=86; 
		pub const LineComment:isize=87;
	pub const RULE_compilationUnit:usize = 0; 
	pub const RULE_primaryExpression:usize = 1; 
	pub const RULE_functionCallExpression:usize = 2; 
	pub const RULE_postfixExpression:usize = 3; 
	pub const RULE_argumentExpressionList:usize = 4; 
	pub const RULE_unaryExpression:usize = 5; 
	pub const RULE_unaryOperator:usize = 6; 
	pub const RULE_castExpression:usize = 7; 
	pub const RULE_multiplicativeExpression:usize = 8; 
	pub const RULE_additiveExpression:usize = 9; 
	pub const RULE_shiftExpression:usize = 10; 
	pub const RULE_relationalExpression:usize = 11; 
	pub const RULE_equalityExpression:usize = 12; 
	pub const RULE_andExpression:usize = 13; 
	pub const RULE_exclusiveOrExpression:usize = 14; 
	pub const RULE_inclusiveOrExpression:usize = 15; 
	pub const RULE_logicalAndExpression:usize = 16; 
	pub const RULE_logicalOrExpression:usize = 17; 
	pub const RULE_conditionalExpression:usize = 18; 
	pub const RULE_assignmentExpression:usize = 19; 
	pub const RULE_assignmentLeftExpression:usize = 20; 
	pub const RULE_assignmentOperator:usize = 21; 
	pub const RULE_expression:usize = 22; 
	pub const RULE_constantExpression:usize = 23; 
	pub const RULE_declaration:usize = 24; 
	pub const RULE_declarationSpecifiers:usize = 25; 
	pub const RULE_declarationSpecifiers2:usize = 26; 
	pub const RULE_declarationSpecifier:usize = 27; 
	pub const RULE_initDeclaratorList:usize = 28; 
	pub const RULE_initDeclarator:usize = 29; 
	pub const RULE_specifierQualifierList:usize = 30; 
	pub const RULE_declarator:usize = 31; 
	pub const RULE_directDeclarator:usize = 32; 
	pub const RULE_nestedParenthesesBlock:usize = 33; 
	pub const RULE_parameterTypeList:usize = 34; 
	pub const RULE_parameterList:usize = 35; 
	pub const RULE_parameterDeclaration:usize = 36; 
	pub const RULE_identifierList:usize = 37; 
	pub const RULE_directAbstractDeclarator:usize = 38; 
	pub const RULE_initializer:usize = 39; 
	pub const RULE_initializerList:usize = 40; 
	pub const RULE_designation:usize = 41; 
	pub const RULE_designatorList:usize = 42; 
	pub const RULE_designator:usize = 43; 
	pub const RULE_statement:usize = 44; 
	pub const RULE_labeledStatement:usize = 45; 
	pub const RULE_compoundStatement:usize = 46; 
	pub const RULE_blockItemList:usize = 47; 
	pub const RULE_blockItem:usize = 48; 
	pub const RULE_expressionStatement:usize = 49; 
	pub const RULE_selectionStatement:usize = 50; 
	pub const RULE_iterationStatement:usize = 51; 
	pub const RULE_forCondition:usize = 52; 
	pub const RULE_forDeclaration:usize = 53; 
	pub const RULE_forExpression:usize = 54; 
	pub const RULE_jumpStatement:usize = 55; 
	pub const RULE_menuStatement:usize = 56; 
	pub const RULE_menuItem:usize = 57; 
	pub const RULE_commandStatement:usize = 58; 
	pub const RULE_dialogStatement:usize = 59; 
	pub const RULE_translationUnit:usize = 60; 
	pub const RULE_externalDeclaration:usize = 61; 
	pub const RULE_functionDefinition:usize = 62; 
	pub const RULE_functionDefinitionArguments:usize = 63; 
	pub const RULE_scriptInitialization:usize = 64; 
	pub const RULE_scope_specifier:usize = 65; 
	pub const RULE_variable:usize = 66; 
	pub const RULE_variable_name:usize = 67;
	pub const ruleNames: [&'static str; 68] =  [
		"compilationUnit", "primaryExpression", "functionCallExpression", "postfixExpression", 
		"argumentExpressionList", "unaryExpression", "unaryOperator", "castExpression", 
		"multiplicativeExpression", "additiveExpression", "shiftExpression", "relationalExpression", 
		"equalityExpression", "andExpression", "exclusiveOrExpression", "inclusiveOrExpression", 
		"logicalAndExpression", "logicalOrExpression", "conditionalExpression", 
		"assignmentExpression", "assignmentLeftExpression", "assignmentOperator", 
		"expression", "constantExpression", "declaration", "declarationSpecifiers", 
		"declarationSpecifiers2", "declarationSpecifier", "initDeclaratorList", 
		"initDeclarator", "specifierQualifierList", "declarator", "directDeclarator", 
		"nestedParenthesesBlock", "parameterTypeList", "parameterList", "parameterDeclaration", 
		"identifierList", "directAbstractDeclarator", "initializer", "initializerList", 
		"designation", "designatorList", "designator", "statement", "labeledStatement", 
		"compoundStatement", "blockItemList", "blockItem", "expressionStatement", 
		"selectionStatement", "iterationStatement", "forCondition", "forDeclaration", 
		"forExpression", "jumpStatement", "menuStatement", "menuItem", "commandStatement", 
		"dialogStatement", "translationUnit", "externalDeclaration", "functionDefinition", 
		"functionDefinitionArguments", "scriptInitialization", "scope_specifier", 
		"variable", "variable_name"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;80] = [
		None, Some("'^'"), Some("'*='"), Some("'/='"), Some("'%='"), Some("'+='"), 
		Some("'-='"), Some("'<<='"), Some("'>>='"), Some("'&='"), Some("'^='"), 
		Some("'|='"), Some("'...'"), Some("'default'"), Some("'continue'"), Some("'::'"), 
		Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), Some("']'"), 
		Some("','"), Some("'@'"), Some("':'"), Some("';'"), Some("'%'"), Some("'*'"), 
		Some("'~'"), Some("'?'"), Some("'''"), Some("'\"'"), Some("'|'"), Some("'||'"), 
		Some("'&'"), Some("'&&'"), Some("'/'"), Some("'/*'"), Some("'*/'"), Some("'//'"), 
		Some("'#'"), Some("'##'"), Some("'-'"), Some("'--'"), Some("'+'"), Some("'++'"), 
		Some("'.'"), Some("'.@'"), Some("'$'"), Some("'$@'"), Some("'!'"), Some("'!='"), 
		Some("'='"), Some("'=='"), Some("'<'"), Some("'<<'"), Some("'<='"), Some("'>'"), 
		Some("'>>'"), Some("'>='"), Some("'if'"), Some("'else'"), Some("'end'"), 
		Some("'set'"), Some("'for'"), Some("'while'"), Some("'do'"), Some("'goto'"), 
		Some("'return'"), Some("'switch'"), Some("'case'"), Some("'function'"), 
		Some("'break'"), Some("'setarray'"), Some("'getarraysize'"), Some("'close'"), 
		Some("'close2'"), Some("'next'"), Some("'menu'"), Some("'eof'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;88]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, Some("LeftParen"), Some("RightParen"), Some("LeftBrace"), 
		Some("RightBrace"), Some("LeftBracket"), Some("RightBracket"), Some("Comma"), 
		Some("At"), Some("Colon"), Some("SemiColon"), Some("Percent"), Some("Star"), 
		Some("Tilde"), Some("QuestionMark"), Some("Quote"), Some("DoubleQuote"), 
		Some("LogicalOr"), Some("OrOp"), Some("LogicalAnd"), Some("AndOp"), Some("Slash"), 
		Some("SlashStar"), Some("StarSlash"), Some("DoubleSlash"), Some("Sharp"), 
		Some("DoubleSharp"), Some("Minus"), Some("DecrementOp"), Some("Plus"), 
		Some("IncrementOp"), Some("Dot"), Some("DotAt"), Some("Dollar"), Some("DollarAt"), 
		Some("Bang"), Some("BangEqual"), Some("Equal"), Some("DoubleEqual"), Some("LeftCaret"), 
		Some("DoubleLeftCaret"), Some("LeftCaretEqual"), Some("RightCaret"), Some("DoubleRightCaret"), 
		Some("RightCaretEqual"), Some("If"), Some("Else"), Some("End"), Some("Set"), 
		Some("For"), Some("While"), Some("Do"), Some("Goto"), Some("Return"), 
		Some("Switch"), Some("Case"), Some("Function"), Some("Break"), Some("SetArray"), 
		Some("GetArraySize"), Some("Close"), Some("Close2"), Some("Next"), Some("Menu"), 
		Some("Eof"), Some("Identifier"), Some("Label"), Some("String"), Some("Number"), 
		Some("Whitespace"), Some("Newline"), Some("BlockComment"), Some("LineComment")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,RathenaScriptLangParserExt, I, RathenaScriptLangParserContextType , dyn RathenaScriptLangListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type RathenaScriptLangTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, RathenaScriptLangParserContextType , dyn RathenaScriptLangListener<'input> + 'a>;

/// Parser for RathenaScriptLang grammar
pub struct RathenaScriptLangParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				RathenaScriptLangParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> RathenaScriptLangParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> RathenaScriptLangParser<'input, I, DefaultErrorStrategy<'input,RathenaScriptLangParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for RathenaScriptLangParser
pub trait RathenaScriptLangParserContext<'input>:
	for<'x> Listenable<dyn RathenaScriptLangListener<'input> + 'x > + 
	for<'x> Visitable<dyn RathenaScriptLangVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=RathenaScriptLangParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn RathenaScriptLangParserContext<'input> + 'input
where
    T: RathenaScriptLangVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn RathenaScriptLangVisitor<'input> + 'x))
    }
}

impl<'input> RathenaScriptLangParserContext<'input> for TerminalNode<'input,RathenaScriptLangParserContextType> {}
impl<'input> RathenaScriptLangParserContext<'input> for ErrorNode<'input,RathenaScriptLangParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn RathenaScriptLangParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn RathenaScriptLangListener<'input> + 'input{}

pub struct RathenaScriptLangParserContextType;
antlr_rust::type_id!{RathenaScriptLangParserContextType}

impl<'input> ParserNodeType<'input> for RathenaScriptLangParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn RathenaScriptLangParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct RathenaScriptLangParserExt{
}

impl RathenaScriptLangParserExt{
}


impl<'input> TokenAware<'input> for RathenaScriptLangParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for RathenaScriptLangParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for RathenaScriptLangParserExt{
	fn get_grammar_file_name(&self) -> & str{ "RathenaScriptLang.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn RathenaScriptLangParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					32 => RathenaScriptLangParser::<'input,I,_>::directDeclarator_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					38 => RathenaScriptLangParser::<'input,I,_>::directAbstractDeclarator_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> RathenaScriptLangParser<'input, I, DefaultErrorStrategy<'input,RathenaScriptLangParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn directDeclarator_sempred(_localctx: Option<&DirectDeclaratorContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 4)
				}
				1=>{
					recog.precpred(None, 3)
				}
				2=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn directAbstractDeclarator_sempred(_localctx: Option<&DirectAbstractDeclaratorContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- compilationUnit ----------------
pub type CompilationUnitContextAll<'input> = CompilationUnitContext<'input>;


pub type CompilationUnitContext<'input> = BaseParserRuleContext<'input,CompilationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct CompilationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for CompilationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for CompilationUnitContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compilationUnit(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_compilationUnit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for CompilationUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_compilationUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompilationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilationUnit }
}
antlr_rust::type_id!{CompilationUnitContextExt<'a>}

impl<'input> CompilationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilationUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilationUnitContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<CompilationUnitContextExt<'input>>{

fn translationUnit(&self) -> Option<Rc<TranslationUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> CompilationUnitContextAttrs<'input> for CompilationUnitContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilationUnit(&mut self,)
	-> Result<Rc<CompilationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_compilationUnit);
        let mut _localctx: Rc<CompilationUnitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule translationUnit*/
			recog.base.set_state(136);
			recog.translationUnit()?;

			recog.base.set_state(137);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primaryExpression ----------------
pub type PrimaryExpressionContextAll<'input> = PrimaryExpressionContext<'input>;


pub type PrimaryExpressionContext<'input> = BaseParserRuleContext<'input,PrimaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for PrimaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for PrimaryExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_primaryExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_primaryExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for PrimaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_primaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primaryExpression }
}
antlr_rust::type_id!{PrimaryExpressionContextExt<'a>}

impl<'input> PrimaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<PrimaryExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}
/// Retrieves first TerminalNode corresponding to token String
/// Returns `None` if there is no child corresponding to token String
fn String(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(String, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Minus, 0)
}

}

impl<'input> PrimaryExpressionContextAttrs<'input> for PrimaryExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primaryExpression(&mut self,)
	-> Result<Rc<PrimaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_primaryExpression);
        let mut _localctx: Rc<PrimaryExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(144);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(139);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(140);
					recog.variable()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(141);
					recog.base.match_token(Number,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(142);
					recog.base.match_token(String,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(143);
					recog.base.match_token(Minus,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionCallExpression ----------------
pub type FunctionCallExpressionContextAll<'input> = FunctionCallExpressionContext<'input>;


pub type FunctionCallExpressionContext<'input> = BaseParserRuleContext<'input,FunctionCallExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionCallExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for FunctionCallExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for FunctionCallExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionCallExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_functionCallExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for FunctionCallExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_functionCallExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCallExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCallExpression }
}
antlr_rust::type_id!{FunctionCallExpressionContextExt<'a>}

impl<'input> FunctionCallExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<FunctionCallExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn argumentExpressionList(&self) -> Option<Rc<ArgumentExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionCallExpressionContextAttrs<'input> for FunctionCallExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCallExpression(&mut self,)
	-> Result<Rc<FunctionCallExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_functionCallExpression);
        let mut _localctx: Rc<FunctionCallExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(146);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(147);
			recog.base.match_token(LeftParen,&mut recog.err_handler)?;

			recog.base.set_state(149);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Star - 23)) | (1usize << (Tilde - 23)) | (1usize << (Quote - 23)) | (1usize << (LogicalAnd - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Minus - 23)) | (1usize << (DecrementOp - 23)) | (1usize << (Plus - 23)) | (1usize << (IncrementOp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Bang - 23)) | (1usize << (GetArraySize - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)) | (1usize << (String - 23)) | (1usize << (Number - 23)))) != 0) {
				{
				/*InvokeRule argumentExpressionList*/
				recog.base.set_state(148);
				recog.argumentExpressionList()?;

				}
			}

			recog.base.set_state(151);
			recog.base.match_token(RightParen,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- postfixExpression ----------------
pub type PostfixExpressionContextAll<'input> = PostfixExpressionContext<'input>;


pub type PostfixExpressionContext<'input> = BaseParserRuleContext<'input,PostfixExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PostfixExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for PostfixExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for PostfixExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_postfixExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_postfixExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for PostfixExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_postfixExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostfixExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfixExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfixExpression }
}
antlr_rust::type_id!{PostfixExpressionContextExt<'a>}

impl<'input> PostfixExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PostfixExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostfixExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PostfixExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<PostfixExpressionContextExt<'input>>{

fn functionCallExpression(&self) -> Option<Rc<FunctionCallExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn primaryExpression(&self) -> Option<Rc<PrimaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftBracket in current rule
fn LeftBracket_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftBracket, starting from 0.
/// Returns `None` if number of children corresponding to token LeftBracket is less or equal than `i`.
fn LeftBracket(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBracket, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightBracket in current rule
fn RightBracket_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightBracket, starting from 0.
/// Returns `None` if number of children corresponding to token RightBracket is less or equal than `i`.
fn RightBracket(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBracket, i)
}
/// Retrieves all `TerminalNode`s corresponding to token IncrementOp in current rule
fn IncrementOp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IncrementOp, starting from 0.
/// Returns `None` if number of children corresponding to token IncrementOp is less or equal than `i`.
fn IncrementOp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(IncrementOp, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DecrementOp in current rule
fn DecrementOp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DecrementOp, starting from 0.
/// Returns `None` if number of children corresponding to token DecrementOp is less or equal than `i`.
fn DecrementOp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DecrementOp, i)
}

}

impl<'input> PostfixExpressionContextAttrs<'input> for PostfixExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postfixExpression(&mut self,)
	-> Result<Rc<PostfixExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostfixExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_postfixExpression);
        let mut _localctx: Rc<PostfixExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(171);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule functionCallExpression*/
					recog.base.set_state(153);
					recog.functionCallExpression()?;

					recog.base.set_state(157);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule expression*/
							recog.base.set_state(154);
							recog.expression()?;

							}
							} 
						}
						recog.base.set_state(159);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule primaryExpression*/
					recog.base.set_state(160);
					recog.primaryExpression()?;

					recog.base.set_state(168);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							recog.base.set_state(166);
							recog.err_handler.sync(&mut recog.base)?;
							match recog.base.input.la(1) {
							 LeftBracket 
								=> {
									{
									recog.base.set_state(161);
									recog.base.match_token(LeftBracket,&mut recog.err_handler)?;

									/*InvokeRule expression*/
									recog.base.set_state(162);
									recog.expression()?;

									recog.base.set_state(163);
									recog.base.match_token(RightBracket,&mut recog.err_handler)?;

									}
								}

							 DecrementOp | IncrementOp 
								=> {
									{
									recog.base.set_state(165);
									_la = recog.base.input.la(1);
									if { !(_la==DecrementOp || _la==IncrementOp) } {
										recog.err_handler.recover_inline(&mut recog.base)?;

									}
									else {
										if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
										recog.err_handler.report_match(&mut recog.base);
										recog.base.consume(&mut recog.err_handler);
									}
									}
								}

								_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
							}
							} 
						}
						recog.base.set_state(170);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
					}
					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- argumentExpressionList ----------------
pub type ArgumentExpressionListContextAll<'input> = ArgumentExpressionListContext<'input>;


pub type ArgumentExpressionListContext<'input> = BaseParserRuleContext<'input,ArgumentExpressionListContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentExpressionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ArgumentExpressionListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ArgumentExpressionListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_argumentExpressionList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_argumentExpressionList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ArgumentExpressionListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_argumentExpressionList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArgumentExpressionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argumentExpressionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argumentExpressionList }
}
antlr_rust::type_id!{ArgumentExpressionListContextExt<'a>}

impl<'input> ArgumentExpressionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentExpressionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentExpressionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentExpressionListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ArgumentExpressionListContextExt<'input>>{

fn assignmentExpression_all(&self) ->  Vec<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentExpression(&self, i: usize) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> ArgumentExpressionListContextAttrs<'input> for ArgumentExpressionListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn argumentExpressionList(&mut self,)
	-> Result<Rc<ArgumentExpressionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentExpressionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_argumentExpressionList);
        let mut _localctx: Rc<ArgumentExpressionListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule assignmentExpression*/
			recog.base.set_state(173);
			recog.assignmentExpression()?;

			recog.base.set_state(178);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(174);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				/*InvokeRule assignmentExpression*/
				recog.base.set_state(175);
				recog.assignmentExpression()?;

				}
				}
				recog.base.set_state(180);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryExpression ----------------
pub type UnaryExpressionContextAll<'input> = UnaryExpressionContext<'input>;


pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for UnaryExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_unaryExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for UnaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_unaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}
antlr_rust::type_id!{UnaryExpressionContextExt<'a>}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<UnaryExpressionContextExt<'input>>{

fn postfixExpression(&self) -> Option<Rc<PostfixExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unaryOperator(&self) -> Option<Rc<UnaryOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn castExpression(&self) -> Option<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token GetArraySize
/// Returns `None` if there is no child corresponding to token GetArraySize
fn GetArraySize(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(GetArraySize, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IncrementOp in current rule
fn IncrementOp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IncrementOp, starting from 0.
/// Returns `None` if number of children corresponding to token IncrementOp is less or equal than `i`.
fn IncrementOp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(IncrementOp, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DecrementOp in current rule
fn DecrementOp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DecrementOp, starting from 0.
/// Returns `None` if number of children corresponding to token DecrementOp is less or equal than `i`.
fn DecrementOp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DecrementOp, i)
}

}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryExpression(&mut self,)
	-> Result<Rc<UnaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_unaryExpression);
        let mut _localctx: Rc<UnaryExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(184);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DecrementOp || _la==IncrementOp {
				{
				{
				recog.base.set_state(181);
				_la = recog.base.input.la(1);
				if { !(_la==DecrementOp || _la==IncrementOp) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(186);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(196);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule postfixExpression*/
					recog.base.set_state(187);
					recog.postfixExpression()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule unaryOperator*/
					recog.base.set_state(188);
					recog.unaryOperator()?;

					/*InvokeRule castExpression*/
					recog.base.set_state(189);
					recog.castExpression()?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(191);
					recog.base.match_token(GetArraySize,&mut recog.err_handler)?;

					recog.base.set_state(192);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule variable*/
					recog.base.set_state(193);
					recog.variable()?;

					recog.base.set_state(194);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryOperator ----------------
pub type UnaryOperatorContextAll<'input> = UnaryOperatorContext<'input>;


pub type UnaryOperatorContext<'input> = BaseParserRuleContext<'input,UnaryOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for UnaryOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for UnaryOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_unaryOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for UnaryOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_unaryOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryOperator }
}
antlr_rust::type_id!{UnaryOperatorContextExt<'a>}

impl<'input> UnaryOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<UnaryOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LogicalAnd
/// Returns `None` if there is no child corresponding to token LogicalAnd
fn LogicalAnd(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LogicalAnd, 0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Star, 0)
}
/// Retrieves first TerminalNode corresponding to token Plus
/// Returns `None` if there is no child corresponding to token Plus
fn Plus(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Plus, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Minus, 0)
}
/// Retrieves first TerminalNode corresponding to token Tilde
/// Returns `None` if there is no child corresponding to token Tilde
fn Tilde(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Tilde, 0)
}
/// Retrieves first TerminalNode corresponding to token Bang
/// Returns `None` if there is no child corresponding to token Bang
fn Bang(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Bang, 0)
}

}

impl<'input> UnaryOperatorContextAttrs<'input> for UnaryOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryOperator(&mut self,)
	-> Result<Rc<UnaryOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_unaryOperator);
        let mut _localctx: Rc<UnaryOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(198);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << Star) | (1usize << Tilde) | (1usize << LogicalAnd) | (1usize << Minus) | (1usize << Plus) | (1usize << Bang))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- castExpression ----------------
pub type CastExpressionContextAll<'input> = CastExpressionContext<'input>;


pub type CastExpressionContext<'input> = BaseParserRuleContext<'input,CastExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct CastExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for CastExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for CastExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_castExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_castExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for CastExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_castExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for CastExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_castExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_castExpression }
}
antlr_rust::type_id!{CastExpressionContextExt<'a>}

impl<'input> CastExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CastExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CastExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CastExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<CastExpressionContextExt<'input>>{

fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> CastExpressionContextAttrs<'input> for CastExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn castExpression(&mut self,)
	-> Result<Rc<CastExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CastExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_castExpression);
        let mut _localctx: Rc<CastExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(202);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule unaryExpression*/
					recog.base.set_state(200);
					recog.unaryExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(201);
					recog.base.match_token(Number,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiplicativeExpression ----------------
pub type MultiplicativeExpressionContextAll<'input> = MultiplicativeExpressionContext<'input>;


pub type MultiplicativeExpressionContext<'input> = BaseParserRuleContext<'input,MultiplicativeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicativeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for MultiplicativeExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_multiplicativeExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_multiplicativeExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for MultiplicativeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_multiplicativeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeExpression }
}
antlr_rust::type_id!{MultiplicativeExpressionContextExt<'a>}

impl<'input> MultiplicativeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiplicativeExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicativeExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplicativeExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<MultiplicativeExpressionContextExt<'input>>{

fn castExpression_all(&self) ->  Vec<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn castExpression(&self, i: usize) -> Option<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Star in current rule
fn Star_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Star, starting from 0.
/// Returns `None` if number of children corresponding to token Star is less or equal than `i`.
fn Star(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Star, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Slash in current rule
fn Slash_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Slash, starting from 0.
/// Returns `None` if number of children corresponding to token Slash is less or equal than `i`.
fn Slash(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Slash, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Percent in current rule
fn Percent_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Percent, starting from 0.
/// Returns `None` if number of children corresponding to token Percent is less or equal than `i`.
fn Percent(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Percent, i)
}

}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiplicativeExpression(&mut self,)
	-> Result<Rc<MultiplicativeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicativeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_multiplicativeExpression);
        let mut _localctx: Rc<MultiplicativeExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule castExpression*/
			recog.base.set_state(204);
			recog.castExpression()?;

			recog.base.set_state(209);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(205);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << Percent) | (1usize << Star) | (1usize << Slash))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule castExpression*/
					recog.base.set_state(206);
					recog.castExpression()?;

					}
					} 
				}
				recog.base.set_state(211);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- additiveExpression ----------------
pub type AdditiveExpressionContextAll<'input> = AdditiveExpressionContext<'input>;


pub type AdditiveExpressionContext<'input> = BaseParserRuleContext<'input,AdditiveExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for AdditiveExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for AdditiveExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_additiveExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_additiveExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for AdditiveExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_additiveExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveExpression }
}
antlr_rust::type_id!{AdditiveExpressionContextExt<'a>}

impl<'input> AdditiveExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AdditiveExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AdditiveExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<AdditiveExpressionContextExt<'input>>{

fn multiplicativeExpression_all(&self) ->  Vec<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplicativeExpression(&self, i: usize) -> Option<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Plus in current rule
fn Plus_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Plus, starting from 0.
/// Returns `None` if number of children corresponding to token Plus is less or equal than `i`.
fn Plus(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Plus, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Minus in current rule
fn Minus_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Minus, starting from 0.
/// Returns `None` if number of children corresponding to token Minus is less or equal than `i`.
fn Minus(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Minus, i)
}

}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn additiveExpression(&mut self,)
	-> Result<Rc<AdditiveExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditiveExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_additiveExpression);
        let mut _localctx: Rc<AdditiveExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule multiplicativeExpression*/
			recog.base.set_state(212);
			recog.multiplicativeExpression()?;

			recog.base.set_state(217);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(213);
					_la = recog.base.input.la(1);
					if { !(_la==Minus || _la==Plus) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule multiplicativeExpression*/
					recog.base.set_state(214);
					recog.multiplicativeExpression()?;

					}
					} 
				}
				recog.base.set_state(219);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shiftExpression ----------------
pub type ShiftExpressionContextAll<'input> = ShiftExpressionContext<'input>;


pub type ShiftExpressionContext<'input> = BaseParserRuleContext<'input,ShiftExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ShiftExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ShiftExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ShiftExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shiftExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_shiftExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ShiftExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_shiftExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShiftExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shiftExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shiftExpression }
}
antlr_rust::type_id!{ShiftExpressionContextExt<'a>}

impl<'input> ShiftExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShiftExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShiftExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShiftExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ShiftExpressionContextExt<'input>>{

fn additiveExpression_all(&self) ->  Vec<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn additiveExpression(&self, i: usize) -> Option<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DoubleLeftCaret in current rule
fn DoubleLeftCaret_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DoubleLeftCaret, starting from 0.
/// Returns `None` if number of children corresponding to token DoubleLeftCaret is less or equal than `i`.
fn DoubleLeftCaret(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleLeftCaret, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DoubleRightCaret in current rule
fn DoubleRightCaret_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DoubleRightCaret, starting from 0.
/// Returns `None` if number of children corresponding to token DoubleRightCaret is less or equal than `i`.
fn DoubleRightCaret(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleRightCaret, i)
}

}

impl<'input> ShiftExpressionContextAttrs<'input> for ShiftExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shiftExpression(&mut self,)
	-> Result<Rc<ShiftExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShiftExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_shiftExpression);
        let mut _localctx: Rc<ShiftExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule additiveExpression*/
			recog.base.set_state(220);
			recog.additiveExpression()?;

			recog.base.set_state(225);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(221);
					_la = recog.base.input.la(1);
					if { !(_la==DoubleLeftCaret || _la==DoubleRightCaret) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule additiveExpression*/
					recog.base.set_state(222);
					recog.additiveExpression()?;

					}
					} 
				}
				recog.base.set_state(227);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relationalExpression ----------------
pub type RelationalExpressionContextAll<'input> = RelationalExpressionContext<'input>;


pub type RelationalExpressionContext<'input> = BaseParserRuleContext<'input,RelationalExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct RelationalExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for RelationalExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for RelationalExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationalExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_relationalExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for RelationalExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_relationalExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationalExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relationalExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relationalExpression }
}
antlr_rust::type_id!{RelationalExpressionContextExt<'a>}

impl<'input> RelationalExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationalExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationalExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationalExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<RelationalExpressionContextExt<'input>>{

fn shiftExpression_all(&self) ->  Vec<Rc<ShiftExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shiftExpression(&self, i: usize) -> Option<Rc<ShiftExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftCaret in current rule
fn LeftCaret_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftCaret, starting from 0.
/// Returns `None` if number of children corresponding to token LeftCaret is less or equal than `i`.
fn LeftCaret(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftCaret, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightCaret in current rule
fn RightCaret_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightCaret, starting from 0.
/// Returns `None` if number of children corresponding to token RightCaret is less or equal than `i`.
fn RightCaret(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightCaret, i)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftCaretEqual in current rule
fn LeftCaretEqual_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftCaretEqual, starting from 0.
/// Returns `None` if number of children corresponding to token LeftCaretEqual is less or equal than `i`.
fn LeftCaretEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftCaretEqual, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightCaretEqual in current rule
fn RightCaretEqual_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightCaretEqual, starting from 0.
/// Returns `None` if number of children corresponding to token RightCaretEqual is less or equal than `i`.
fn RightCaretEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightCaretEqual, i)
}

}

impl<'input> RelationalExpressionContextAttrs<'input> for RelationalExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relationalExpression(&mut self,)
	-> Result<Rc<RelationalExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationalExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_relationalExpression);
        let mut _localctx: Rc<RelationalExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule shiftExpression*/
			recog.base.set_state(228);
			recog.shiftExpression()?;

			recog.base.set_state(233);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(229);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftCaret) | (1usize << LeftCaretEqual) | (1usize << RightCaret) | (1usize << RightCaretEqual))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule shiftExpression*/
					recog.base.set_state(230);
					recog.shiftExpression()?;

					}
					} 
				}
				recog.base.set_state(235);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- equalityExpression ----------------
pub type EqualityExpressionContextAll<'input> = EqualityExpressionContext<'input>;


pub type EqualityExpressionContext<'input> = BaseParserRuleContext<'input,EqualityExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for EqualityExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for EqualityExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_equalityExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_equalityExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for EqualityExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_equalityExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityExpression }
}
antlr_rust::type_id!{EqualityExpressionContextExt<'a>}

impl<'input> EqualityExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqualityExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<EqualityExpressionContextExt<'input>>{

fn relationalExpression_all(&self) ->  Vec<Rc<RelationalExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relationalExpression(&self, i: usize) -> Option<Rc<RelationalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DoubleEqual in current rule
fn DoubleEqual_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DoubleEqual, starting from 0.
/// Returns `None` if number of children corresponding to token DoubleEqual is less or equal than `i`.
fn DoubleEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleEqual, i)
}
/// Retrieves all `TerminalNode`s corresponding to token BangEqual in current rule
fn BangEqual_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BangEqual, starting from 0.
/// Returns `None` if number of children corresponding to token BangEqual is less or equal than `i`.
fn BangEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(BangEqual, i)
}

}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equalityExpression(&mut self,)
	-> Result<Rc<EqualityExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_equalityExpression);
        let mut _localctx: Rc<EqualityExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relationalExpression*/
			recog.base.set_state(236);
			recog.relationalExpression()?;

			recog.base.set_state(241);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(237);
					_la = recog.base.input.la(1);
					if { !(_la==BangEqual || _la==DoubleEqual) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule relationalExpression*/
					recog.base.set_state(238);
					recog.relationalExpression()?;

					}
					} 
				}
				recog.base.set_state(243);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- andExpression ----------------
pub type AndExpressionContextAll<'input> = AndExpressionContext<'input>;


pub type AndExpressionContext<'input> = BaseParserRuleContext<'input,AndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for AndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for AndExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_andExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_andExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for AndExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_andExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_andExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_andExpression }
}
antlr_rust::type_id!{AndExpressionContextExt<'a>}

impl<'input> AndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AndExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AndExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<AndExpressionContextExt<'input>>{

fn equalityExpression_all(&self) ->  Vec<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equalityExpression(&self, i: usize) -> Option<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LogicalAnd in current rule
fn LogicalAnd_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LogicalAnd, starting from 0.
/// Returns `None` if number of children corresponding to token LogicalAnd is less or equal than `i`.
fn LogicalAnd(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LogicalAnd, i)
}

}

impl<'input> AndExpressionContextAttrs<'input> for AndExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn andExpression(&mut self,)
	-> Result<Rc<AndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_andExpression);
        let mut _localctx: Rc<AndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule equalityExpression*/
			recog.base.set_state(244);
			recog.equalityExpression()?;

			recog.base.set_state(249);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(245);
					recog.base.match_token(LogicalAnd,&mut recog.err_handler)?;

					/*InvokeRule equalityExpression*/
					recog.base.set_state(246);
					recog.equalityExpression()?;

					}
					} 
				}
				recog.base.set_state(251);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exclusiveOrExpression ----------------
pub type ExclusiveOrExpressionContextAll<'input> = ExclusiveOrExpressionContext<'input>;


pub type ExclusiveOrExpressionContext<'input> = BaseParserRuleContext<'input,ExclusiveOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExclusiveOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ExclusiveOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ExclusiveOrExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_exclusiveOrExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_exclusiveOrExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ExclusiveOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_exclusiveOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExclusiveOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exclusiveOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exclusiveOrExpression }
}
antlr_rust::type_id!{ExclusiveOrExpressionContextExt<'a>}

impl<'input> ExclusiveOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExclusiveOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExclusiveOrExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExclusiveOrExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ExclusiveOrExpressionContextExt<'input>>{

fn andExpression_all(&self) ->  Vec<Rc<AndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn andExpression(&self, i: usize) -> Option<Rc<AndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExclusiveOrExpressionContextAttrs<'input> for ExclusiveOrExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exclusiveOrExpression(&mut self,)
	-> Result<Rc<ExclusiveOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExclusiveOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_exclusiveOrExpression);
        let mut _localctx: Rc<ExclusiveOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule andExpression*/
			recog.base.set_state(252);
			recog.andExpression()?;

			recog.base.set_state(257);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(253);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule andExpression*/
					recog.base.set_state(254);
					recog.andExpression()?;

					}
					} 
				}
				recog.base.set_state(259);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inclusiveOrExpression ----------------
pub type InclusiveOrExpressionContextAll<'input> = InclusiveOrExpressionContext<'input>;


pub type InclusiveOrExpressionContext<'input> = BaseParserRuleContext<'input,InclusiveOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct InclusiveOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for InclusiveOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for InclusiveOrExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inclusiveOrExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_inclusiveOrExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for InclusiveOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_inclusiveOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for InclusiveOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inclusiveOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inclusiveOrExpression }
}
antlr_rust::type_id!{InclusiveOrExpressionContextExt<'a>}

impl<'input> InclusiveOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InclusiveOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InclusiveOrExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InclusiveOrExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<InclusiveOrExpressionContextExt<'input>>{

fn exclusiveOrExpression_all(&self) ->  Vec<Rc<ExclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn exclusiveOrExpression(&self, i: usize) -> Option<Rc<ExclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LogicalOr in current rule
fn LogicalOr_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LogicalOr, starting from 0.
/// Returns `None` if number of children corresponding to token LogicalOr is less or equal than `i`.
fn LogicalOr(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LogicalOr, i)
}

}

impl<'input> InclusiveOrExpressionContextAttrs<'input> for InclusiveOrExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inclusiveOrExpression(&mut self,)
	-> Result<Rc<InclusiveOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InclusiveOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_inclusiveOrExpression);
        let mut _localctx: Rc<InclusiveOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule exclusiveOrExpression*/
			recog.base.set_state(260);
			recog.exclusiveOrExpression()?;

			recog.base.set_state(265);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(261);
					recog.base.match_token(LogicalOr,&mut recog.err_handler)?;

					/*InvokeRule exclusiveOrExpression*/
					recog.base.set_state(262);
					recog.exclusiveOrExpression()?;

					}
					} 
				}
				recog.base.set_state(267);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logicalAndExpression ----------------
pub type LogicalAndExpressionContextAll<'input> = LogicalAndExpressionContext<'input>;


pub type LogicalAndExpressionContext<'input> = BaseParserRuleContext<'input,LogicalAndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalAndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for LogicalAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for LogicalAndExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_logicalAndExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_logicalAndExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for LogicalAndExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_logicalAndExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalAndExpression }
}
antlr_rust::type_id!{LogicalAndExpressionContextExt<'a>}

impl<'input> LogicalAndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicalAndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalAndExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LogicalAndExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<LogicalAndExpressionContextExt<'input>>{

fn inclusiveOrExpression_all(&self) ->  Vec<Rc<InclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inclusiveOrExpression(&self, i: usize) -> Option<Rc<InclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AndOp in current rule
fn AndOp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AndOp, starting from 0.
/// Returns `None` if number of children corresponding to token AndOp is less or equal than `i`.
fn AndOp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(AndOp, i)
}

}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicalAndExpression(&mut self,)
	-> Result<Rc<LogicalAndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalAndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_logicalAndExpression);
        let mut _localctx: Rc<LogicalAndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inclusiveOrExpression*/
			recog.base.set_state(268);
			recog.inclusiveOrExpression()?;

			recog.base.set_state(273);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(269);
					recog.base.match_token(AndOp,&mut recog.err_handler)?;

					/*InvokeRule inclusiveOrExpression*/
					recog.base.set_state(270);
					recog.inclusiveOrExpression()?;

					}
					} 
				}
				recog.base.set_state(275);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logicalOrExpression ----------------
pub type LogicalOrExpressionContextAll<'input> = LogicalOrExpressionContext<'input>;


pub type LogicalOrExpressionContext<'input> = BaseParserRuleContext<'input,LogicalOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for LogicalOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for LogicalOrExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_logicalOrExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_logicalOrExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for LogicalOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_logicalOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOrExpression }
}
antlr_rust::type_id!{LogicalOrExpressionContextExt<'a>}

impl<'input> LogicalOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicalOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalOrExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LogicalOrExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<LogicalOrExpressionContextExt<'input>>{

fn logicalAndExpression_all(&self) ->  Vec<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logicalAndExpression(&self, i: usize) -> Option<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OrOp in current rule
fn OrOp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OrOp, starting from 0.
/// Returns `None` if number of children corresponding to token OrOp is less or equal than `i`.
fn OrOp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(OrOp, i)
}

}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicalOrExpression(&mut self,)
	-> Result<Rc<LogicalOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_logicalOrExpression);
        let mut _localctx: Rc<LogicalOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logicalAndExpression*/
			recog.base.set_state(276);
			recog.logicalAndExpression()?;

			recog.base.set_state(281);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(277);
					recog.base.match_token(OrOp,&mut recog.err_handler)?;

					/*InvokeRule logicalAndExpression*/
					recog.base.set_state(278);
					recog.logicalAndExpression()?;

					}
					} 
				}
				recog.base.set_state(283);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- conditionalExpression ----------------
pub type ConditionalExpressionContextAll<'input> = ConditionalExpressionContext<'input>;


pub type ConditionalExpressionContext<'input> = BaseParserRuleContext<'input,ConditionalExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ConditionalExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ConditionalExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_conditionalExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_conditionalExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ConditionalExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_conditionalExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalExpression }
}
antlr_rust::type_id!{ConditionalExpressionContextExt<'a>}

impl<'input> ConditionalExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionalExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ConditionalExpressionContextExt<'input>>{

fn logicalOrExpression(&self) -> Option<Rc<LogicalOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QuestionMark
/// Returns `None` if there is no child corresponding to token QuestionMark
fn QuestionMark(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(QuestionMark, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Colon, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConditionalExpressionContextAttrs<'input> for ConditionalExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditionalExpression(&mut self,)
	-> Result<Rc<ConditionalExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_conditionalExpression);
        let mut _localctx: Rc<ConditionalExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logicalOrExpression*/
			recog.base.set_state(284);
			recog.logicalOrExpression()?;

			recog.base.set_state(290);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(285);
					recog.base.match_token(QuestionMark,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(286);
					recog.expression()?;

					recog.base.set_state(287);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(288);
					recog.conditionalExpression()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignmentExpression ----------------
pub type AssignmentExpressionContextAll<'input> = AssignmentExpressionContext<'input>;


pub type AssignmentExpressionContext<'input> = BaseParserRuleContext<'input,AssignmentExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for AssignmentExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for AssignmentExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assignmentExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_assignmentExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for AssignmentExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_assignmentExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentExpression }
}
antlr_rust::type_id!{AssignmentExpressionContextExt<'a>}

impl<'input> AssignmentExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<AssignmentExpressionContextExt<'input>>{

fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentLeftExpression(&self) -> Option<Rc<AssignmentLeftExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentOperator(&self) -> Option<Rc<AssignmentOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> AssignmentExpressionContextAttrs<'input> for AssignmentExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignmentExpression(&mut self,)
	-> Result<Rc<AssignmentExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_assignmentExpression);
        let mut _localctx: Rc<AssignmentExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(298);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(292);
					recog.conditionalExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assignmentLeftExpression*/
					recog.base.set_state(293);
					recog.assignmentLeftExpression()?;

					/*InvokeRule assignmentOperator*/
					recog.base.set_state(294);
					recog.assignmentOperator()?;

					/*InvokeRule assignmentExpression*/
					recog.base.set_state(295);
					recog.assignmentExpression()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(297);
					recog.base.match_token(Number,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignmentLeftExpression ----------------
pub type AssignmentLeftExpressionContextAll<'input> = AssignmentLeftExpressionContext<'input>;


pub type AssignmentLeftExpressionContext<'input> = BaseParserRuleContext<'input,AssignmentLeftExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentLeftExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for AssignmentLeftExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for AssignmentLeftExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assignmentLeftExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_assignmentLeftExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for AssignmentLeftExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_assignmentLeftExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentLeftExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentLeftExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentLeftExpression }
}
antlr_rust::type_id!{AssignmentLeftExpressionContextExt<'a>}

impl<'input> AssignmentLeftExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentLeftExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentLeftExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentLeftExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<AssignmentLeftExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignmentLeftExpressionContextAttrs<'input> for AssignmentLeftExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignmentLeftExpression(&mut self,)
	-> Result<Rc<AssignmentLeftExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentLeftExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_assignmentLeftExpression);
        let mut _localctx: Rc<AssignmentLeftExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(302);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(300);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(301);
					recog.variable()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignmentOperator ----------------
pub type AssignmentOperatorContextAll<'input> = AssignmentOperatorContext<'input>;


pub type AssignmentOperatorContext<'input> = BaseParserRuleContext<'input,AssignmentOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for AssignmentOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for AssignmentOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assignmentOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_assignmentOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for AssignmentOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_assignmentOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentOperator }
}
antlr_rust::type_id!{AssignmentOperatorContextExt<'a>}

impl<'input> AssignmentOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<AssignmentOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Equal
/// Returns `None` if there is no child corresponding to token Equal
fn Equal(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Equal, 0)
}

}

impl<'input> AssignmentOperatorContextAttrs<'input> for AssignmentOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignmentOperator(&mut self,)
	-> Result<Rc<AssignmentOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_assignmentOperator);
        let mut _localctx: Rc<AssignmentOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(304);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__1) | (1usize << T__2) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10) | (1usize << Equal))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::type_id!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn assignmentExpression_all(&self) ->  Vec<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentExpression(&self, i: usize) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule assignmentExpression*/
			recog.base.set_state(306);
			recog.assignmentExpression()?;

			recog.base.set_state(311);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(307);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule assignmentExpression*/
					recog.base.set_state(308);
					recog.assignmentExpression()?;

					}
					} 
				}
				recog.base.set_state(313);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constantExpression ----------------
pub type ConstantExpressionContextAll<'input> = ConstantExpressionContext<'input>;


pub type ConstantExpressionContext<'input> = BaseParserRuleContext<'input,ConstantExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ConstantExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ConstantExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_constantExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_constantExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ConstantExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_constantExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constantExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constantExpression }
}
antlr_rust::type_id!{ConstantExpressionContextExt<'a>}

impl<'input> ConstantExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ConstantExpressionContextExt<'input>>{

fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConstantExpressionContextAttrs<'input> for ConstantExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constantExpression(&mut self,)
	-> Result<Rc<ConstantExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_constantExpression);
        let mut _localctx: Rc<ConstantExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalExpression*/
			recog.base.set_state(314);
			recog.conditionalExpression()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declaration ----------------
pub type DeclarationContextAll<'input> = DeclarationContext<'input>;


pub type DeclarationContext<'input> = BaseParserRuleContext<'input,DeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declaration(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_declaration(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_declaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declaration }
}
antlr_rust::type_id!{DeclarationContextExt<'a>}

impl<'input> DeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}
fn initDeclaratorList(&self) -> Option<Rc<InitDeclaratorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declaration(&mut self,)
	-> Result<Rc<DeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_declaration);
        let mut _localctx: Rc<DeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(317);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Quote - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Function - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)))) != 0) {
				{
				/*InvokeRule initDeclaratorList*/
				recog.base.set_state(316);
				recog.initDeclaratorList()?;

				}
			}

			recog.base.set_state(319);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declarationSpecifiers ----------------
pub type DeclarationSpecifiersContextAll<'input> = DeclarationSpecifiersContext<'input>;


pub type DeclarationSpecifiersContext<'input> = BaseParserRuleContext<'input,DeclarationSpecifiersContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationSpecifiersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DeclarationSpecifiersContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DeclarationSpecifiersContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declarationSpecifiers(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_declarationSpecifiers(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DeclarationSpecifiersContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_declarationSpecifiers(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationSpecifiersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationSpecifiers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationSpecifiers }
}
antlr_rust::type_id!{DeclarationSpecifiersContextExt<'a>}

impl<'input> DeclarationSpecifiersContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationSpecifiersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationSpecifiersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationSpecifiersContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DeclarationSpecifiersContextExt<'input>>{

fn declarationSpecifier_all(&self) ->  Vec<Rc<DeclarationSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declarationSpecifier(&self, i: usize) -> Option<Rc<DeclarationSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DeclarationSpecifiersContextAttrs<'input> for DeclarationSpecifiersContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declarationSpecifiers(&mut self,)
	-> Result<Rc<DeclarationSpecifiersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationSpecifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_declarationSpecifiers);
        let mut _localctx: Rc<DeclarationSpecifiersContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(322); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule declarationSpecifier*/
					recog.base.set_state(321);
					recog.declarationSpecifier()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(324); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declarationSpecifiers2 ----------------
pub type DeclarationSpecifiers2ContextAll<'input> = DeclarationSpecifiers2Context<'input>;


pub type DeclarationSpecifiers2Context<'input> = BaseParserRuleContext<'input,DeclarationSpecifiers2ContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationSpecifiers2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DeclarationSpecifiers2Context<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DeclarationSpecifiers2Context<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declarationSpecifiers2(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_declarationSpecifiers2(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DeclarationSpecifiers2Context<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_declarationSpecifiers2(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationSpecifiers2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationSpecifiers2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationSpecifiers2 }
}
antlr_rust::type_id!{DeclarationSpecifiers2ContextExt<'a>}

impl<'input> DeclarationSpecifiers2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationSpecifiers2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationSpecifiers2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationSpecifiers2ContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DeclarationSpecifiers2ContextExt<'input>>{

fn scope_specifier(&self) -> Option<Rc<Scope_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationSpecifiers2ContextAttrs<'input> for DeclarationSpecifiers2Context<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declarationSpecifiers2(&mut self,)
	-> Result<Rc<DeclarationSpecifiers2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationSpecifiers2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_declarationSpecifiers2);
        let mut _localctx: Rc<DeclarationSpecifiers2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule scope_specifier*/
			recog.base.set_state(326);
			recog.scope_specifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declarationSpecifier ----------------
pub type DeclarationSpecifierContextAll<'input> = DeclarationSpecifierContext<'input>;


pub type DeclarationSpecifierContext<'input> = BaseParserRuleContext<'input,DeclarationSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DeclarationSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DeclarationSpecifierContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declarationSpecifier(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_declarationSpecifier(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DeclarationSpecifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_declarationSpecifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationSpecifier }
}
antlr_rust::type_id!{DeclarationSpecifierContextExt<'a>}

impl<'input> DeclarationSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationSpecifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationSpecifierContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DeclarationSpecifierContextExt<'input>>{

fn scope_specifier(&self) -> Option<Rc<Scope_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationSpecifierContextAttrs<'input> for DeclarationSpecifierContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declarationSpecifier(&mut self,)
	-> Result<Rc<DeclarationSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_declarationSpecifier);
        let mut _localctx: Rc<DeclarationSpecifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule scope_specifier*/
			recog.base.set_state(328);
			recog.scope_specifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- initDeclaratorList ----------------
pub type InitDeclaratorListContextAll<'input> = InitDeclaratorListContext<'input>;


pub type InitDeclaratorListContext<'input> = BaseParserRuleContext<'input,InitDeclaratorListContextExt<'input>>;

#[derive(Clone)]
pub struct InitDeclaratorListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for InitDeclaratorListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for InitDeclaratorListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_initDeclaratorList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_initDeclaratorList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for InitDeclaratorListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_initDeclaratorList(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitDeclaratorListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initDeclaratorList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initDeclaratorList }
}
antlr_rust::type_id!{InitDeclaratorListContextExt<'a>}

impl<'input> InitDeclaratorListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitDeclaratorListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitDeclaratorListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitDeclaratorListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<InitDeclaratorListContextExt<'input>>{

fn initDeclarator_all(&self) ->  Vec<Rc<InitDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn initDeclarator(&self, i: usize) -> Option<Rc<InitDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> InitDeclaratorListContextAttrs<'input> for InitDeclaratorListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initDeclaratorList(&mut self,)
	-> Result<Rc<InitDeclaratorListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitDeclaratorListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_initDeclaratorList);
        let mut _localctx: Rc<InitDeclaratorListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule initDeclarator*/
			recog.base.set_state(330);
			recog.initDeclarator()?;

			recog.base.set_state(335);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(331);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				/*InvokeRule initDeclarator*/
				recog.base.set_state(332);
				recog.initDeclarator()?;

				}
				}
				recog.base.set_state(337);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- initDeclarator ----------------
pub type InitDeclaratorContextAll<'input> = InitDeclaratorContext<'input>;


pub type InitDeclaratorContext<'input> = BaseParserRuleContext<'input,InitDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct InitDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for InitDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for InitDeclaratorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_initDeclarator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_initDeclarator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for InitDeclaratorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_initDeclarator(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initDeclarator }
}
antlr_rust::type_id!{InitDeclaratorContextExt<'a>}

impl<'input> InitDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitDeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitDeclaratorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<InitDeclaratorContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Equal
/// Returns `None` if there is no child corresponding to token Equal
fn Equal(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Equal, 0)
}
fn initializer(&self) -> Option<Rc<InitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Function
/// Returns `None` if there is no child corresponding to token Function
fn Function(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Function, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> InitDeclaratorContextAttrs<'input> for InitDeclaratorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initDeclarator(&mut self,)
	-> Result<Rc<InitDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_initDeclarator);
        let mut _localctx: Rc<InitDeclaratorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(345);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 At | Quote | Sharp | DoubleSharp | Dot | DotAt | Dollar | DollarAt |
			 Menu | Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule variable*/
					recog.base.set_state(338);
					recog.variable()?;

					recog.base.set_state(341);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Equal {
						{
						recog.base.set_state(339);
						recog.base.match_token(Equal,&mut recog.err_handler)?;

						/*InvokeRule initializer*/
						recog.base.set_state(340);
						recog.initializer()?;

						}
					}

					}
				}

			 Function 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(343);
					recog.base.match_token(Function,&mut recog.err_handler)?;

					recog.base.set_state(344);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- specifierQualifierList ----------------
pub type SpecifierQualifierListContextAll<'input> = SpecifierQualifierListContext<'input>;


pub type SpecifierQualifierListContext<'input> = BaseParserRuleContext<'input,SpecifierQualifierListContextExt<'input>>;

#[derive(Clone)]
pub struct SpecifierQualifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SpecifierQualifierListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SpecifierQualifierListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_specifierQualifierList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_specifierQualifierList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SpecifierQualifierListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_specifierQualifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for SpecifierQualifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_specifierQualifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_specifierQualifierList }
}
antlr_rust::type_id!{SpecifierQualifierListContextExt<'a>}

impl<'input> SpecifierQualifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SpecifierQualifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SpecifierQualifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SpecifierQualifierListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SpecifierQualifierListContextExt<'input>>{

fn scope_specifier(&self) -> Option<Rc<Scope_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn specifierQualifierList(&self) -> Option<Rc<SpecifierQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SpecifierQualifierListContextAttrs<'input> for SpecifierQualifierListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn specifierQualifierList(&mut self,)
	-> Result<Rc<SpecifierQualifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SpecifierQualifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_specifierQualifierList);
        let mut _localctx: Rc<SpecifierQualifierListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule scope_specifier*/
			recog.base.set_state(347);
			recog.scope_specifier()?;

			}
			recog.base.set_state(349);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule specifierQualifierList*/
					recog.base.set_state(348);
					recog.specifierQualifierList()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declarator ----------------
pub type DeclaratorContextAll<'input> = DeclaratorContext<'input>;


pub type DeclaratorContext<'input> = BaseParserRuleContext<'input,DeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct DeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DeclaratorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declarator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_declarator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DeclaratorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_declarator(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarator }
}
antlr_rust::type_id!{DeclaratorContextExt<'a>}

impl<'input> DeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclaratorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DeclaratorContextExt<'input>>{

fn directDeclarator(&self) -> Option<Rc<DirectDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclaratorContextAttrs<'input> for DeclaratorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declarator(&mut self,)
	-> Result<Rc<DeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_declarator);
        let mut _localctx: Rc<DeclaratorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule directDeclarator*/
			recog.base.set_state(351);
			recog.directDeclarator_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- directDeclarator ----------------
pub type DirectDeclaratorContextAll<'input> = DirectDeclaratorContext<'input>;


pub type DirectDeclaratorContext<'input> = BaseParserRuleContext<'input,DirectDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct DirectDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DirectDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DirectDeclaratorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_directDeclarator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_directDeclarator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DirectDeclaratorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_directDeclarator(self);
	}
}

impl<'input> CustomRuleContext<'input> for DirectDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_directDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_directDeclarator }
}
antlr_rust::type_id!{DirectDeclaratorContextExt<'a>}

impl<'input> DirectDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DirectDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DirectDeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DirectDeclaratorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DirectDeclaratorContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn directDeclarator(&self) -> Option<Rc<DirectDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Star, 0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBracket, 0)
}
fn parameterTypeList(&self) -> Option<Rc<ParameterTypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DirectDeclaratorContextAttrs<'input> for DirectDeclaratorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  directDeclarator(&mut self,)
	-> Result<Rc<DirectDeclaratorContextAll<'input>>,ANTLRError> {
		self.directDeclarator_rec(0)
	}

	fn directDeclarator_rec(&mut self, _p: isize)
	-> Result<Rc<DirectDeclaratorContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = DirectDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 64, RULE_directDeclarator, _p);
	    let mut _localctx: Rc<DirectDeclaratorContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 64;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(363);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule variable*/
					recog.base.set_state(354);
					recog.variable()?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(355);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule declarator*/
					recog.base.set_state(356);
					recog.declarator()?;

					recog.base.set_state(357);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(359);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule directDeclarator*/
					recog.base.set_state(360);
					recog.directDeclarator_rec(0)?;

					recog.base.set_state(361);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(382);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(33,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(380);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator);
							_localctx = tmp;
							recog.base.set_state(365);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(366);
							recog.base.match_token(LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(367);
							recog.base.match_token(Star,&mut recog.err_handler)?;

							recog.base.set_state(368);
							recog.base.match_token(RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator);
							_localctx = tmp;
							recog.base.set_state(369);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(370);
							recog.base.match_token(LeftParen,&mut recog.err_handler)?;

							/*InvokeRule parameterTypeList*/
							recog.base.set_state(371);
							recog.parameterTypeList()?;

							recog.base.set_state(372);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator);
							_localctx = tmp;
							recog.base.set_state(374);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(375);
							recog.base.match_token(LeftParen,&mut recog.err_handler)?;

							recog.base.set_state(377);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Identifier {
								{
								/*InvokeRule identifierList*/
								recog.base.set_state(376);
								recog.identifierList()?;

								}
							}

							recog.base.set_state(379);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(384);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(33,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- nestedParenthesesBlock ----------------
pub type NestedParenthesesBlockContextAll<'input> = NestedParenthesesBlockContext<'input>;


pub type NestedParenthesesBlockContext<'input> = BaseParserRuleContext<'input,NestedParenthesesBlockContextExt<'input>>;

#[derive(Clone)]
pub struct NestedParenthesesBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for NestedParenthesesBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for NestedParenthesesBlockContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nestedParenthesesBlock(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_nestedParenthesesBlock(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for NestedParenthesesBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_nestedParenthesesBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for NestedParenthesesBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nestedParenthesesBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nestedParenthesesBlock }
}
antlr_rust::type_id!{NestedParenthesesBlockContextExt<'a>}

impl<'input> NestedParenthesesBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NestedParenthesesBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NestedParenthesesBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NestedParenthesesBlockContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<NestedParenthesesBlockContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token LeftParen in current rule
fn LeftParen_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftParen, starting from 0.
/// Returns `None` if number of children corresponding to token LeftParen is less or equal than `i`.
fn LeftParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, i)
}
fn nestedParenthesesBlock_all(&self) ->  Vec<Rc<NestedParenthesesBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn nestedParenthesesBlock(&self, i: usize) -> Option<Rc<NestedParenthesesBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightParen in current rule
fn RightParen_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightParen, starting from 0.
/// Returns `None` if number of children corresponding to token RightParen is less or equal than `i`.
fn RightParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, i)
}

}

impl<'input> NestedParenthesesBlockContextAttrs<'input> for NestedParenthesesBlockContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nestedParenthesesBlock(&mut self,)
	-> Result<Rc<NestedParenthesesBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NestedParenthesesBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_nestedParenthesesBlock);
        let mut _localctx: Rc<NestedParenthesesBlockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(392);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10) | (1usize << T__11) | (1usize << T__12) | (1usize << T__13) | (1usize << T__14) | (1usize << LeftParen) | (1usize << LeftBrace) | (1usize << RightBrace) | (1usize << LeftBracket) | (1usize << RightBracket) | (1usize << Comma) | (1usize << At) | (1usize << Colon) | (1usize << SemiColon) | (1usize << Percent) | (1usize << Star) | (1usize << Tilde) | (1usize << QuestionMark) | (1usize << Quote) | (1usize << DoubleQuote) | (1usize << LogicalOr) | (1usize << OrOp) | (1usize << LogicalAnd) | (1usize << AndOp) | (1usize << Slash) | (1usize << SlashStar) | (1usize << StarSlash) | (1usize << DoubleSlash) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << BangEqual) | (1usize << Equal) | (1usize << DoubleEqual) | (1usize << LeftCaret) | (1usize << DoubleLeftCaret) | (1usize << LeftCaretEqual) | (1usize << RightCaret) | (1usize << DoubleRightCaret) | (1usize << RightCaretEqual) | (1usize << If) | (1usize << Else) | (1usize << End) | (1usize << Set))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (For - 64)) | (1usize << (While - 64)) | (1usize << (Do - 64)) | (1usize << (Goto - 64)) | (1usize << (Return - 64)) | (1usize << (Switch - 64)) | (1usize << (Case - 64)) | (1usize << (Function - 64)) | (1usize << (Break - 64)) | (1usize << (SetArray - 64)) | (1usize << (GetArraySize - 64)) | (1usize << (Close - 64)) | (1usize << (Close2 - 64)) | (1usize << (Next - 64)) | (1usize << (Menu - 64)) | (1usize << (Eof - 64)) | (1usize << (Identifier - 64)) | (1usize << (Label - 64)) | (1usize << (String - 64)) | (1usize << (Number - 64)) | (1usize << (Whitespace - 64)) | (1usize << (Newline - 64)) | (1usize << (BlockComment - 64)) | (1usize << (LineComment - 64)))) != 0) {
				{
				recog.base.set_state(390);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 T__0 | T__1 | T__2 | T__3 | T__4 | T__5 | T__6 | T__7 | T__8 | T__9 |
				 T__10 | T__11 | T__12 | T__13 | T__14 | LeftBrace | RightBrace | LeftBracket |
				 RightBracket | Comma | At | Colon | SemiColon | Percent | Star | Tilde |
				 QuestionMark | Quote | DoubleQuote | LogicalOr | OrOp | LogicalAnd |
				 AndOp | Slash | SlashStar | StarSlash | DoubleSlash | Sharp | DoubleSharp |
				 Minus | DecrementOp | Plus | IncrementOp | Dot | DotAt | Dollar | DollarAt |
				 Bang | BangEqual | Equal | DoubleEqual | LeftCaret | DoubleLeftCaret |
				 LeftCaretEqual | RightCaret | DoubleRightCaret | RightCaretEqual | If |
				 Else | End | Set | For | While | Do | Goto | Return | Switch | Case |
				 Function | Break | SetArray | GetArraySize | Close | Close2 | Next |
				 Menu | Eof | Identifier | Label | String | Number | Whitespace | Newline |
				 BlockComment | LineComment 
					=> {
						{
						recog.base.set_state(385);
						_la = recog.base.input.la(1);
						if { _la <= 0 || (_la==LeftParen || _la==RightParen) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

				 LeftParen 
					=> {
						{
						recog.base.set_state(386);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						/*InvokeRule nestedParenthesesBlock*/
						recog.base.set_state(387);
						recog.nestedParenthesesBlock()?;

						recog.base.set_state(388);
						recog.base.match_token(RightParen,&mut recog.err_handler)?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(394);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameterTypeList ----------------
pub type ParameterTypeListContextAll<'input> = ParameterTypeListContext<'input>;


pub type ParameterTypeListContext<'input> = BaseParserRuleContext<'input,ParameterTypeListContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterTypeListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ParameterTypeListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ParameterTypeListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parameterTypeList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_parameterTypeList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ParameterTypeListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_parameterTypeList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterTypeListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterTypeList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterTypeList }
}
antlr_rust::type_id!{ParameterTypeListContextExt<'a>}

impl<'input> ParameterTypeListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterTypeListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterTypeListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterTypeListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ParameterTypeListContextExt<'input>>{

fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}

}

impl<'input> ParameterTypeListContextAttrs<'input> for ParameterTypeListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterTypeList(&mut self,)
	-> Result<Rc<ParameterTypeListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterTypeListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_parameterTypeList);
        let mut _localctx: Rc<ParameterTypeListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule parameterList*/
			recog.base.set_state(395);
			recog.parameterList()?;

			recog.base.set_state(398);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Comma {
				{
				recog.base.set_state(396);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				recog.base.set_state(397);
				recog.base.match_token(T__11,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameterList ----------------
pub type ParameterListContextAll<'input> = ParameterListContext<'input>;


pub type ParameterListContext<'input> = BaseParserRuleContext<'input,ParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ParameterListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parameterList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_parameterList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ParameterListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_parameterList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterList }
}
antlr_rust::type_id!{ParameterListContextExt<'a>}

impl<'input> ParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ParameterListContextExt<'input>>{

fn parameterDeclaration_all(&self) ->  Vec<Rc<ParameterDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameterDeclaration(&self, i: usize) -> Option<Rc<ParameterDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> ParameterListContextAttrs<'input> for ParameterListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterList(&mut self,)
	-> Result<Rc<ParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_parameterList);
        let mut _localctx: Rc<ParameterListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule parameterDeclaration*/
			recog.base.set_state(400);
			recog.parameterDeclaration()?;

			recog.base.set_state(405);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(37,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(401);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule parameterDeclaration*/
					recog.base.set_state(402);
					recog.parameterDeclaration()?;

					}
					} 
				}
				recog.base.set_state(407);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(37,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameterDeclaration ----------------
pub type ParameterDeclarationContextAll<'input> = ParameterDeclarationContext<'input>;


pub type ParameterDeclarationContext<'input> = BaseParserRuleContext<'input,ParameterDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ParameterDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ParameterDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parameterDeclaration(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_parameterDeclaration(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ParameterDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_parameterDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterDeclaration }
}
antlr_rust::type_id!{ParameterDeclarationContextExt<'a>}

impl<'input> ParameterDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterDeclarationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ParameterDeclarationContextExt<'input>>{

fn declarationSpecifiers(&self) -> Option<Rc<DeclarationSpecifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declarationSpecifiers2(&self) -> Option<Rc<DeclarationSpecifiers2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn directAbstractDeclarator(&self) -> Option<Rc<DirectAbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterDeclarationContextAttrs<'input> for ParameterDeclarationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterDeclaration(&mut self,)
	-> Result<Rc<ParameterDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_parameterDeclaration);
        let mut _localctx: Rc<ParameterDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(415);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule declarationSpecifiers*/
					recog.base.set_state(408);
					recog.declarationSpecifiers()?;

					/*InvokeRule declarator*/
					recog.base.set_state(409);
					recog.declarator()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule declarationSpecifiers2*/
					recog.base.set_state(411);
					recog.declarationSpecifiers2()?;

					recog.base.set_state(413);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftBracket {
						{
						/*InvokeRule directAbstractDeclarator*/
						recog.base.set_state(412);
						recog.directAbstractDeclarator_rec(0)?;

						}
					}

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifierList ----------------
pub type IdentifierListContextAll<'input> = IdentifierListContext<'input>;


pub type IdentifierListContext<'input> = BaseParserRuleContext<'input,IdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for IdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for IdentifierListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_identifierList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_identifierList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for IdentifierListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_identifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierList }
}
antlr_rust::type_id!{IdentifierListContextExt<'a>}

impl<'input> IdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<IdentifierListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> IdentifierListContextAttrs<'input> for IdentifierListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierList(&mut self,)
	-> Result<Rc<IdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_identifierList);
        let mut _localctx: Rc<IdentifierListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(417);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(422);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(418);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				recog.base.set_state(419);
				recog.base.match_token(Identifier,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(424);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- directAbstractDeclarator ----------------
pub type DirectAbstractDeclaratorContextAll<'input> = DirectAbstractDeclaratorContext<'input>;


pub type DirectAbstractDeclaratorContext<'input> = BaseParserRuleContext<'input,DirectAbstractDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct DirectAbstractDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DirectAbstractDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DirectAbstractDeclaratorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_directAbstractDeclarator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_directAbstractDeclarator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DirectAbstractDeclaratorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_directAbstractDeclarator(self);
	}
}

impl<'input> CustomRuleContext<'input> for DirectAbstractDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_directAbstractDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_directAbstractDeclarator }
}
antlr_rust::type_id!{DirectAbstractDeclaratorContextExt<'a>}

impl<'input> DirectAbstractDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DirectAbstractDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DirectAbstractDeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DirectAbstractDeclaratorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DirectAbstractDeclaratorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Star, 0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBracket, 0)
}
fn directAbstractDeclarator(&self) -> Option<Rc<DirectAbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DirectAbstractDeclaratorContextAttrs<'input> for DirectAbstractDeclaratorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  directAbstractDeclarator(&mut self,)
	-> Result<Rc<DirectAbstractDeclaratorContextAll<'input>>,ANTLRError> {
		self.directAbstractDeclarator_rec(0)
	}

	fn directAbstractDeclarator_rec(&mut self, _p: isize)
	-> Result<Rc<DirectAbstractDeclaratorContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 76, RULE_directAbstractDeclarator, _p);
	    let mut _localctx: Rc<DirectAbstractDeclaratorContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 76;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(426);
			recog.base.match_token(LeftBracket,&mut recog.err_handler)?;

			recog.base.set_state(427);
			recog.base.match_token(Star,&mut recog.err_handler)?;

			recog.base.set_state(428);
			recog.base.match_token(RightBracket,&mut recog.err_handler)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(436);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(41,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directAbstractDeclarator);
					_localctx = tmp;
					recog.base.set_state(430);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(431);
					recog.base.match_token(LeftBracket,&mut recog.err_handler)?;

					recog.base.set_state(432);
					recog.base.match_token(Star,&mut recog.err_handler)?;

					recog.base.set_state(433);
					recog.base.match_token(RightBracket,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(438);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(41,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- initializer ----------------
pub type InitializerContextAll<'input> = InitializerContext<'input>;


pub type InitializerContext<'input> = BaseParserRuleContext<'input,InitializerContextExt<'input>>;

#[derive(Clone)]
pub struct InitializerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for InitializerContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for InitializerContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_initializer(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_initializer(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for InitializerContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_initializer(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitializerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initializer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initializer }
}
antlr_rust::type_id!{InitializerContextExt<'a>}

impl<'input> InitializerContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitializerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitializerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitializerContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<InitializerContextExt<'input>>{

fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBrace, 0)
}
fn initializerList(&self) -> Option<Rc<InitializerListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}

}

impl<'input> InitializerContextAttrs<'input> for InitializerContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initializer(&mut self,)
	-> Result<Rc<InitializerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_initializer);
        let mut _localctx: Rc<InitializerContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(447);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 At | Star | Tilde | Quote | LogicalAnd | Sharp | DoubleSharp | Minus |
			 DecrementOp | Plus | IncrementOp | Dot | DotAt | Dollar | DollarAt |
			 Bang | GetArraySize | Menu | Identifier | String | Number 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assignmentExpression*/
					recog.base.set_state(439);
					recog.assignmentExpression()?;

					}
				}

			 LeftBrace 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(440);
					recog.base.match_token(LeftBrace,&mut recog.err_handler)?;

					/*InvokeRule initializerList*/
					recog.base.set_state(441);
					recog.initializerList()?;

					recog.base.set_state(443);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Comma {
						{
						recog.base.set_state(442);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(445);
					recog.base.match_token(RightBrace,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- initializerList ----------------
pub type InitializerListContextAll<'input> = InitializerListContext<'input>;


pub type InitializerListContext<'input> = BaseParserRuleContext<'input,InitializerListContextExt<'input>>;

#[derive(Clone)]
pub struct InitializerListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for InitializerListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for InitializerListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_initializerList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_initializerList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for InitializerListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_initializerList(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitializerListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initializerList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initializerList }
}
antlr_rust::type_id!{InitializerListContextExt<'a>}

impl<'input> InitializerListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitializerListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitializerListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitializerListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<InitializerListContextExt<'input>>{

fn initializer_all(&self) ->  Vec<Rc<InitializerContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn initializer(&self, i: usize) -> Option<Rc<InitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn designation_all(&self) ->  Vec<Rc<DesignationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn designation(&self, i: usize) -> Option<Rc<DesignationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> InitializerListContextAttrs<'input> for InitializerListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initializerList(&mut self,)
	-> Result<Rc<InitializerListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitializerListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_initializerList);
        let mut _localctx: Rc<InitializerListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(450);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(44,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule designation*/
					recog.base.set_state(449);
					recog.designation()?;

					}
				}

				_ => {}
			}
			/*InvokeRule initializer*/
			recog.base.set_state(452);
			recog.initializer()?;

			recog.base.set_state(460);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(46,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(453);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					recog.base.set_state(455);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule designation*/
							recog.base.set_state(454);
							recog.designation()?;

							}
						}

						_ => {}
					}
					/*InvokeRule initializer*/
					recog.base.set_state(457);
					recog.initializer()?;

					}
					} 
				}
				recog.base.set_state(462);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(46,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- designation ----------------
pub type DesignationContextAll<'input> = DesignationContext<'input>;


pub type DesignationContext<'input> = BaseParserRuleContext<'input,DesignationContextExt<'input>>;

#[derive(Clone)]
pub struct DesignationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DesignationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DesignationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_designation(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_designation(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DesignationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_designation(self);
	}
}

impl<'input> CustomRuleContext<'input> for DesignationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designation }
}
antlr_rust::type_id!{DesignationContextExt<'a>}

impl<'input> DesignationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DesignationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DesignationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DesignationContextExt<'input>>{

fn designatorList(&self) -> Option<Rc<DesignatorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Equal
/// Returns `None` if there is no child corresponding to token Equal
fn Equal(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Equal, 0)
}

}

impl<'input> DesignationContextAttrs<'input> for DesignationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn designation(&mut self,)
	-> Result<Rc<DesignationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_designation);
        let mut _localctx: Rc<DesignationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule designatorList*/
			recog.base.set_state(463);
			recog.designatorList()?;

			recog.base.set_state(464);
			recog.base.match_token(Equal,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- designatorList ----------------
pub type DesignatorListContextAll<'input> = DesignatorListContext<'input>;


pub type DesignatorListContext<'input> = BaseParserRuleContext<'input,DesignatorListContextExt<'input>>;

#[derive(Clone)]
pub struct DesignatorListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DesignatorListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DesignatorListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_designatorList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_designatorList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DesignatorListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_designatorList(self);
	}
}

impl<'input> CustomRuleContext<'input> for DesignatorListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designatorList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designatorList }
}
antlr_rust::type_id!{DesignatorListContextExt<'a>}

impl<'input> DesignatorListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DesignatorListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignatorListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DesignatorListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DesignatorListContextExt<'input>>{

fn designator_all(&self) ->  Vec<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn designator(&self, i: usize) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DesignatorListContextAttrs<'input> for DesignatorListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn designatorList(&mut self,)
	-> Result<Rc<DesignatorListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignatorListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_designatorList);
        let mut _localctx: Rc<DesignatorListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(467); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule designator*/
				recog.base.set_state(466);
				recog.designator()?;

				}
				}
				recog.base.set_state(469); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==LeftBracket || _la==Dot) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- designator ----------------
pub type DesignatorContextAll<'input> = DesignatorContext<'input>;


pub type DesignatorContext<'input> = BaseParserRuleContext<'input,DesignatorContextExt<'input>>;

#[derive(Clone)]
pub struct DesignatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DesignatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DesignatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_designator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_designator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DesignatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_designator(self);
	}
}

impl<'input> CustomRuleContext<'input> for DesignatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designator }
}
antlr_rust::type_id!{DesignatorContextExt<'a>}

impl<'input> DesignatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DesignatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DesignatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DesignatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBracket, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Dot
/// Returns `None` if there is no child corresponding to token Dot
fn Dot(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Dot, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> DesignatorContextAttrs<'input> for DesignatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn designator(&mut self,)
	-> Result<Rc<DesignatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_designator);
        let mut _localctx: Rc<DesignatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(477);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LeftBracket 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(471);
					recog.base.match_token(LeftBracket,&mut recog.err_handler)?;

					/*InvokeRule constantExpression*/
					recog.base.set_state(472);
					recog.constantExpression()?;

					recog.base.set_state(473);
					recog.base.match_token(RightBracket,&mut recog.err_handler)?;

					}
				}

			 Dot 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(475);
					recog.base.match_token(Dot,&mut recog.err_handler)?;

					recog.base.set_state(476);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for StatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_statement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::type_id!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn labeledStatement(&self) -> Option<Rc<LabeledStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionStatement(&self) -> Option<Rc<ExpressionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn selectionStatement(&self) -> Option<Rc<SelectionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iterationStatement(&self) -> Option<Rc<IterationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn jumpStatement(&self) -> Option<Rc<JumpStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn menuStatement(&self) -> Option<Rc<MenuStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn commandStatement(&self) -> Option<Rc<CommandStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dialogStatement(&self) -> Option<Rc<DialogStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(488);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(49,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule labeledStatement*/
					recog.base.set_state(479);
					recog.labeledStatement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule compoundStatement*/
					recog.base.set_state(480);
					recog.compoundStatement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expressionStatement*/
					recog.base.set_state(481);
					recog.expressionStatement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule selectionStatement*/
					recog.base.set_state(482);
					recog.selectionStatement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule iterationStatement*/
					recog.base.set_state(483);
					recog.iterationStatement()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule jumpStatement*/
					recog.base.set_state(484);
					recog.jumpStatement()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule menuStatement*/
					recog.base.set_state(485);
					recog.menuStatement()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule commandStatement*/
					recog.base.set_state(486);
					recog.commandStatement()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule dialogStatement*/
					recog.base.set_state(487);
					recog.dialogStatement()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- labeledStatement ----------------
pub type LabeledStatementContextAll<'input> = LabeledStatementContext<'input>;


pub type LabeledStatementContext<'input> = BaseParserRuleContext<'input,LabeledStatementContextExt<'input>>;

#[derive(Clone)]
pub struct LabeledStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for LabeledStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for LabeledStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_labeledStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_labeledStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for LabeledStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_labeledStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for LabeledStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_labeledStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_labeledStatement }
}
antlr_rust::type_id!{LabeledStatementContextExt<'a>}

impl<'input> LabeledStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabeledStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabeledStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LabeledStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<LabeledStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Colon, 0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Case
/// Returns `None` if there is no child corresponding to token Case
fn Case(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Case, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LabeledStatementContextAttrs<'input> for LabeledStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn labeledStatement(&mut self,)
	-> Result<Rc<LabeledStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabeledStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_labeledStatement);
        let mut _localctx: Rc<LabeledStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(501);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(490);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(491);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(492);
					recog.statement()?;

					}
				}

			 Case 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(493);
					recog.base.match_token(Case,&mut recog.err_handler)?;

					/*InvokeRule constantExpression*/
					recog.base.set_state(494);
					recog.constantExpression()?;

					recog.base.set_state(495);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(496);
					recog.statement()?;

					}
				}

			 T__12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(498);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					recog.base.set_state(499);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(500);
					recog.statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compoundStatement ----------------
pub type CompoundStatementContextAll<'input> = CompoundStatementContext<'input>;


pub type CompoundStatementContext<'input> = BaseParserRuleContext<'input,CompoundStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CompoundStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for CompoundStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for CompoundStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compoundStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_compoundStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for CompoundStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_compoundStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompoundStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compoundStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compoundStatement }
}
antlr_rust::type_id!{CompoundStatementContextExt<'a>}

impl<'input> CompoundStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompoundStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompoundStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompoundStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<CompoundStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBrace, 0)
}
fn blockItemList(&self) -> Option<Rc<BlockItemListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompoundStatementContextAttrs<'input> for CompoundStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compoundStatement(&mut self,)
	-> Result<Rc<CompoundStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompoundStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_compoundStatement);
        let mut _localctx: Rc<CompoundStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(503);
			recog.base.match_token(LeftBrace,&mut recog.err_handler)?;

			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__12) | (1usize << T__13) | (1usize << LeftBrace) | (1usize << At) | (1usize << SemiColon) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << LogicalAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << If) | (1usize << End))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (For - 64)) | (1usize << (While - 64)) | (1usize << (Do - 64)) | (1usize << (Goto - 64)) | (1usize << (Return - 64)) | (1usize << (Switch - 64)) | (1usize << (Case - 64)) | (1usize << (Function - 64)) | (1usize << (Break - 64)) | (1usize << (GetArraySize - 64)) | (1usize << (Close - 64)) | (1usize << (Close2 - 64)) | (1usize << (Next - 64)) | (1usize << (Menu - 64)) | (1usize << (Identifier - 64)) | (1usize << (String - 64)) | (1usize << (Number - 64)))) != 0) {
				{
				/*InvokeRule blockItemList*/
				recog.base.set_state(504);
				recog.blockItemList()?;

				}
			}

			recog.base.set_state(507);
			recog.base.match_token(RightBrace,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- blockItemList ----------------
pub type BlockItemListContextAll<'input> = BlockItemListContext<'input>;


pub type BlockItemListContext<'input> = BaseParserRuleContext<'input,BlockItemListContextExt<'input>>;

#[derive(Clone)]
pub struct BlockItemListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for BlockItemListContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for BlockItemListContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_blockItemList(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_blockItemList(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for BlockItemListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_blockItemList(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockItemListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockItemList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockItemList }
}
antlr_rust::type_id!{BlockItemListContextExt<'a>}

impl<'input> BlockItemListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockItemListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockItemListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockItemListContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<BlockItemListContextExt<'input>>{

fn blockItem_all(&self) ->  Vec<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockItem(&self, i: usize) -> Option<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockItemListContextAttrs<'input> for BlockItemListContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blockItemList(&mut self,)
	-> Result<Rc<BlockItemListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockItemListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_blockItemList);
        let mut _localctx: Rc<BlockItemListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(510); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule blockItem*/
				recog.base.set_state(509);
				recog.blockItem()?;

				}
				}
				recog.base.set_state(512); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__12) | (1usize << T__13) | (1usize << LeftBrace) | (1usize << At) | (1usize << SemiColon) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << LogicalAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << If) | (1usize << End))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (For - 64)) | (1usize << (While - 64)) | (1usize << (Do - 64)) | (1usize << (Goto - 64)) | (1usize << (Return - 64)) | (1usize << (Switch - 64)) | (1usize << (Case - 64)) | (1usize << (Function - 64)) | (1usize << (Break - 64)) | (1usize << (GetArraySize - 64)) | (1usize << (Close - 64)) | (1usize << (Close2 - 64)) | (1usize << (Next - 64)) | (1usize << (Menu - 64)) | (1usize << (Identifier - 64)) | (1usize << (String - 64)) | (1usize << (Number - 64)))) != 0)) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- blockItem ----------------
pub type BlockItemContextAll<'input> = BlockItemContext<'input>;


pub type BlockItemContext<'input> = BaseParserRuleContext<'input,BlockItemContextExt<'input>>;

#[derive(Clone)]
pub struct BlockItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for BlockItemContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for BlockItemContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_blockItem(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_blockItem(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for BlockItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_blockItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockItem }
}
antlr_rust::type_id!{BlockItemContextExt<'a>}

impl<'input> BlockItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockItemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockItemContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<BlockItemContextExt<'input>>{

fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionDefinition(&self) -> Option<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BlockItemContextAttrs<'input> for BlockItemContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blockItem(&mut self,)
	-> Result<Rc<BlockItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_blockItem);
        let mut _localctx: Rc<BlockItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(517);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(53,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule statement*/
					recog.base.set_state(514);
					recog.statement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(515);
					recog.functionDefinition()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule declaration*/
					recog.base.set_state(516);
					recog.declaration()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionStatement ----------------
pub type ExpressionStatementContextAll<'input> = ExpressionStatementContext<'input>;


pub type ExpressionStatementContext<'input> = BaseParserRuleContext<'input,ExpressionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ExpressionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ExpressionStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_expressionStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ExpressionStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_expressionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionStatement }
}
antlr_rust::type_id!{ExpressionStatementContextExt<'a>}

impl<'input> ExpressionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ExpressionStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionStatementContextAttrs<'input> for ExpressionStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionStatement(&mut self,)
	-> Result<Rc<ExpressionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_expressionStatement);
        let mut _localctx: Rc<ExpressionStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(520);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Star - 23)) | (1usize << (Tilde - 23)) | (1usize << (Quote - 23)) | (1usize << (LogicalAnd - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Minus - 23)) | (1usize << (DecrementOp - 23)) | (1usize << (Plus - 23)) | (1usize << (IncrementOp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Bang - 23)) | (1usize << (GetArraySize - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)) | (1usize << (String - 23)) | (1usize << (Number - 23)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(519);
				recog.expression()?;

				}
			}

			recog.base.set_state(522);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- selectionStatement ----------------
pub type SelectionStatementContextAll<'input> = SelectionStatementContext<'input>;


pub type SelectionStatementContext<'input> = BaseParserRuleContext<'input,SelectionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct SelectionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SelectionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SelectionStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_selectionStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_selectionStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SelectionStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_selectionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for SelectionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_selectionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_selectionStatement }
}
antlr_rust::type_id!{SelectionStatementContextExt<'a>}

impl<'input> SelectionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SelectionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SelectionStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SelectionStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SelectionStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token If
/// Returns `None` if there is no child corresponding to token If
fn If(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(If, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Else
/// Returns `None` if there is no child corresponding to token Else
fn Else(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Else, 0)
}
/// Retrieves first TerminalNode corresponding to token Switch
/// Returns `None` if there is no child corresponding to token Switch
fn Switch(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Switch, 0)
}

}

impl<'input> SelectionStatementContextAttrs<'input> for SelectionStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn selectionStatement(&mut self,)
	-> Result<Rc<SelectionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SelectionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_selectionStatement);
        let mut _localctx: Rc<SelectionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(539);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 If 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(524);
					recog.base.match_token(If,&mut recog.err_handler)?;

					recog.base.set_state(525);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(526);
					recog.expression()?;

					recog.base.set_state(527);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(528);
					recog.statement()?;

					recog.base.set_state(531);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(529);
							recog.base.match_token(Else,&mut recog.err_handler)?;

							/*InvokeRule statement*/
							recog.base.set_state(530);
							recog.statement()?;

							}
						}

						_ => {}
					}
					}
				}

			 Switch 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(533);
					recog.base.match_token(Switch,&mut recog.err_handler)?;

					recog.base.set_state(534);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(535);
					recog.expression()?;

					recog.base.set_state(536);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(537);
					recog.statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iterationStatement ----------------
pub type IterationStatementContextAll<'input> = IterationStatementContext<'input>;


pub type IterationStatementContext<'input> = BaseParserRuleContext<'input,IterationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IterationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for IterationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for IterationStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_iterationStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_iterationStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for IterationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_iterationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IterationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iterationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iterationStatement }
}
antlr_rust::type_id!{IterationStatementContextExt<'a>}

impl<'input> IterationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IterationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IterationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IterationStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<IterationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token While
/// Returns `None` if there is no child corresponding to token While
fn While(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(While, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Do
/// Returns `None` if there is no child corresponding to token Do
fn Do(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Do, 0)
}
/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}
/// Retrieves first TerminalNode corresponding to token For
/// Returns `None` if there is no child corresponding to token For
fn For(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(For, 0)
}
fn forCondition(&self) -> Option<Rc<ForConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IterationStatementContextAttrs<'input> for IterationStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iterationStatement(&mut self,)
	-> Result<Rc<IterationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IterationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_iterationStatement);
        let mut _localctx: Rc<IterationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(561);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 While 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(541);
					recog.base.match_token(While,&mut recog.err_handler)?;

					recog.base.set_state(542);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(543);
					recog.expression()?;

					recog.base.set_state(544);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(545);
					recog.statement()?;

					}
				}

			 Do 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(547);
					recog.base.match_token(Do,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(548);
					recog.statement()?;

					recog.base.set_state(549);
					recog.base.match_token(While,&mut recog.err_handler)?;

					recog.base.set_state(550);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(551);
					recog.expression()?;

					recog.base.set_state(552);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					recog.base.set_state(553);
					recog.base.match_token(SemiColon,&mut recog.err_handler)?;

					}
				}

			 For 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(555);
					recog.base.match_token(For,&mut recog.err_handler)?;

					recog.base.set_state(556);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule forCondition*/
					recog.base.set_state(557);
					recog.forCondition()?;

					recog.base.set_state(558);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(559);
					recog.statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forCondition ----------------
pub type ForConditionContextAll<'input> = ForConditionContext<'input>;


pub type ForConditionContext<'input> = BaseParserRuleContext<'input,ForConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ForConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ForConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ForConditionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forCondition(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_forCondition(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ForConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_forCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forCondition }
}
antlr_rust::type_id!{ForConditionContextExt<'a>}

impl<'input> ForConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForConditionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ForConditionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SemiColon in current rule
fn SemiColon_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SemiColon, starting from 0.
/// Returns `None` if number of children corresponding to token SemiColon is less or equal than `i`.
fn SemiColon(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, i)
}
fn forDeclaration(&self) -> Option<Rc<ForDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forExpression_all(&self) ->  Vec<Rc<ForExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn forExpression(&self, i: usize) -> Option<Rc<ForExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForConditionContextAttrs<'input> for ForConditionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forCondition(&mut self,)
	-> Result<Rc<ForConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_forCondition);
        let mut _localctx: Rc<ForConditionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(567);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(59,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule forDeclaration*/
					recog.base.set_state(563);
					recog.forDeclaration()?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(565);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Star - 23)) | (1usize << (Tilde - 23)) | (1usize << (Quote - 23)) | (1usize << (LogicalAnd - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Minus - 23)) | (1usize << (DecrementOp - 23)) | (1usize << (Plus - 23)) | (1usize << (IncrementOp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Bang - 23)) | (1usize << (GetArraySize - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)) | (1usize << (String - 23)) | (1usize << (Number - 23)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(564);
						recog.expression()?;

						}
					}

					}
				}

				_ => {}
			}
			recog.base.set_state(569);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			recog.base.set_state(571);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Star - 23)) | (1usize << (Tilde - 23)) | (1usize << (Quote - 23)) | (1usize << (LogicalAnd - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Minus - 23)) | (1usize << (DecrementOp - 23)) | (1usize << (Plus - 23)) | (1usize << (IncrementOp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Bang - 23)) | (1usize << (GetArraySize - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)) | (1usize << (String - 23)) | (1usize << (Number - 23)))) != 0) {
				{
				/*InvokeRule forExpression*/
				recog.base.set_state(570);
				recog.forExpression()?;

				}
			}

			recog.base.set_state(573);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			recog.base.set_state(575);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Star - 23)) | (1usize << (Tilde - 23)) | (1usize << (Quote - 23)) | (1usize << (LogicalAnd - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Minus - 23)) | (1usize << (DecrementOp - 23)) | (1usize << (Plus - 23)) | (1usize << (IncrementOp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Bang - 23)) | (1usize << (GetArraySize - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)) | (1usize << (String - 23)) | (1usize << (Number - 23)))) != 0) {
				{
				/*InvokeRule forExpression*/
				recog.base.set_state(574);
				recog.forExpression()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forDeclaration ----------------
pub type ForDeclarationContextAll<'input> = ForDeclarationContext<'input>;


pub type ForDeclarationContext<'input> = BaseParserRuleContext<'input,ForDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ForDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ForDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ForDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forDeclaration(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_forDeclaration(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ForDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_forDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forDeclaration }
}
antlr_rust::type_id!{ForDeclarationContextExt<'a>}

impl<'input> ForDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForDeclarationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ForDeclarationContextExt<'input>>{

fn initDeclaratorList(&self) -> Option<Rc<InitDeclaratorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForDeclarationContextAttrs<'input> for ForDeclarationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forDeclaration(&mut self,)
	-> Result<Rc<ForDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_forDeclaration);
        let mut _localctx: Rc<ForDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(578);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Quote - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Function - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)))) != 0) {
				{
				/*InvokeRule initDeclaratorList*/
				recog.base.set_state(577);
				recog.initDeclaratorList()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forExpression ----------------
pub type ForExpressionContextAll<'input> = ForExpressionContext<'input>;


pub type ForExpressionContext<'input> = BaseParserRuleContext<'input,ForExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ForExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ForExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ForExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_forExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ForExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_forExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forExpression }
}
antlr_rust::type_id!{ForExpressionContextExt<'a>}

impl<'input> ForExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ForExpressionContextExt<'input>>{

fn assignmentExpression_all(&self) ->  Vec<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentExpression(&self, i: usize) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> ForExpressionContextAttrs<'input> for ForExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forExpression(&mut self,)
	-> Result<Rc<ForExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_forExpression);
        let mut _localctx: Rc<ForExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule assignmentExpression*/
			recog.base.set_state(580);
			recog.assignmentExpression()?;

			recog.base.set_state(585);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(581);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				/*InvokeRule assignmentExpression*/
				recog.base.set_state(582);
				recog.assignmentExpression()?;

				}
				}
				recog.base.set_state(587);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- jumpStatement ----------------
pub type JumpStatementContextAll<'input> = JumpStatementContext<'input>;


pub type JumpStatementContext<'input> = BaseParserRuleContext<'input,JumpStatementContextExt<'input>>;

#[derive(Clone)]
pub struct JumpStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for JumpStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for JumpStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_jumpStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_jumpStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for JumpStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_jumpStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for JumpStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_jumpStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_jumpStatement }
}
antlr_rust::type_id!{JumpStatementContextExt<'a>}

impl<'input> JumpStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<JumpStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,JumpStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait JumpStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<JumpStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}
/// Retrieves first TerminalNode corresponding to token Goto
/// Returns `None` if there is no child corresponding to token Goto
fn Goto(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Goto, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Return
/// Returns `None` if there is no child corresponding to token Return
fn Return(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Return, 0)
}
/// Retrieves first TerminalNode corresponding to token Break
/// Returns `None` if there is no child corresponding to token Break
fn Break(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Break, 0)
}
/// Retrieves first TerminalNode corresponding to token End
/// Returns `None` if there is no child corresponding to token End
fn End(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(End, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> JumpStatementContextAttrs<'input> for JumpStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn jumpStatement(&mut self,)
	-> Result<Rc<JumpStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = JumpStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_jumpStatement);
        let mut _localctx: Rc<JumpStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(595);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Goto 
				=> {
					{
					recog.base.set_state(588);
					recog.base.match_token(Goto,&mut recog.err_handler)?;

					recog.base.set_state(589);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

			 T__13 | End | Break 
				=> {
					{
					recog.base.set_state(590);
					_la = recog.base.input.la(1);
					if { !(((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (End - 14)) | (1usize << (Break - 14)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

			 Return 
				=> {
					{
					recog.base.set_state(591);
					recog.base.match_token(Return,&mut recog.err_handler)?;

					recog.base.set_state(593);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (At - 23)) | (1usize << (Star - 23)) | (1usize << (Tilde - 23)) | (1usize << (Quote - 23)) | (1usize << (LogicalAnd - 23)) | (1usize << (Sharp - 23)) | (1usize << (DoubleSharp - 23)) | (1usize << (Minus - 23)) | (1usize << (DecrementOp - 23)) | (1usize << (Plus - 23)) | (1usize << (IncrementOp - 23)) | (1usize << (Dot - 23)) | (1usize << (DotAt - 23)) | (1usize << (Dollar - 23)) | (1usize << (DollarAt - 23)) | (1usize << (Bang - 23)) | (1usize << (GetArraySize - 23)) | (1usize << (Menu - 23)) | (1usize << (Identifier - 23)) | (1usize << (String - 23)) | (1usize << (Number - 23)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(592);
						recog.expression()?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(597);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- menuStatement ----------------
pub type MenuStatementContextAll<'input> = MenuStatementContext<'input>;


pub type MenuStatementContext<'input> = BaseParserRuleContext<'input,MenuStatementContextExt<'input>>;

#[derive(Clone)]
pub struct MenuStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for MenuStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for MenuStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_menuStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_menuStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for MenuStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_menuStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for MenuStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_menuStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_menuStatement }
}
antlr_rust::type_id!{MenuStatementContextExt<'a>}

impl<'input> MenuStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MenuStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MenuStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MenuStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<MenuStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Menu
/// Returns `None` if there is no child corresponding to token Menu
fn Menu(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Menu, 0)
}
fn menuItem_all(&self) ->  Vec<Rc<MenuItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn menuItem(&self, i: usize) -> Option<Rc<MenuItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> MenuStatementContextAttrs<'input> for MenuStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn menuStatement(&mut self,)
	-> Result<Rc<MenuStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MenuStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_menuStatement);
        let mut _localctx: Rc<MenuStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(599);
			recog.base.match_token(Menu,&mut recog.err_handler)?;

			/*InvokeRule menuItem*/
			recog.base.set_state(600);
			recog.menuItem()?;

			recog.base.set_state(605);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(601);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				/*InvokeRule menuItem*/
				recog.base.set_state(602);
				recog.menuItem()?;

				}
				}
				recog.base.set_state(607);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- menuItem ----------------
pub type MenuItemContextAll<'input> = MenuItemContext<'input>;


pub type MenuItemContext<'input> = BaseParserRuleContext<'input,MenuItemContextExt<'input>>;

#[derive(Clone)]
pub struct MenuItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for MenuItemContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for MenuItemContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_menuItem(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_menuItem(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for MenuItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_menuItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for MenuItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_menuItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_menuItem }
}
antlr_rust::type_id!{MenuItemContextExt<'a>}

impl<'input> MenuItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MenuItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MenuItemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MenuItemContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<MenuItemContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Minus, 0)
}

}

impl<'input> MenuItemContextAttrs<'input> for MenuItemContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn menuItem(&mut self,)
	-> Result<Rc<MenuItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MenuItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_menuItem);
        let mut _localctx: Rc<MenuItemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(608);
			recog.expression()?;

			recog.base.set_state(609);
			recog.base.match_token(Comma,&mut recog.err_handler)?;

			recog.base.set_state(610);
			_la = recog.base.input.la(1);
			if { !(_la==Minus || _la==Identifier) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- commandStatement ----------------
pub type CommandStatementContextAll<'input> = CommandStatementContext<'input>;


pub type CommandStatementContext<'input> = BaseParserRuleContext<'input,CommandStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CommandStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for CommandStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for CommandStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_commandStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_commandStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for CommandStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_commandStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for CommandStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_commandStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_commandStatement }
}
antlr_rust::type_id!{CommandStatementContextExt<'a>}

impl<'input> CommandStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommandStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommandStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommandStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<CommandStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> CommandStatementContextAttrs<'input> for CommandStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn commandStatement(&mut self,)
	-> Result<Rc<CommandStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommandStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_commandStatement);
        let mut _localctx: Rc<CommandStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(612);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(613);
			recog.expression()?;

			recog.base.set_state(618);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(614);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(615);
				recog.expression()?;

				}
				}
				recog.base.set_state(620);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- dialogStatement ----------------
pub type DialogStatementContextAll<'input> = DialogStatementContext<'input>;


pub type DialogStatementContext<'input> = BaseParserRuleContext<'input,DialogStatementContextExt<'input>>;

#[derive(Clone)]
pub struct DialogStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for DialogStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for DialogStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dialogStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_dialogStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for DialogStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_dialogStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DialogStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dialogStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dialogStatement }
}
antlr_rust::type_id!{DialogStatementContextExt<'a>}

impl<'input> DialogStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DialogStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DialogStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DialogStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<DialogStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}
/// Retrieves first TerminalNode corresponding to token Close
/// Returns `None` if there is no child corresponding to token Close
fn Close(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Close, 0)
}
/// Retrieves first TerminalNode corresponding to token Close2
/// Returns `None` if there is no child corresponding to token Close2
fn Close2(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Close2, 0)
}
/// Retrieves first TerminalNode corresponding to token Next
/// Returns `None` if there is no child corresponding to token Next
fn Next(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Next, 0)
}

}

impl<'input> DialogStatementContextAttrs<'input> for DialogStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dialogStatement(&mut self,)
	-> Result<Rc<DialogStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DialogStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_dialogStatement);
        let mut _localctx: Rc<DialogStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(621);
			_la = recog.base.input.la(1);
			if { !(((((_la - 75)) & !0x3f) == 0 && ((1usize << (_la - 75)) & ((1usize << (Close - 75)) | (1usize << (Close2 - 75)) | (1usize << (Next - 75)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(622);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- translationUnit ----------------
pub type TranslationUnitContextAll<'input> = TranslationUnitContext<'input>;


pub type TranslationUnitContext<'input> = BaseParserRuleContext<'input,TranslationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct TranslationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for TranslationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for TranslationUnitContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_translationUnit(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_translationUnit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for TranslationUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_translationUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for TranslationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_translationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_translationUnit }
}
antlr_rust::type_id!{TranslationUnitContextExt<'a>}

impl<'input> TranslationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TranslationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TranslationUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TranslationUnitContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<TranslationUnitContextExt<'input>>{

fn externalDeclaration_all(&self) ->  Vec<Rc<ExternalDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn externalDeclaration(&self, i: usize) -> Option<Rc<ExternalDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TranslationUnitContextAttrs<'input> for TranslationUnitContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn translationUnit(&mut self,)
	-> Result<Rc<TranslationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TranslationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_translationUnit);
        let mut _localctx: Rc<TranslationUnitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(625); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule externalDeclaration*/
				recog.base.set_state(624);
				recog.externalDeclaration()?;

				}
				}
				recog.base.set_state(627); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__12) | (1usize << T__13) | (1usize << LeftBrace) | (1usize << At) | (1usize << SemiColon) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << LogicalAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << If) | (1usize << End))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (For - 64)) | (1usize << (While - 64)) | (1usize << (Do - 64)) | (1usize << (Goto - 64)) | (1usize << (Return - 64)) | (1usize << (Switch - 64)) | (1usize << (Case - 64)) | (1usize << (Function - 64)) | (1usize << (Break - 64)) | (1usize << (GetArraySize - 64)) | (1usize << (Close - 64)) | (1usize << (Close2 - 64)) | (1usize << (Next - 64)) | (1usize << (Menu - 64)) | (1usize << (Identifier - 64)) | (1usize << (String - 64)) | (1usize << (Number - 64)))) != 0)) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externalDeclaration ----------------
pub type ExternalDeclarationContextAll<'input> = ExternalDeclarationContext<'input>;


pub type ExternalDeclarationContext<'input> = BaseParserRuleContext<'input,ExternalDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ExternalDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ExternalDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ExternalDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_externalDeclaration(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_externalDeclaration(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ExternalDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_externalDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExternalDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externalDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externalDeclaration }
}
antlr_rust::type_id!{ExternalDeclarationContextExt<'a>}

impl<'input> ExternalDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExternalDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExternalDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExternalDeclarationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ExternalDeclarationContextExt<'input>>{

fn functionDefinition(&self) -> Option<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blockItem(&self) -> Option<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptInitialization(&self) -> Option<Rc<ScriptInitializationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}

}

impl<'input> ExternalDeclarationContextAttrs<'input> for ExternalDeclarationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externalDeclaration(&mut self,)
	-> Result<Rc<ExternalDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExternalDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_externalDeclaration);
        let mut _localctx: Rc<ExternalDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(633);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(69,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(629);
					recog.functionDefinition()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule blockItem*/
					recog.base.set_state(630);
					recog.blockItem()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule scriptInitialization*/
					recog.base.set_state(631);
					recog.scriptInitialization()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(632);
					recog.base.match_token(SemiColon,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionDefinition ----------------
pub type FunctionDefinitionContextAll<'input> = FunctionDefinitionContext<'input>;


pub type FunctionDefinitionContext<'input> = BaseParserRuleContext<'input,FunctionDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for FunctionDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for FunctionDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionDefinition(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_functionDefinition(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for FunctionDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_functionDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDefinition }
}
antlr_rust::type_id!{FunctionDefinitionContextExt<'a>}

impl<'input> FunctionDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDefinitionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<FunctionDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Function
/// Returns `None` if there is no child corresponding to token Function
fn Function(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Function, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn functionDefinitionArguments(&self) -> Option<Rc<FunctionDefinitionArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionDefinitionContextAttrs<'input> for FunctionDefinitionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionDefinition(&mut self,)
	-> Result<Rc<FunctionDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_functionDefinition);
        let mut _localctx: Rc<FunctionDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(635);
			recog.base.match_token(Function,&mut recog.err_handler)?;

			recog.base.set_state(636);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(641);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LeftParen {
				{
				recog.base.set_state(637);
				recog.base.match_token(LeftParen,&mut recog.err_handler)?;

				/*InvokeRule functionDefinitionArguments*/
				recog.base.set_state(638);
				recog.functionDefinitionArguments()?;

				recog.base.set_state(639);
				recog.base.match_token(RightParen,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(644);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule compoundStatement*/
					recog.base.set_state(643);
					recog.compoundStatement()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionDefinitionArguments ----------------
pub type FunctionDefinitionArgumentsContextAll<'input> = FunctionDefinitionArgumentsContext<'input>;


pub type FunctionDefinitionArgumentsContext<'input> = BaseParserRuleContext<'input,FunctionDefinitionArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDefinitionArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for FunctionDefinitionArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for FunctionDefinitionArgumentsContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionDefinitionArguments(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_functionDefinitionArguments(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for FunctionDefinitionArgumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_functionDefinitionArguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionDefinitionArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDefinitionArguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDefinitionArguments }
}
antlr_rust::type_id!{FunctionDefinitionArgumentsContextExt<'a>}

impl<'input> FunctionDefinitionArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionDefinitionArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDefinitionArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDefinitionArgumentsContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<FunctionDefinitionArgumentsContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> FunctionDefinitionArgumentsContextAttrs<'input> for FunctionDefinitionArgumentsContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionDefinitionArguments(&mut self,)
	-> Result<Rc<FunctionDefinitionArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDefinitionArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_functionDefinitionArguments);
        let mut _localctx: Rc<FunctionDefinitionArgumentsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(647);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Identifier {
				{
				recog.base.set_state(646);
				recog.base.match_token(Identifier,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(653);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Comma {
				{
				{
				recog.base.set_state(649);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				recog.base.set_state(650);
				recog.base.match_token(Identifier,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(655);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scriptInitialization ----------------
pub type ScriptInitializationContextAll<'input> = ScriptInitializationContext<'input>;


pub type ScriptInitializationContext<'input> = BaseParserRuleContext<'input,ScriptInitializationContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptInitializationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptInitializationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptInitializationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptInitialization(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptInitialization(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptInitializationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptInitialization(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptInitializationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptInitialization }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptInitialization }
}
antlr_rust::type_id!{ScriptInitializationContextExt<'a>}

impl<'input> ScriptInitializationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptInitializationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptInitializationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptInitializationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptInitializationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Minus, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}

}

impl<'input> ScriptInitializationContextAttrs<'input> for ScriptInitializationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptInitialization(&mut self,)
	-> Result<Rc<ScriptInitializationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptInitializationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_scriptInitialization);
        let mut _localctx: Rc<ScriptInitializationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(656);
			recog.base.match_token(Minus,&mut recog.err_handler)?;

			recog.base.set_state(660);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(74,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(657);
					_la = recog.base.input.la(1);
					if { !(_la==T__14 || _la==Identifier) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
					} 
				}
				recog.base.set_state(662);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(74,&mut recog.base)?;
			}
			recog.base.set_state(664);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Comma {
				{
				recog.base.set_state(663);
				recog.base.match_token(Comma,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(667);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(76,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule compoundStatement*/
					recog.base.set_state(666);
					recog.compoundStatement()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scope_specifier ----------------
pub type Scope_specifierContextAll<'input> = Scope_specifierContext<'input>;


pub type Scope_specifierContext<'input> = BaseParserRuleContext<'input,Scope_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Scope_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for Scope_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for Scope_specifierContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scope_specifier(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scope_specifier(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for Scope_specifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scope_specifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Scope_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scope_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scope_specifier }
}
antlr_rust::type_id!{Scope_specifierContextExt<'a>}

impl<'input> Scope_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Scope_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Scope_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Scope_specifierContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<Scope_specifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token At
/// Returns `None` if there is no child corresponding to token At
fn At(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(At, 0)
}
/// Retrieves first TerminalNode corresponding to token Dollar
/// Returns `None` if there is no child corresponding to token Dollar
fn Dollar(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Dollar, 0)
}
/// Retrieves first TerminalNode corresponding to token DollarAt
/// Returns `None` if there is no child corresponding to token DollarAt
fn DollarAt(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DollarAt, 0)
}
/// Retrieves first TerminalNode corresponding to token Dot
/// Returns `None` if there is no child corresponding to token Dot
fn Dot(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Dot, 0)
}
/// Retrieves first TerminalNode corresponding to token DotAt
/// Returns `None` if there is no child corresponding to token DotAt
fn DotAt(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DotAt, 0)
}
/// Retrieves first TerminalNode corresponding to token Quote
/// Returns `None` if there is no child corresponding to token Quote
fn Quote(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Quote, 0)
}
/// Retrieves first TerminalNode corresponding to token Sharp
/// Returns `None` if there is no child corresponding to token Sharp
fn Sharp(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Sharp, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleSharp
/// Returns `None` if there is no child corresponding to token DoubleSharp
fn DoubleSharp(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleSharp, 0)
}

}

impl<'input> Scope_specifierContextAttrs<'input> for Scope_specifierContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scope_specifier(&mut self,)
	-> Result<Rc<Scope_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Scope_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_scope_specifier);
        let mut _localctx: Rc<Scope_specifierContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(669);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << At) | (1usize << Quote) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for VariableContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_variable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::type_id!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

fn scope_specifier(&self) -> Option<Rc<Scope_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable_name(&self) -> Option<Rc<Variable_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(675);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 At | Quote | Sharp | DoubleSharp | Dot | DotAt | Dollar | DollarAt 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule scope_specifier*/
					recog.base.set_state(671);
					recog.scope_specifier()?;

					/*InvokeRule variable_name*/
					recog.base.set_state(672);
					recog.variable_name()?;

					}
				}

			 Menu | Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable_name*/
					recog.base.set_state(674);
					recog.variable_name()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variable_name ----------------
pub type Variable_nameContextAll<'input> = Variable_nameContext<'input>;


pub type Variable_nameContext<'input> = BaseParserRuleContext<'input,Variable_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for Variable_nameContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for Variable_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_name(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_variable_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for Variable_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_variable_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_name }
}
antlr_rust::type_id!{Variable_nameContextExt<'a>}

impl<'input> Variable_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_nameContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<Variable_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Menu
/// Returns `None` if there is no child corresponding to token Menu
fn Menu(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Menu, 0)
}
/// Retrieves first TerminalNode corresponding to token Dollar
/// Returns `None` if there is no child corresponding to token Dollar
fn Dollar(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Dollar, 0)
}

}

impl<'input> Variable_nameContextAttrs<'input> for Variable_nameContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_name(&mut self,)
	-> Result<Rc<Variable_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_variable_name);
        let mut _localctx: Rc<Variable_nameContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(677);
			_la = recog.base.input.la(1);
			if { !(_la==Menu || _la==Identifier) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(679);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(78,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(678);
					recog.base.match_token(Dollar,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x59\u{2ac}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x05\x03\u{93}\x0a\x03\x03\x04\x03\x04\x03\x04\x05\x04\u{98}\
	\x0a\x04\x03\x04\x03\x04\x03\x05\x03\x05\x07\x05\u{9e}\x0a\x05\x0c\x05\x0e\
	\x05\u{a1}\x0b\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x07\x05\
	\u{a9}\x0a\x05\x0c\x05\x0e\x05\u{ac}\x0b\x05\x05\x05\u{ae}\x0a\x05\x03\x06\
	\x03\x06\x03\x06\x07\x06\u{b3}\x0a\x06\x0c\x06\x0e\x06\u{b6}\x0b\x06\x03\
	\x07\x07\x07\u{b9}\x0a\x07\x0c\x07\x0e\x07\u{bc}\x0b\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\u{c7}\x0a\
	\x07\x03\x08\x03\x08\x03\x09\x03\x09\x05\x09\u{cd}\x0a\x09\x03\x0a\x03\x0a\
	\x03\x0a\x07\x0a\u{d2}\x0a\x0a\x0c\x0a\x0e\x0a\u{d5}\x0b\x0a\x03\x0b\x03\
	\x0b\x03\x0b\x07\x0b\u{da}\x0a\x0b\x0c\x0b\x0e\x0b\u{dd}\x0b\x0b\x03\x0c\
	\x03\x0c\x03\x0c\x07\x0c\u{e2}\x0a\x0c\x0c\x0c\x0e\x0c\u{e5}\x0b\x0c\x03\
	\x0d\x03\x0d\x03\x0d\x07\x0d\u{ea}\x0a\x0d\x0c\x0d\x0e\x0d\u{ed}\x0b\x0d\
	\x03\x0e\x03\x0e\x03\x0e\x07\x0e\u{f2}\x0a\x0e\x0c\x0e\x0e\x0e\u{f5}\x0b\
	\x0e\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{fa}\x0a\x0f\x0c\x0f\x0e\x0f\u{fd}\
	\x0b\x0f\x03\x10\x03\x10\x03\x10\x07\x10\u{102}\x0a\x10\x0c\x10\x0e\x10\
	\u{105}\x0b\x10\x03\x11\x03\x11\x03\x11\x07\x11\u{10a}\x0a\x11\x0c\x11\x0e\
	\x11\u{10d}\x0b\x11\x03\x12\x03\x12\x03\x12\x07\x12\u{112}\x0a\x12\x0c\x12\
	\x0e\x12\u{115}\x0b\x12\x03\x13\x03\x13\x03\x13\x07\x13\u{11a}\x0a\x13\x0c\
	\x13\x0e\x13\u{11d}\x0b\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\
	\x14\x05\x14\u{125}\x0a\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\
	\x15\x05\x15\u{12d}\x0a\x15\x03\x16\x03\x16\x05\x16\u{131}\x0a\x16\x03\x17\
	\x03\x17\x03\x18\x03\x18\x03\x18\x07\x18\u{138}\x0a\x18\x0c\x18\x0e\x18\
	\u{13b}\x0b\x18\x03\x19\x03\x19\x03\x1a\x05\x1a\u{140}\x0a\x1a\x03\x1a\x03\
	\x1a\x03\x1b\x06\x1b\u{145}\x0a\x1b\x0d\x1b\x0e\x1b\u{146}\x03\x1c\x03\x1c\
	\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{150}\x0a\x1e\x0c\x1e\
	\x0e\x1e\u{153}\x0b\x1e\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{158}\x0a\x1f\x03\
	\x1f\x03\x1f\x05\x1f\u{15c}\x0a\x1f\x03\x20\x03\x20\x05\x20\u{160}\x0a\x20\
	\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x05\x22\u{16e}\x0a\x22\x03\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\
	\x05\x22\u{17c}\x0a\x22\x03\x22\x07\x22\u{17f}\x0a\x22\x0c\x22\x0e\x22\u{182}\
	\x0b\x22\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x07\x23\u{189}\x0a\x23\
	\x0c\x23\x0e\x23\u{18c}\x0b\x23\x03\x24\x03\x24\x03\x24\x05\x24\u{191}\x0a\
	\x24\x03\x25\x03\x25\x03\x25\x07\x25\u{196}\x0a\x25\x0c\x25\x0e\x25\u{199}\
	\x0b\x25\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{1a0}\x0a\x26\
	\x05\x26\u{1a2}\x0a\x26\x03\x27\x03\x27\x03\x27\x07\x27\u{1a7}\x0a\x27\x0c\
	\x27\x0e\x27\u{1aa}\x0b\x27\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\
	\x28\x03\x28\x03\x28\x03\x28\x07\x28\u{1b5}\x0a\x28\x0c\x28\x0e\x28\u{1b8}\
	\x0b\x28\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{1be}\x0a\x29\x03\x29\
	\x03\x29\x05\x29\u{1c2}\x0a\x29\x03\x2a\x05\x2a\u{1c5}\x0a\x2a\x03\x2a\x03\
	\x2a\x03\x2a\x05\x2a\u{1ca}\x0a\x2a\x03\x2a\x07\x2a\u{1cd}\x0a\x2a\x0c\x2a\
	\x0e\x2a\u{1d0}\x0b\x2a\x03\x2b\x03\x2b\x03\x2b\x03\x2c\x06\x2c\u{1d6}\x0a\
	\x2c\x0d\x2c\x0e\x2c\u{1d7}\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\
	\x2d\x05\x2d\u{1e0}\x0a\x2d\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\
	\x2e\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{1eb}\x0a\x2e\x03\x2f\x03\x2f\x03\
	\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x05\
	\x2f\u{1f8}\x0a\x2f\x03\x30\x03\x30\x05\x30\u{1fc}\x0a\x30\x03\x30\x03\x30\
	\x03\x31\x06\x31\u{201}\x0a\x31\x0d\x31\x0e\x31\u{202}\x03\x32\x03\x32\x03\
	\x32\x05\x32\u{208}\x0a\x32\x03\x33\x05\x33\u{20b}\x0a\x33\x03\x33\x03\x33\
	\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x05\x34\u{216}\
	\x0a\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x05\x34\u{21e}\
	\x0a\x34\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\
	\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\
	\x03\x35\x03\x35\x03\x35\x05\x35\u{234}\x0a\x35\x03\x36\x03\x36\x05\x36\
	\u{238}\x0a\x36\x05\x36\u{23a}\x0a\x36\x03\x36\x03\x36\x05\x36\u{23e}\x0a\
	\x36\x03\x36\x03\x36\x05\x36\u{242}\x0a\x36\x03\x37\x05\x37\u{245}\x0a\x37\
	\x03\x38\x03\x38\x03\x38\x07\x38\u{24a}\x0a\x38\x0c\x38\x0e\x38\u{24d}\x0b\
	\x38\x03\x39\x03\x39\x03\x39\x03\x39\x03\x39\x05\x39\u{254}\x0a\x39\x05\
	\x39\u{256}\x0a\x39\x03\x39\x03\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x07\
	\x3a\u{25e}\x0a\x3a\x0c\x3a\x0e\x3a\u{261}\x0b\x3a\x03\x3b\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x07\x3c\u{26b}\x0a\x3c\x0c\x3c\
	\x0e\x3c\u{26e}\x0b\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3e\x06\x3e\u{274}\x0a\
	\x3e\x0d\x3e\x0e\x3e\u{275}\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{27c}\
	\x0a\x3f\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x05\x40\u{284}\
	\x0a\x40\x03\x40\x05\x40\u{287}\x0a\x40\x03\x41\x05\x41\u{28a}\x0a\x41\x03\
	\x41\x03\x41\x07\x41\u{28e}\x0a\x41\x0c\x41\x0e\x41\u{291}\x0b\x41\x03\x42\
	\x03\x42\x07\x42\u{295}\x0a\x42\x0c\x42\x0e\x42\u{298}\x0b\x42\x03\x42\x05\
	\x42\u{29b}\x0a\x42\x03\x42\x05\x42\u{29e}\x0a\x42\x03\x43\x03\x43\x03\x44\
	\x03\x44\x03\x44\x03\x44\x05\x44\u{2a6}\x0a\x44\x03\x45\x03\x45\x05\x45\
	\u{2aa}\x0a\x45\x03\x45\x02\x04\x42\x4e\x46\x02\x04\x06\x08\x0a\x0c\x0e\
	\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\
	\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\
	\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\
	\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\x02\x11\x04\x02\x2d\x2d\x2f\x2f\x07\
	\x02\x1d\x1e\x24\x24\x2c\x2c\x2e\x2e\x34\x34\x04\x02\x1c\x1d\x26\x26\x04\
	\x02\x2c\x2c\x2e\x2e\x04\x02\x39\x39\x3c\x3c\x05\x02\x38\x38\x3a\x3b\x3d\
	\x3d\x04\x02\x35\x35\x37\x37\x04\x02\x04\x0d\x36\x36\x03\x02\x12\x13\x05\
	\x02\x10\x10\x40\x40\x4a\x4a\x04\x02\x2c\x2c\x52\x52\x03\x02\x4d\x4f\x04\
	\x02\x11\x11\x52\x52\x06\x02\x19\x19\x20\x20\x2a\x2b\x30\x33\x04\x02\x50\
	\x50\x52\x52\x02\u{2ca}\x02\u{8a}\x03\x02\x02\x02\x04\u{92}\x03\x02\x02\
	\x02\x06\u{94}\x03\x02\x02\x02\x08\u{ad}\x03\x02\x02\x02\x0a\u{af}\x03\x02\
	\x02\x02\x0c\u{ba}\x03\x02\x02\x02\x0e\u{c8}\x03\x02\x02\x02\x10\u{cc}\x03\
	\x02\x02\x02\x12\u{ce}\x03\x02\x02\x02\x14\u{d6}\x03\x02\x02\x02\x16\u{de}\
	\x03\x02\x02\x02\x18\u{e6}\x03\x02\x02\x02\x1a\u{ee}\x03\x02\x02\x02\x1c\
	\u{f6}\x03\x02\x02\x02\x1e\u{fe}\x03\x02\x02\x02\x20\u{106}\x03\x02\x02\
	\x02\x22\u{10e}\x03\x02\x02\x02\x24\u{116}\x03\x02\x02\x02\x26\u{11e}\x03\
	\x02\x02\x02\x28\u{12c}\x03\x02\x02\x02\x2a\u{130}\x03\x02\x02\x02\x2c\u{132}\
	\x03\x02\x02\x02\x2e\u{134}\x03\x02\x02\x02\x30\u{13c}\x03\x02\x02\x02\x32\
	\u{13f}\x03\x02\x02\x02\x34\u{144}\x03\x02\x02\x02\x36\u{148}\x03\x02\x02\
	\x02\x38\u{14a}\x03\x02\x02\x02\x3a\u{14c}\x03\x02\x02\x02\x3c\u{15b}\x03\
	\x02\x02\x02\x3e\u{15d}\x03\x02\x02\x02\x40\u{161}\x03\x02\x02\x02\x42\u{16d}\
	\x03\x02\x02\x02\x44\u{18a}\x03\x02\x02\x02\x46\u{18d}\x03\x02\x02\x02\x48\
	\u{192}\x03\x02\x02\x02\x4a\u{1a1}\x03\x02\x02\x02\x4c\u{1a3}\x03\x02\x02\
	\x02\x4e\u{1ab}\x03\x02\x02\x02\x50\u{1c1}\x03\x02\x02\x02\x52\u{1c4}\x03\
	\x02\x02\x02\x54\u{1d1}\x03\x02\x02\x02\x56\u{1d5}\x03\x02\x02\x02\x58\u{1df}\
	\x03\x02\x02\x02\x5a\u{1ea}\x03\x02\x02\x02\x5c\u{1f7}\x03\x02\x02\x02\x5e\
	\u{1f9}\x03\x02\x02\x02\x60\u{200}\x03\x02\x02\x02\x62\u{207}\x03\x02\x02\
	\x02\x64\u{20a}\x03\x02\x02\x02\x66\u{21d}\x03\x02\x02\x02\x68\u{233}\x03\
	\x02\x02\x02\x6a\u{239}\x03\x02\x02\x02\x6c\u{244}\x03\x02\x02\x02\x6e\u{246}\
	\x03\x02\x02\x02\x70\u{255}\x03\x02\x02\x02\x72\u{259}\x03\x02\x02\x02\x74\
	\u{262}\x03\x02\x02\x02\x76\u{266}\x03\x02\x02\x02\x78\u{26f}\x03\x02\x02\
	\x02\x7a\u{273}\x03\x02\x02\x02\x7c\u{27b}\x03\x02\x02\x02\x7e\u{27d}\x03\
	\x02\x02\x02\u{80}\u{289}\x03\x02\x02\x02\u{82}\u{292}\x03\x02\x02\x02\u{84}\
	\u{29f}\x03\x02\x02\x02\u{86}\u{2a5}\x03\x02\x02\x02\u{88}\u{2a7}\x03\x02\
	\x02\x02\u{8a}\u{8b}\x05\x7a\x3e\x02\u{8b}\u{8c}\x07\x02\x02\x03\u{8c}\x03\
	\x03\x02\x02\x02\u{8d}\u{93}\x07\x52\x02\x02\u{8e}\u{93}\x05\u{86}\x44\x02\
	\u{8f}\u{93}\x07\x55\x02\x02\u{90}\u{93}\x07\x54\x02\x02\u{91}\u{93}\x07\
	\x2c\x02\x02\u{92}\u{8d}\x03\x02\x02\x02\u{92}\u{8e}\x03\x02\x02\x02\u{92}\
	\u{8f}\x03\x02\x02\x02\u{92}\u{90}\x03\x02\x02\x02\u{92}\u{91}\x03\x02\x02\
	\x02\u{93}\x05\x03\x02\x02\x02\u{94}\u{95}\x07\x52\x02\x02\u{95}\u{97}\x07\
	\x12\x02\x02\u{96}\u{98}\x05\x0a\x06\x02\u{97}\u{96}\x03\x02\x02\x02\u{97}\
	\u{98}\x03\x02\x02\x02\u{98}\u{99}\x03\x02\x02\x02\u{99}\u{9a}\x07\x13\x02\
	\x02\u{9a}\x07\x03\x02\x02\x02\u{9b}\u{9f}\x05\x06\x04\x02\u{9c}\u{9e}\x05\
	\x2e\x18\x02\u{9d}\u{9c}\x03\x02\x02\x02\u{9e}\u{a1}\x03\x02\x02\x02\u{9f}\
	\u{9d}\x03\x02\x02\x02\u{9f}\u{a0}\x03\x02\x02\x02\u{a0}\u{ae}\x03\x02\x02\
	\x02\u{a1}\u{9f}\x03\x02\x02\x02\u{a2}\u{aa}\x05\x04\x03\x02\u{a3}\u{a4}\
	\x07\x16\x02\x02\u{a4}\u{a5}\x05\x2e\x18\x02\u{a5}\u{a6}\x07\x17\x02\x02\
	\u{a6}\u{a9}\x03\x02\x02\x02\u{a7}\u{a9}\x09\x02\x02\x02\u{a8}\u{a3}\x03\
	\x02\x02\x02\u{a8}\u{a7}\x03\x02\x02\x02\u{a9}\u{ac}\x03\x02\x02\x02\u{aa}\
	\u{a8}\x03\x02\x02\x02\u{aa}\u{ab}\x03\x02\x02\x02\u{ab}\u{ae}\x03\x02\x02\
	\x02\u{ac}\u{aa}\x03\x02\x02\x02\u{ad}\u{9b}\x03\x02\x02\x02\u{ad}\u{a2}\
	\x03\x02\x02\x02\u{ae}\x09\x03\x02\x02\x02\u{af}\u{b4}\x05\x28\x15\x02\u{b0}\
	\u{b1}\x07\x18\x02\x02\u{b1}\u{b3}\x05\x28\x15\x02\u{b2}\u{b0}\x03\x02\x02\
	\x02\u{b3}\u{b6}\x03\x02\x02\x02\u{b4}\u{b2}\x03\x02\x02\x02\u{b4}\u{b5}\
	\x03\x02\x02\x02\u{b5}\x0b\x03\x02\x02\x02\u{b6}\u{b4}\x03\x02\x02\x02\u{b7}\
	\u{b9}\x09\x02\x02\x02\u{b8}\u{b7}\x03\x02\x02\x02\u{b9}\u{bc}\x03\x02\x02\
	\x02\u{ba}\u{b8}\x03\x02\x02\x02\u{ba}\u{bb}\x03\x02\x02\x02\u{bb}\u{c6}\
	\x03\x02\x02\x02\u{bc}\u{ba}\x03\x02\x02\x02\u{bd}\u{c7}\x05\x08\x05\x02\
	\u{be}\u{bf}\x05\x0e\x08\x02\u{bf}\u{c0}\x05\x10\x09\x02\u{c0}\u{c7}\x03\
	\x02\x02\x02\u{c1}\u{c2}\x07\x4c\x02\x02\u{c2}\u{c3}\x07\x12\x02\x02\u{c3}\
	\u{c4}\x05\u{86}\x44\x02\u{c4}\u{c5}\x07\x13\x02\x02\u{c5}\u{c7}\x03\x02\
	\x02\x02\u{c6}\u{bd}\x03\x02\x02\x02\u{c6}\u{be}\x03\x02\x02\x02\u{c6}\u{c1}\
	\x03\x02\x02\x02\u{c7}\x0d\x03\x02\x02\x02\u{c8}\u{c9}\x09\x03\x02\x02\u{c9}\
	\x0f\x03\x02\x02\x02\u{ca}\u{cd}\x05\x0c\x07\x02\u{cb}\u{cd}\x07\x55\x02\
	\x02\u{cc}\u{ca}\x03\x02\x02\x02\u{cc}\u{cb}\x03\x02\x02\x02\u{cd}\x11\x03\
	\x02\x02\x02\u{ce}\u{d3}\x05\x10\x09\x02\u{cf}\u{d0}\x09\x04\x02\x02\u{d0}\
	\u{d2}\x05\x10\x09\x02\u{d1}\u{cf}\x03\x02\x02\x02\u{d2}\u{d5}\x03\x02\x02\
	\x02\u{d3}\u{d1}\x03\x02\x02\x02\u{d3}\u{d4}\x03\x02\x02\x02\u{d4}\x13\x03\
	\x02\x02\x02\u{d5}\u{d3}\x03\x02\x02\x02\u{d6}\u{db}\x05\x12\x0a\x02\u{d7}\
	\u{d8}\x09\x05\x02\x02\u{d8}\u{da}\x05\x12\x0a\x02\u{d9}\u{d7}\x03\x02\x02\
	\x02\u{da}\u{dd}\x03\x02\x02\x02\u{db}\u{d9}\x03\x02\x02\x02\u{db}\u{dc}\
	\x03\x02\x02\x02\u{dc}\x15\x03\x02\x02\x02\u{dd}\u{db}\x03\x02\x02\x02\u{de}\
	\u{e3}\x05\x14\x0b\x02\u{df}\u{e0}\x09\x06\x02\x02\u{e0}\u{e2}\x05\x14\x0b\
	\x02\u{e1}\u{df}\x03\x02\x02\x02\u{e2}\u{e5}\x03\x02\x02\x02\u{e3}\u{e1}\
	\x03\x02\x02\x02\u{e3}\u{e4}\x03\x02\x02\x02\u{e4}\x17\x03\x02\x02\x02\u{e5}\
	\u{e3}\x03\x02\x02\x02\u{e6}\u{eb}\x05\x16\x0c\x02\u{e7}\u{e8}\x09\x07\x02\
	\x02\u{e8}\u{ea}\x05\x16\x0c\x02\u{e9}\u{e7}\x03\x02\x02\x02\u{ea}\u{ed}\
	\x03\x02\x02\x02\u{eb}\u{e9}\x03\x02\x02\x02\u{eb}\u{ec}\x03\x02\x02\x02\
	\u{ec}\x19\x03\x02\x02\x02\u{ed}\u{eb}\x03\x02\x02\x02\u{ee}\u{f3}\x05\x18\
	\x0d\x02\u{ef}\u{f0}\x09\x08\x02\x02\u{f0}\u{f2}\x05\x18\x0d\x02\u{f1}\u{ef}\
	\x03\x02\x02\x02\u{f2}\u{f5}\x03\x02\x02\x02\u{f3}\u{f1}\x03\x02\x02\x02\
	\u{f3}\u{f4}\x03\x02\x02\x02\u{f4}\x1b\x03\x02\x02\x02\u{f5}\u{f3}\x03\x02\
	\x02\x02\u{f6}\u{fb}\x05\x1a\x0e\x02\u{f7}\u{f8}\x07\x24\x02\x02\u{f8}\u{fa}\
	\x05\x1a\x0e\x02\u{f9}\u{f7}\x03\x02\x02\x02\u{fa}\u{fd}\x03\x02\x02\x02\
	\u{fb}\u{f9}\x03\x02\x02\x02\u{fb}\u{fc}\x03\x02\x02\x02\u{fc}\x1d\x03\x02\
	\x02\x02\u{fd}\u{fb}\x03\x02\x02\x02\u{fe}\u{103}\x05\x1c\x0f\x02\u{ff}\
	\u{100}\x07\x03\x02\x02\u{100}\u{102}\x05\x1c\x0f\x02\u{101}\u{ff}\x03\x02\
	\x02\x02\u{102}\u{105}\x03\x02\x02\x02\u{103}\u{101}\x03\x02\x02\x02\u{103}\
	\u{104}\x03\x02\x02\x02\u{104}\x1f\x03\x02\x02\x02\u{105}\u{103}\x03\x02\
	\x02\x02\u{106}\u{10b}\x05\x1e\x10\x02\u{107}\u{108}\x07\x22\x02\x02\u{108}\
	\u{10a}\x05\x1e\x10\x02\u{109}\u{107}\x03\x02\x02\x02\u{10a}\u{10d}\x03\
	\x02\x02\x02\u{10b}\u{109}\x03\x02\x02\x02\u{10b}\u{10c}\x03\x02\x02\x02\
	\u{10c}\x21\x03\x02\x02\x02\u{10d}\u{10b}\x03\x02\x02\x02\u{10e}\u{113}\
	\x05\x20\x11\x02\u{10f}\u{110}\x07\x25\x02\x02\u{110}\u{112}\x05\x20\x11\
	\x02\u{111}\u{10f}\x03\x02\x02\x02\u{112}\u{115}\x03\x02\x02\x02\u{113}\
	\u{111}\x03\x02\x02\x02\u{113}\u{114}\x03\x02\x02\x02\u{114}\x23\x03\x02\
	\x02\x02\u{115}\u{113}\x03\x02\x02\x02\u{116}\u{11b}\x05\x22\x12\x02\u{117}\
	\u{118}\x07\x23\x02\x02\u{118}\u{11a}\x05\x22\x12\x02\u{119}\u{117}\x03\
	\x02\x02\x02\u{11a}\u{11d}\x03\x02\x02\x02\u{11b}\u{119}\x03\x02\x02\x02\
	\u{11b}\u{11c}\x03\x02\x02\x02\u{11c}\x25\x03\x02\x02\x02\u{11d}\u{11b}\
	\x03\x02\x02\x02\u{11e}\u{124}\x05\x24\x13\x02\u{11f}\u{120}\x07\x1f\x02\
	\x02\u{120}\u{121}\x05\x2e\x18\x02\u{121}\u{122}\x07\x1a\x02\x02\u{122}\
	\u{123}\x05\x26\x14\x02\u{123}\u{125}\x03\x02\x02\x02\u{124}\u{11f}\x03\
	\x02\x02\x02\u{124}\u{125}\x03\x02\x02\x02\u{125}\x27\x03\x02\x02\x02\u{126}\
	\u{12d}\x05\x26\x14\x02\u{127}\u{128}\x05\x2a\x16\x02\u{128}\u{129}\x05\
	\x2c\x17\x02\u{129}\u{12a}\x05\x28\x15\x02\u{12a}\u{12d}\x03\x02\x02\x02\
	\u{12b}\u{12d}\x07\x55\x02\x02\u{12c}\u{126}\x03\x02\x02\x02\u{12c}\u{127}\
	\x03\x02\x02\x02\u{12c}\u{12b}\x03\x02\x02\x02\u{12d}\x29\x03\x02\x02\x02\
	\u{12e}\u{131}\x07\x52\x02\x02\u{12f}\u{131}\x05\u{86}\x44\x02\u{130}\u{12e}\
	\x03\x02\x02\x02\u{130}\u{12f}\x03\x02\x02\x02\u{131}\x2b\x03\x02\x02\x02\
	\u{132}\u{133}\x09\x09\x02\x02\u{133}\x2d\x03\x02\x02\x02\u{134}\u{139}\
	\x05\x28\x15\x02\u{135}\u{136}\x07\x18\x02\x02\u{136}\u{138}\x05\x28\x15\
	\x02\u{137}\u{135}\x03\x02\x02\x02\u{138}\u{13b}\x03\x02\x02\x02\u{139}\
	\u{137}\x03\x02\x02\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\x2f\x03\x02\
	\x02\x02\u{13b}\u{139}\x03\x02\x02\x02\u{13c}\u{13d}\x05\x26\x14\x02\u{13d}\
	\x31\x03\x02\x02\x02\u{13e}\u{140}\x05\x3a\x1e\x02\u{13f}\u{13e}\x03\x02\
	\x02\x02\u{13f}\u{140}\x03\x02\x02\x02\u{140}\u{141}\x03\x02\x02\x02\u{141}\
	\u{142}\x07\x1b\x02\x02\u{142}\x33\x03\x02\x02\x02\u{143}\u{145}\x05\x38\
	\x1d\x02\u{144}\u{143}\x03\x02\x02\x02\u{145}\u{146}\x03\x02\x02\x02\u{146}\
	\u{144}\x03\x02\x02\x02\u{146}\u{147}\x03\x02\x02\x02\u{147}\x35\x03\x02\
	\x02\x02\u{148}\u{149}\x05\u{84}\x43\x02\u{149}\x37\x03\x02\x02\x02\u{14a}\
	\u{14b}\x05\u{84}\x43\x02\u{14b}\x39\x03\x02\x02\x02\u{14c}\u{151}\x05\x3c\
	\x1f\x02\u{14d}\u{14e}\x07\x18\x02\x02\u{14e}\u{150}\x05\x3c\x1f\x02\u{14f}\
	\u{14d}\x03\x02\x02\x02\u{150}\u{153}\x03\x02\x02\x02\u{151}\u{14f}\x03\
	\x02\x02\x02\u{151}\u{152}\x03\x02\x02\x02\u{152}\x3b\x03\x02\x02\x02\u{153}\
	\u{151}\x03\x02\x02\x02\u{154}\u{157}\x05\u{86}\x44\x02\u{155}\u{156}\x07\
	\x36\x02\x02\u{156}\u{158}\x05\x50\x29\x02\u{157}\u{155}\x03\x02\x02\x02\
	\u{157}\u{158}\x03\x02\x02\x02\u{158}\u{15c}\x03\x02\x02\x02\u{159}\u{15a}\
	\x07\x49\x02\x02\u{15a}\u{15c}\x07\x52\x02\x02\u{15b}\u{154}\x03\x02\x02\
	\x02\u{15b}\u{159}\x03\x02\x02\x02\u{15c}\x3d\x03\x02\x02\x02\u{15d}\u{15f}\
	\x05\u{84}\x43\x02\u{15e}\u{160}\x05\x3e\x20\x02\u{15f}\u{15e}\x03\x02\x02\
	\x02\u{15f}\u{160}\x03\x02\x02\x02\u{160}\x3f\x03\x02\x02\x02\u{161}\u{162}\
	\x05\x42\x22\x02\u{162}\x41\x03\x02\x02\x02\u{163}\u{164}\x08\x22\x01\x02\
	\u{164}\u{16e}\x05\u{86}\x44\x02\u{165}\u{166}\x07\x12\x02\x02\u{166}\u{167}\
	\x05\x40\x21\x02\u{167}\u{168}\x07\x13\x02\x02\u{168}\u{16e}\x03\x02\x02\
	\x02\u{169}\u{16a}\x07\x12\x02\x02\u{16a}\u{16b}\x05\x42\x22\x02\u{16b}\
	\u{16c}\x07\x13\x02\x02\u{16c}\u{16e}\x03\x02\x02\x02\u{16d}\u{163}\x03\
	\x02\x02\x02\u{16d}\u{165}\x03\x02\x02\x02\u{16d}\u{169}\x03\x02\x02\x02\
	\u{16e}\u{180}\x03\x02\x02\x02\u{16f}\u{170}\x0c\x06\x02\x02\u{170}\u{171}\
	\x07\x16\x02\x02\u{171}\u{172}\x07\x1d\x02\x02\u{172}\u{17f}\x07\x17\x02\
	\x02\u{173}\u{174}\x0c\x05\x02\x02\u{174}\u{175}\x07\x12\x02\x02\u{175}\
	\u{176}\x05\x46\x24\x02\u{176}\u{177}\x07\x13\x02\x02\u{177}\u{17f}\x03\
	\x02\x02\x02\u{178}\u{179}\x0c\x04\x02\x02\u{179}\u{17b}\x07\x12\x02\x02\
	\u{17a}\u{17c}\x05\x4c\x27\x02\u{17b}\u{17a}\x03\x02\x02\x02\u{17b}\u{17c}\
	\x03\x02\x02\x02\u{17c}\u{17d}\x03\x02\x02\x02\u{17d}\u{17f}\x07\x13\x02\
	\x02\u{17e}\u{16f}\x03\x02\x02\x02\u{17e}\u{173}\x03\x02\x02\x02\u{17e}\
	\u{178}\x03\x02\x02\x02\u{17f}\u{182}\x03\x02\x02\x02\u{180}\u{17e}\x03\
	\x02\x02\x02\u{180}\u{181}\x03\x02\x02\x02\u{181}\x43\x03\x02\x02\x02\u{182}\
	\u{180}\x03\x02\x02\x02\u{183}\u{189}\x0a\x0a\x02\x02\u{184}\u{185}\x07\
	\x12\x02\x02\u{185}\u{186}\x05\x44\x23\x02\u{186}\u{187}\x07\x13\x02\x02\
	\u{187}\u{189}\x03\x02\x02\x02\u{188}\u{183}\x03\x02\x02\x02\u{188}\u{184}\
	\x03\x02\x02\x02\u{189}\u{18c}\x03\x02\x02\x02\u{18a}\u{188}\x03\x02\x02\
	\x02\u{18a}\u{18b}\x03\x02\x02\x02\u{18b}\x45\x03\x02\x02\x02\u{18c}\u{18a}\
	\x03\x02\x02\x02\u{18d}\u{190}\x05\x48\x25\x02\u{18e}\u{18f}\x07\x18\x02\
	\x02\u{18f}\u{191}\x07\x0e\x02\x02\u{190}\u{18e}\x03\x02\x02\x02\u{190}\
	\u{191}\x03\x02\x02\x02\u{191}\x47\x03\x02\x02\x02\u{192}\u{197}\x05\x4a\
	\x26\x02\u{193}\u{194}\x07\x18\x02\x02\u{194}\u{196}\x05\x4a\x26\x02\u{195}\
	\u{193}\x03\x02\x02\x02\u{196}\u{199}\x03\x02\x02\x02\u{197}\u{195}\x03\
	\x02\x02\x02\u{197}\u{198}\x03\x02\x02\x02\u{198}\x49\x03\x02\x02\x02\u{199}\
	\u{197}\x03\x02\x02\x02\u{19a}\u{19b}\x05\x34\x1b\x02\u{19b}\u{19c}\x05\
	\x40\x21\x02\u{19c}\u{1a2}\x03\x02\x02\x02\u{19d}\u{19f}\x05\x36\x1c\x02\
	\u{19e}\u{1a0}\x05\x4e\x28\x02\u{19f}\u{19e}\x03\x02\x02\x02\u{19f}\u{1a0}\
	\x03\x02\x02\x02\u{1a0}\u{1a2}\x03\x02\x02\x02\u{1a1}\u{19a}\x03\x02\x02\
	\x02\u{1a1}\u{19d}\x03\x02\x02\x02\u{1a2}\x4b\x03\x02\x02\x02\u{1a3}\u{1a8}\
	\x07\x52\x02\x02\u{1a4}\u{1a5}\x07\x18\x02\x02\u{1a5}\u{1a7}\x07\x52\x02\
	\x02\u{1a6}\u{1a4}\x03\x02\x02\x02\u{1a7}\u{1aa}\x03\x02\x02\x02\u{1a8}\
	\u{1a6}\x03\x02\x02\x02\u{1a8}\u{1a9}\x03\x02\x02\x02\u{1a9}\x4d\x03\x02\
	\x02\x02\u{1aa}\u{1a8}\x03\x02\x02\x02\u{1ab}\u{1ac}\x08\x28\x01\x02\u{1ac}\
	\u{1ad}\x07\x16\x02\x02\u{1ad}\u{1ae}\x07\x1d\x02\x02\u{1ae}\u{1af}\x07\
	\x17\x02\x02\u{1af}\u{1b6}\x03\x02\x02\x02\u{1b0}\u{1b1}\x0c\x03\x02\x02\
	\u{1b1}\u{1b2}\x07\x16\x02\x02\u{1b2}\u{1b3}\x07\x1d\x02\x02\u{1b3}\u{1b5}\
	\x07\x17\x02\x02\u{1b4}\u{1b0}\x03\x02\x02\x02\u{1b5}\u{1b8}\x03\x02\x02\
	\x02\u{1b6}\u{1b4}\x03\x02\x02\x02\u{1b6}\u{1b7}\x03\x02\x02\x02\u{1b7}\
	\x4f\x03\x02\x02\x02\u{1b8}\u{1b6}\x03\x02\x02\x02\u{1b9}\u{1c2}\x05\x28\
	\x15\x02\u{1ba}\u{1bb}\x07\x14\x02\x02\u{1bb}\u{1bd}\x05\x52\x2a\x02\u{1bc}\
	\u{1be}\x07\x18\x02\x02\u{1bd}\u{1bc}\x03\x02\x02\x02\u{1bd}\u{1be}\x03\
	\x02\x02\x02\u{1be}\u{1bf}\x03\x02\x02\x02\u{1bf}\u{1c0}\x07\x15\x02\x02\
	\u{1c0}\u{1c2}\x03\x02\x02\x02\u{1c1}\u{1b9}\x03\x02\x02\x02\u{1c1}\u{1ba}\
	\x03\x02\x02\x02\u{1c2}\x51\x03\x02\x02\x02\u{1c3}\u{1c5}\x05\x54\x2b\x02\
	\u{1c4}\u{1c3}\x03\x02\x02\x02\u{1c4}\u{1c5}\x03\x02\x02\x02\u{1c5}\u{1c6}\
	\x03\x02\x02\x02\u{1c6}\u{1ce}\x05\x50\x29\x02\u{1c7}\u{1c9}\x07\x18\x02\
	\x02\u{1c8}\u{1ca}\x05\x54\x2b\x02\u{1c9}\u{1c8}\x03\x02\x02\x02\u{1c9}\
	\u{1ca}\x03\x02\x02\x02\u{1ca}\u{1cb}\x03\x02\x02\x02\u{1cb}\u{1cd}\x05\
	\x50\x29\x02\u{1cc}\u{1c7}\x03\x02\x02\x02\u{1cd}\u{1d0}\x03\x02\x02\x02\
	\u{1ce}\u{1cc}\x03\x02\x02\x02\u{1ce}\u{1cf}\x03\x02\x02\x02\u{1cf}\x53\
	\x03\x02\x02\x02\u{1d0}\u{1ce}\x03\x02\x02\x02\u{1d1}\u{1d2}\x05\x56\x2c\
	\x02\u{1d2}\u{1d3}\x07\x36\x02\x02\u{1d3}\x55\x03\x02\x02\x02\u{1d4}\u{1d6}\
	\x05\x58\x2d\x02\u{1d5}\u{1d4}\x03\x02\x02\x02\u{1d6}\u{1d7}\x03\x02\x02\
	\x02\u{1d7}\u{1d5}\x03\x02\x02\x02\u{1d7}\u{1d8}\x03\x02\x02\x02\u{1d8}\
	\x57\x03\x02\x02\x02\u{1d9}\u{1da}\x07\x16\x02\x02\u{1da}\u{1db}\x05\x30\
	\x19\x02\u{1db}\u{1dc}\x07\x17\x02\x02\u{1dc}\u{1e0}\x03\x02\x02\x02\u{1dd}\
	\u{1de}\x07\x30\x02\x02\u{1de}\u{1e0}\x07\x52\x02\x02\u{1df}\u{1d9}\x03\
	\x02\x02\x02\u{1df}\u{1dd}\x03\x02\x02\x02\u{1e0}\x59\x03\x02\x02\x02\u{1e1}\
	\u{1eb}\x05\x5c\x2f\x02\u{1e2}\u{1eb}\x05\x5e\x30\x02\u{1e3}\u{1eb}\x05\
	\x64\x33\x02\u{1e4}\u{1eb}\x05\x66\x34\x02\u{1e5}\u{1eb}\x05\x68\x35\x02\
	\u{1e6}\u{1eb}\x05\x70\x39\x02\u{1e7}\u{1eb}\x05\x72\x3a\x02\u{1e8}\u{1eb}\
	\x05\x76\x3c\x02\u{1e9}\u{1eb}\x05\x78\x3d\x02\u{1ea}\u{1e1}\x03\x02\x02\
	\x02\u{1ea}\u{1e2}\x03\x02\x02\x02\u{1ea}\u{1e3}\x03\x02\x02\x02\u{1ea}\
	\u{1e4}\x03\x02\x02\x02\u{1ea}\u{1e5}\x03\x02\x02\x02\u{1ea}\u{1e6}\x03\
	\x02\x02\x02\u{1ea}\u{1e7}\x03\x02\x02\x02\u{1ea}\u{1e8}\x03\x02\x02\x02\
	\u{1ea}\u{1e9}\x03\x02\x02\x02\u{1eb}\x5b\x03\x02\x02\x02\u{1ec}\u{1ed}\
	\x07\x52\x02\x02\u{1ed}\u{1ee}\x07\x1a\x02\x02\u{1ee}\u{1f8}\x05\x5a\x2e\
	\x02\u{1ef}\u{1f0}\x07\x48\x02\x02\u{1f0}\u{1f1}\x05\x30\x19\x02\u{1f1}\
	\u{1f2}\x07\x1a\x02\x02\u{1f2}\u{1f3}\x05\x5a\x2e\x02\u{1f3}\u{1f8}\x03\
	\x02\x02\x02\u{1f4}\u{1f5}\x07\x0f\x02\x02\u{1f5}\u{1f6}\x07\x1a\x02\x02\
	\u{1f6}\u{1f8}\x05\x5a\x2e\x02\u{1f7}\u{1ec}\x03\x02\x02\x02\u{1f7}\u{1ef}\
	\x03\x02\x02\x02\u{1f7}\u{1f4}\x03\x02\x02\x02\u{1f8}\x5d\x03\x02\x02\x02\
	\u{1f9}\u{1fb}\x07\x14\x02\x02\u{1fa}\u{1fc}\x05\x60\x31\x02\u{1fb}\u{1fa}\
	\x03\x02\x02\x02\u{1fb}\u{1fc}\x03\x02\x02\x02\u{1fc}\u{1fd}\x03\x02\x02\
	\x02\u{1fd}\u{1fe}\x07\x15\x02\x02\u{1fe}\x5f\x03\x02\x02\x02\u{1ff}\u{201}\
	\x05\x62\x32\x02\u{200}\u{1ff}\x03\x02\x02\x02\u{201}\u{202}\x03\x02\x02\
	\x02\u{202}\u{200}\x03\x02\x02\x02\u{202}\u{203}\x03\x02\x02\x02\u{203}\
	\x61\x03\x02\x02\x02\u{204}\u{208}\x05\x5a\x2e\x02\u{205}\u{208}\x05\x7e\
	\x40\x02\u{206}\u{208}\x05\x32\x1a\x02\u{207}\u{204}\x03\x02\x02\x02\u{207}\
	\u{205}\x03\x02\x02\x02\u{207}\u{206}\x03\x02\x02\x02\u{208}\x63\x03\x02\
	\x02\x02\u{209}\u{20b}\x05\x2e\x18\x02\u{20a}\u{209}\x03\x02\x02\x02\u{20a}\
	\u{20b}\x03\x02\x02\x02\u{20b}\u{20c}\x03\x02\x02\x02\u{20c}\u{20d}\x07\
	\x1b\x02\x02\u{20d}\x65\x03\x02\x02\x02\u{20e}\u{20f}\x07\x3e\x02\x02\u{20f}\
	\u{210}\x07\x12\x02\x02\u{210}\u{211}\x05\x2e\x18\x02\u{211}\u{212}\x07\
	\x13\x02\x02\u{212}\u{215}\x05\x5a\x2e\x02\u{213}\u{214}\x07\x3f\x02\x02\
	\u{214}\u{216}\x05\x5a\x2e\x02\u{215}\u{213}\x03\x02\x02\x02\u{215}\u{216}\
	\x03\x02\x02\x02\u{216}\u{21e}\x03\x02\x02\x02\u{217}\u{218}\x07\x47\x02\
	\x02\u{218}\u{219}\x07\x12\x02\x02\u{219}\u{21a}\x05\x2e\x18\x02\u{21a}\
	\u{21b}\x07\x13\x02\x02\u{21b}\u{21c}\x05\x5a\x2e\x02\u{21c}\u{21e}\x03\
	\x02\x02\x02\u{21d}\u{20e}\x03\x02\x02\x02\u{21d}\u{217}\x03\x02\x02\x02\
	\u{21e}\x67\x03\x02\x02\x02\u{21f}\u{220}\x07\x43\x02\x02\u{220}\u{221}\
	\x07\x12\x02\x02\u{221}\u{222}\x05\x2e\x18\x02\u{222}\u{223}\x07\x13\x02\
	\x02\u{223}\u{224}\x05\x5a\x2e\x02\u{224}\u{234}\x03\x02\x02\x02\u{225}\
	\u{226}\x07\x44\x02\x02\u{226}\u{227}\x05\x5a\x2e\x02\u{227}\u{228}\x07\
	\x43\x02\x02\u{228}\u{229}\x07\x12\x02\x02\u{229}\u{22a}\x05\x2e\x18\x02\
	\u{22a}\u{22b}\x07\x13\x02\x02\u{22b}\u{22c}\x07\x1b\x02\x02\u{22c}\u{234}\
	\x03\x02\x02\x02\u{22d}\u{22e}\x07\x42\x02\x02\u{22e}\u{22f}\x07\x12\x02\
	\x02\u{22f}\u{230}\x05\x6a\x36\x02\u{230}\u{231}\x07\x13\x02\x02\u{231}\
	\u{232}\x05\x5a\x2e\x02\u{232}\u{234}\x03\x02\x02\x02\u{233}\u{21f}\x03\
	\x02\x02\x02\u{233}\u{225}\x03\x02\x02\x02\u{233}\u{22d}\x03\x02\x02\x02\
	\u{234}\x69\x03\x02\x02\x02\u{235}\u{23a}\x05\x6c\x37\x02\u{236}\u{238}\
	\x05\x2e\x18\x02\u{237}\u{236}\x03\x02\x02\x02\u{237}\u{238}\x03\x02\x02\
	\x02\u{238}\u{23a}\x03\x02\x02\x02\u{239}\u{235}\x03\x02\x02\x02\u{239}\
	\u{237}\x03\x02\x02\x02\u{23a}\u{23b}\x03\x02\x02\x02\u{23b}\u{23d}\x07\
	\x1b\x02\x02\u{23c}\u{23e}\x05\x6e\x38\x02\u{23d}\u{23c}\x03\x02\x02\x02\
	\u{23d}\u{23e}\x03\x02\x02\x02\u{23e}\u{23f}\x03\x02\x02\x02\u{23f}\u{241}\
	\x07\x1b\x02\x02\u{240}\u{242}\x05\x6e\x38\x02\u{241}\u{240}\x03\x02\x02\
	\x02\u{241}\u{242}\x03\x02\x02\x02\u{242}\x6b\x03\x02\x02\x02\u{243}\u{245}\
	\x05\x3a\x1e\x02\u{244}\u{243}\x03\x02\x02\x02\u{244}\u{245}\x03\x02\x02\
	\x02\u{245}\x6d\x03\x02\x02\x02\u{246}\u{24b}\x05\x28\x15\x02\u{247}\u{248}\
	\x07\x18\x02\x02\u{248}\u{24a}\x05\x28\x15\x02\u{249}\u{247}\x03\x02\x02\
	\x02\u{24a}\u{24d}\x03\x02\x02\x02\u{24b}\u{249}\x03\x02\x02\x02\u{24b}\
	\u{24c}\x03\x02\x02\x02\u{24c}\x6f\x03\x02\x02\x02\u{24d}\u{24b}\x03\x02\
	\x02\x02\u{24e}\u{24f}\x07\x45\x02\x02\u{24f}\u{256}\x07\x52\x02\x02\u{250}\
	\u{256}\x09\x0b\x02\x02\u{251}\u{253}\x07\x46\x02\x02\u{252}\u{254}\x05\
	\x2e\x18\x02\u{253}\u{252}\x03\x02\x02\x02\u{253}\u{254}\x03\x02\x02\x02\
	\u{254}\u{256}\x03\x02\x02\x02\u{255}\u{24e}\x03\x02\x02\x02\u{255}\u{250}\
	\x03\x02\x02\x02\u{255}\u{251}\x03\x02\x02\x02\u{256}\u{257}\x03\x02\x02\
	\x02\u{257}\u{258}\x07\x1b\x02\x02\u{258}\x71\x03\x02\x02\x02\u{259}\u{25a}\
	\x07\x50\x02\x02\u{25a}\u{25f}\x05\x74\x3b\x02\u{25b}\u{25c}\x07\x18\x02\
	\x02\u{25c}\u{25e}\x05\x74\x3b\x02\u{25d}\u{25b}\x03\x02\x02\x02\u{25e}\
	\u{261}\x03\x02\x02\x02\u{25f}\u{25d}\x03\x02\x02\x02\u{25f}\u{260}\x03\
	\x02\x02\x02\u{260}\x73\x03\x02\x02\x02\u{261}\u{25f}\x03\x02\x02\x02\u{262}\
	\u{263}\x05\x2e\x18\x02\u{263}\u{264}\x07\x18\x02\x02\u{264}\u{265}\x09\
	\x0c\x02\x02\u{265}\x75\x03\x02\x02\x02\u{266}\u{267}\x07\x52\x02\x02\u{267}\
	\u{26c}\x05\x2e\x18\x02\u{268}\u{269}\x07\x18\x02\x02\u{269}\u{26b}\x05\
	\x2e\x18\x02\u{26a}\u{268}\x03\x02\x02\x02\u{26b}\u{26e}\x03\x02\x02\x02\
	\u{26c}\u{26a}\x03\x02\x02\x02\u{26c}\u{26d}\x03\x02\x02\x02\u{26d}\x77\
	\x03\x02\x02\x02\u{26e}\u{26c}\x03\x02\x02\x02\u{26f}\u{270}\x09\x0d\x02\
	\x02\u{270}\u{271}\x07\x1b\x02\x02\u{271}\x79\x03\x02\x02\x02\u{272}\u{274}\
	\x05\x7c\x3f\x02\u{273}\u{272}\x03\x02\x02\x02\u{274}\u{275}\x03\x02\x02\
	\x02\u{275}\u{273}\x03\x02\x02\x02\u{275}\u{276}\x03\x02\x02\x02\u{276}\
	\x7b\x03\x02\x02\x02\u{277}\u{27c}\x05\x7e\x40\x02\u{278}\u{27c}\x05\x62\
	\x32\x02\u{279}\u{27c}\x05\u{82}\x42\x02\u{27a}\u{27c}\x07\x1b\x02\x02\u{27b}\
	\u{277}\x03\x02\x02\x02\u{27b}\u{278}\x03\x02\x02\x02\u{27b}\u{279}\x03\
	\x02\x02\x02\u{27b}\u{27a}\x03\x02\x02\x02\u{27c}\x7d\x03\x02\x02\x02\u{27d}\
	\u{27e}\x07\x49\x02\x02\u{27e}\u{283}\x07\x52\x02\x02\u{27f}\u{280}\x07\
	\x12\x02\x02\u{280}\u{281}\x05\u{80}\x41\x02\u{281}\u{282}\x07\x13\x02\x02\
	\u{282}\u{284}\x03\x02\x02\x02\u{283}\u{27f}\x03\x02\x02\x02\u{283}\u{284}\
	\x03\x02\x02\x02\u{284}\u{286}\x03\x02\x02\x02\u{285}\u{287}\x05\x5e\x30\
	\x02\u{286}\u{285}\x03\x02\x02\x02\u{286}\u{287}\x03\x02\x02\x02\u{287}\
	\x7f\x03\x02\x02\x02\u{288}\u{28a}\x07\x52\x02\x02\u{289}\u{288}\x03\x02\
	\x02\x02\u{289}\u{28a}\x03\x02\x02\x02\u{28a}\u{28f}\x03\x02\x02\x02\u{28b}\
	\u{28c}\x07\x18\x02\x02\u{28c}\u{28e}\x07\x52\x02\x02\u{28d}\u{28b}\x03\
	\x02\x02\x02\u{28e}\u{291}\x03\x02\x02\x02\u{28f}\u{28d}\x03\x02\x02\x02\
	\u{28f}\u{290}\x03\x02\x02\x02\u{290}\u{81}\x03\x02\x02\x02\u{291}\u{28f}\
	\x03\x02\x02\x02\u{292}\u{296}\x07\x2c\x02\x02\u{293}\u{295}\x09\x0e\x02\
	\x02\u{294}\u{293}\x03\x02\x02\x02\u{295}\u{298}\x03\x02\x02\x02\u{296}\
	\u{294}\x03\x02\x02\x02\u{296}\u{297}\x03\x02\x02\x02\u{297}\u{29a}\x03\
	\x02\x02\x02\u{298}\u{296}\x03\x02\x02\x02\u{299}\u{29b}\x07\x18\x02\x02\
	\u{29a}\u{299}\x03\x02\x02\x02\u{29a}\u{29b}\x03\x02\x02\x02\u{29b}\u{29d}\
	\x03\x02\x02\x02\u{29c}\u{29e}\x05\x5e\x30\x02\u{29d}\u{29c}\x03\x02\x02\
	\x02\u{29d}\u{29e}\x03\x02\x02\x02\u{29e}\u{83}\x03\x02\x02\x02\u{29f}\u{2a0}\
	\x09\x0f\x02\x02\u{2a0}\u{85}\x03\x02\x02\x02\u{2a1}\u{2a2}\x05\u{84}\x43\
	\x02\u{2a2}\u{2a3}\x05\u{88}\x45\x02\u{2a3}\u{2a6}\x03\x02\x02\x02\u{2a4}\
	\u{2a6}\x05\u{88}\x45\x02\u{2a5}\u{2a1}\x03\x02\x02\x02\u{2a5}\u{2a4}\x03\
	\x02\x02\x02\u{2a6}\u{87}\x03\x02\x02\x02\u{2a7}\u{2a9}\x09\x10\x02\x02\
	\u{2a8}\u{2aa}\x07\x32\x02\x02\u{2a9}\u{2a8}\x03\x02\x02\x02\u{2a9}\u{2aa}\
	\x03\x02\x02\x02\u{2aa}\u{89}\x03\x02\x02\x02\x51\u{92}\u{97}\u{9f}\u{a8}\
	\u{aa}\u{ad}\u{b4}\u{ba}\u{c6}\u{cc}\u{d3}\u{db}\u{e3}\u{eb}\u{f3}\u{fb}\
	\u{103}\u{10b}\u{113}\u{11b}\u{124}\u{12c}\u{130}\u{139}\u{13f}\u{146}\u{151}\
	\u{157}\u{15b}\u{15f}\u{16d}\u{17b}\u{17e}\u{180}\u{188}\u{18a}\u{190}\u{197}\
	\u{19f}\u{1a1}\u{1a8}\u{1b6}\u{1bd}\u{1c1}\u{1c4}\u{1c9}\u{1ce}\u{1d7}\u{1df}\
	\u{1ea}\u{1f7}\u{1fb}\u{202}\u{207}\u{20a}\u{215}\u{21d}\u{233}\u{237}\u{239}\
	\u{23d}\u{241}\u{244}\u{24b}\u{253}\u{255}\u{25f}\u{26c}\u{275}\u{27b}\u{283}\
	\u{286}\u{289}\u{28f}\u{296}\u{29a}\u{29d}\u{2a5}\u{2a9}";

