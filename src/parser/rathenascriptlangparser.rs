// Generated from RathenaScriptLang.g4 by ANTLR 4.9.3
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
		pub const LeftParen:isize=10; 
		pub const RightParen:isize=11; 
		pub const LeftBrace:isize=12; 
		pub const RightBrace:isize=13; 
		pub const LeftBracket:isize=14; 
		pub const RightBracket:isize=15; 
		pub const Comma:isize=16; 
		pub const At:isize=17; 
		pub const Colon:isize=18; 
		pub const SemiColon:isize=19; 
		pub const Percent:isize=20; 
		pub const Star:isize=21; 
		pub const Tilde:isize=22; 
		pub const QuestionMark:isize=23; 
		pub const Quote:isize=24; 
		pub const DoubleQuote:isize=25; 
		pub const Underscore:isize=26; 
		pub const BitOr:isize=27; 
		pub const OrOp:isize=28; 
		pub const BitAnd:isize=29; 
		pub const AndOp:isize=30; 
		pub const Slash:isize=31; 
		pub const SlashStar:isize=32; 
		pub const StarSlash:isize=33; 
		pub const DoubleSlash:isize=34; 
		pub const Sharp:isize=35; 
		pub const DoubleSharp:isize=36; 
		pub const Minus:isize=37; 
		pub const DecrementOp:isize=38; 
		pub const Plus:isize=39; 
		pub const IncrementOp:isize=40; 
		pub const Dot:isize=41; 
		pub const DotAt:isize=42; 
		pub const Dollar:isize=43; 
		pub const DollarAt:isize=44; 
		pub const Bang:isize=45; 
		pub const BangEqual:isize=46; 
		pub const Equal:isize=47; 
		pub const DoubleEqual:isize=48; 
		pub const LeftCaret:isize=49; 
		pub const DoubleLeftCaret:isize=50; 
		pub const LeftCaretEqual:isize=51; 
		pub const RightCaret:isize=52; 
		pub const DoubleRightCaret:isize=53; 
		pub const RightCaretEqual:isize=54; 
		pub const PlusEqual:isize=55; 
		pub const MinusEqual:isize=56; 
		pub const MultiplyEqual:isize=57; 
		pub const DivideEqual:isize=58; 
		pub const PercentEqual:isize=59; 
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
		pub const Default:isize=71; 
		pub const Function:isize=72; 
		pub const Break:isize=73; 
		pub const Callfunc:isize=74; 
		pub const Callsub:isize=75; 
		pub const Eof:isize=76; 
		pub const Setarray:isize=77; 
		pub const Copyarray:isize=78; 
		pub const True:isize=79; 
		pub const False:isize=80; 
		pub const Menu:isize=81; 
		pub const Close:isize=82; 
		pub const CloseShop:isize=83; 
		pub const Close2:isize=84; 
		pub const Next:isize=85; 
		pub const Script:isize=86; 
		pub const Identifier:isize=87; 
		pub const Label:isize=88; 
		pub const String:isize=89; 
		pub const Number:isize=90; 
		pub const Whitespace:isize=91; 
		pub const Newline:isize=92; 
		pub const BlockComment:isize=93; 
		pub const LineComment:isize=94;
	pub const RULE_compilationUnit:usize = 0; 
	pub const RULE_primaryExpression:usize = 1; 
	pub const RULE_functionCallExpression:usize = 2; 
	pub const RULE_postfixExpression:usize = 3; 
	pub const RULE_incrementThenLoad:usize = 4; 
	pub const RULE_loadThenIncrement:usize = 5; 
	pub const RULE_argumentExpressionList:usize = 6; 
	pub const RULE_unaryExpression:usize = 7; 
	pub const RULE_unaryOperator:usize = 8; 
	pub const RULE_multiplicativeExpression:usize = 9; 
	pub const RULE_multiplicativeOperator:usize = 10; 
	pub const RULE_additiveExpression:usize = 11; 
	pub const RULE_additiveOperator:usize = 12; 
	pub const RULE_shiftExpression:usize = 13; 
	pub const RULE_shiftOperator:usize = 14; 
	pub const RULE_relationalExpression:usize = 15; 
	pub const RULE_relationalOperator:usize = 16; 
	pub const RULE_equalityExpression:usize = 17; 
	pub const RULE_equalityOperator:usize = 18; 
	pub const RULE_andExpression:usize = 19; 
	pub const RULE_exclusiveOrExpression:usize = 20; 
	pub const RULE_inclusiveOrExpression:usize = 21; 
	pub const RULE_logicalAndExpression:usize = 22; 
	pub const RULE_logicalOrExpression:usize = 23; 
	pub const RULE_conditionalExpression:usize = 24; 
	pub const RULE_assignmentExpression:usize = 25; 
	pub const RULE_assignmentLeftExpression:usize = 26; 
	pub const RULE_assignmentOperator:usize = 27; 
	pub const RULE_constantExpression:usize = 28; 
	pub const RULE_statement:usize = 29; 
	pub const RULE_commandStatement:usize = 30; 
	pub const RULE_menuOptionText:usize = 31; 
	pub const RULE_menuLabel:usize = 32; 
	pub const RULE_labeledStatement:usize = 33; 
	pub const RULE_compoundStatement:usize = 34; 
	pub const RULE_blockItemList:usize = 35; 
	pub const RULE_blockItem:usize = 36; 
	pub const RULE_expressionStatement:usize = 37; 
	pub const RULE_selectionStatement:usize = 38; 
	pub const RULE_switchStatement:usize = 39; 
	pub const RULE_switchBlock:usize = 40; 
	pub const RULE_switchBlockStatementGroup:usize = 41; 
	pub const RULE_switchLabels:usize = 42; 
	pub const RULE_switchLabel:usize = 43; 
	pub const RULE_iterationStatement:usize = 44; 
	pub const RULE_forCondition:usize = 45; 
	pub const RULE_forDeclaration:usize = 46; 
	pub const RULE_forStopExpression:usize = 47; 
	pub const RULE_forExpression:usize = 48; 
	pub const RULE_jumpStatement:usize = 49; 
	pub const RULE_translationUnit:usize = 50; 
	pub const RULE_externalDeclaration:usize = 51; 
	pub const RULE_functionDefinition:usize = 52; 
	pub const RULE_scriptInitialization:usize = 53; 
	pub const RULE_scriptLocation:usize = 54; 
	pub const RULE_scriptXPos:usize = 55; 
	pub const RULE_scriptYPos:usize = 56; 
	pub const RULE_scriptDir:usize = 57; 
	pub const RULE_scriptSprite:usize = 58; 
	pub const RULE_npcInitialization:usize = 59; 
	pub const RULE_scriptName:usize = 60; 
	pub const RULE_npcShopDiscount:usize = 61; 
	pub const RULE_npcShopItem:usize = 62; 
	pub const RULE_npcShopPrice:usize = 63; 
	pub const RULE_scope_specifier:usize = 64; 
	pub const RULE_variable:usize = 65; 
	pub const RULE_variable_name:usize = 66;
	pub const ruleNames: [&'static str; 67] =  [
		"compilationUnit", "primaryExpression", "functionCallExpression", "postfixExpression", 
		"incrementThenLoad", "loadThenIncrement", "argumentExpressionList", "unaryExpression", 
		"unaryOperator", "multiplicativeExpression", "multiplicativeOperator", 
		"additiveExpression", "additiveOperator", "shiftExpression", "shiftOperator", 
		"relationalExpression", "relationalOperator", "equalityExpression", "equalityOperator", 
		"andExpression", "exclusiveOrExpression", "inclusiveOrExpression", "logicalAndExpression", 
		"logicalOrExpression", "conditionalExpression", "assignmentExpression", 
		"assignmentLeftExpression", "assignmentOperator", "constantExpression", 
		"statement", "commandStatement", "menuOptionText", "menuLabel", "labeledStatement", 
		"compoundStatement", "blockItemList", "blockItem", "expressionStatement", 
		"selectionStatement", "switchStatement", "switchBlock", "switchBlockStatementGroup", 
		"switchLabels", "switchLabel", "iterationStatement", "forCondition", "forDeclaration", 
		"forStopExpression", "forExpression", "jumpStatement", "translationUnit", 
		"externalDeclaration", "functionDefinition", "scriptInitialization", "scriptLocation", 
		"scriptXPos", "scriptYPos", "scriptDir", "scriptSprite", "npcInitialization", 
		"scriptName", "npcShopDiscount", "npcShopItem", "npcShopPrice", "scope_specifier", 
		"variable", "variable_name"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;87] = [
		None, Some("'^'"), Some("'<<='"), Some("'>>='"), Some("'&='"), Some("'^='"), 
		Some("'|='"), Some("'continue'"), Some("'duplicate'"), Some("'shop'"), 
		Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), Some("']'"), 
		Some("','"), Some("'@'"), Some("':'"), Some("';'"), Some("'%'"), Some("'*'"), 
		Some("'~'"), Some("'?'"), Some("'''"), Some("'\"'"), Some("'_'"), Some("'|'"), 
		Some("'||'"), Some("'&'"), Some("'&&'"), Some("'/'"), Some("'/*'"), Some("'*/'"), 
		Some("'//'"), Some("'#'"), Some("'##'"), Some("'-'"), Some("'--'"), Some("'+'"), 
		Some("'++'"), Some("'.'"), Some("'.@'"), Some("'$'"), Some("'$@'"), Some("'!'"), 
		Some("'!='"), Some("'='"), Some("'=='"), Some("'<'"), Some("'<<'"), Some("'<='"), 
		Some("'>'"), Some("'>>'"), Some("'>='"), Some("'+='"), Some("'-='"), Some("'*='"), 
		Some("'/='"), Some("'%='"), Some("'if'"), Some("'else'"), Some("'end'"), 
		Some("'set'"), Some("'for'"), Some("'while'"), Some("'do'"), Some("'goto'"), 
		Some("'return'"), Some("'switch'"), Some("'case'"), Some("'default:'"), 
		Some("'function'"), Some("'break'"), Some("'callfunc'"), Some("'callsub'"), 
		Some("'eof'"), Some("'setarray'"), Some("'copyarray'"), Some("'true'"), 
		Some("'false'"), Some("'menu'"), Some("'close'"), Some("'closeshop'"), 
		Some("'close2'"), Some("'next'"), Some("'script'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;95]  = [
		None, None, None, None, None, None, None, None, None, None, Some("LeftParen"), 
		Some("RightParen"), Some("LeftBrace"), Some("RightBrace"), Some("LeftBracket"), 
		Some("RightBracket"), Some("Comma"), Some("At"), Some("Colon"), Some("SemiColon"), 
		Some("Percent"), Some("Star"), Some("Tilde"), Some("QuestionMark"), Some("Quote"), 
		Some("DoubleQuote"), Some("Underscore"), Some("BitOr"), Some("OrOp"), 
		Some("BitAnd"), Some("AndOp"), Some("Slash"), Some("SlashStar"), Some("StarSlash"), 
		Some("DoubleSlash"), Some("Sharp"), Some("DoubleSharp"), Some("Minus"), 
		Some("DecrementOp"), Some("Plus"), Some("IncrementOp"), Some("Dot"), Some("DotAt"), 
		Some("Dollar"), Some("DollarAt"), Some("Bang"), Some("BangEqual"), Some("Equal"), 
		Some("DoubleEqual"), Some("LeftCaret"), Some("DoubleLeftCaret"), Some("LeftCaretEqual"), 
		Some("RightCaret"), Some("DoubleRightCaret"), Some("RightCaretEqual"), 
		Some("PlusEqual"), Some("MinusEqual"), Some("MultiplyEqual"), Some("DivideEqual"), 
		Some("PercentEqual"), Some("If"), Some("Else"), Some("End"), Some("Set"), 
		Some("For"), Some("While"), Some("Do"), Some("Goto"), Some("Return"), 
		Some("Switch"), Some("Case"), Some("Default"), Some("Function"), Some("Break"), 
		Some("Callfunc"), Some("Callsub"), Some("Eof"), Some("Setarray"), Some("Copyarray"), 
		Some("True"), Some("False"), Some("Menu"), Some("Close"), Some("CloseShop"), 
		Some("Close2"), Some("Next"), Some("Script"), Some("Identifier"), Some("Label"), 
		Some("String"), Some("Number"), Some("Whitespace"), Some("Newline"), Some("BlockComment"), 
		Some("LineComment")
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
			recog.base.set_state(134);
			recog.translationUnit()?;

			recog.base.set_state(135);
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

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token True
/// Returns `None` if there is no child corresponding to token True
fn True(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(True, 0)
}
/// Retrieves first TerminalNode corresponding to token False
/// Returns `None` if there is no child corresponding to token False
fn False(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(False, 0)
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
					/*InvokeRule variable*/
					recog.base.set_state(137);
					recog.variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(138);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(139);
					recog.base.match_token(True,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(140);
					recog.base.match_token(False,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(141);
					recog.base.match_token(Number,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(142);
					recog.base.match_token(String,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
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
/// Retrieves first TerminalNode corresponding to token Underscore
/// Returns `None` if there is no child corresponding to token Underscore
fn Underscore(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Underscore, 0)
}
/// Retrieves first TerminalNode corresponding to token Callfunc
/// Returns `None` if there is no child corresponding to token Callfunc
fn Callfunc(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Callfunc, 0)
}
/// Retrieves first TerminalNode corresponding to token String
/// Returns `None` if there is no child corresponding to token String
fn String(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(String, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}
/// Retrieves first TerminalNode corresponding to token Callsub
/// Returns `None` if there is no child corresponding to token Callsub
fn Callsub(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Callsub, 0)
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

			recog.base.set_state(192);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
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
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftParen) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang))) != 0) || ((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & ((1usize << (Callfunc - 74)) | (1usize << (Callsub - 74)) | (1usize << (True - 74)) | (1usize << (False - 74)) | (1usize << (Menu - 74)) | (1usize << (Next - 74)) | (1usize << (Identifier - 74)) | (1usize << (String - 74)) | (1usize << (Number - 74)))) != 0) {
						{
						/*InvokeRule argumentExpressionList*/
						recog.base.set_state(148);
						recog.argumentExpressionList()?;

						}
					}

					recog.base.set_state(151);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(152);
					recog.base.match_token(Underscore,&mut recog.err_handler)?;

					recog.base.set_state(153);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					recog.base.set_state(155);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftParen) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang))) != 0) || ((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & ((1usize << (Callfunc - 74)) | (1usize << (Callsub - 74)) | (1usize << (True - 74)) | (1usize << (False - 74)) | (1usize << (Menu - 74)) | (1usize << (Next - 74)) | (1usize << (Identifier - 74)) | (1usize << (String - 74)) | (1usize << (Number - 74)))) != 0) {
						{
						/*InvokeRule argumentExpressionList*/
						recog.base.set_state(154);
						recog.argumentExpressionList()?;

						}
					}

					recog.base.set_state(157);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(158);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					/*InvokeRule argumentExpressionList*/
					recog.base.set_state(159);
					recog.argumentExpressionList()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(160);
					recog.base.match_token(Callfunc,&mut recog.err_handler)?;

					recog.base.set_state(162);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(161);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(164);
					recog.base.match_token(String,&mut recog.err_handler)?;

					recog.base.set_state(167);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Comma {
						{
						recog.base.set_state(165);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						/*InvokeRule argumentExpressionList*/
						recog.base.set_state(166);
						recog.argumentExpressionList()?;

						}
					}

					recog.base.set_state(169);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(170);
					recog.base.match_token(Callfunc,&mut recog.err_handler)?;

					recog.base.set_state(171);
					recog.base.match_token(String,&mut recog.err_handler)?;

					recog.base.set_state(174);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(172);
							recog.base.match_token(Comma,&mut recog.err_handler)?;

							/*InvokeRule argumentExpressionList*/
							recog.base.set_state(173);
							recog.argumentExpressionList()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(176);
					recog.base.match_token(Callsub,&mut recog.err_handler)?;

					recog.base.set_state(178);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(177);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(180);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(183);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Comma {
						{
						recog.base.set_state(181);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						/*InvokeRule argumentExpressionList*/
						recog.base.set_state(182);
						recog.argumentExpressionList()?;

						}
					}

					recog.base.set_state(185);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(186);
					recog.base.match_token(Callsub,&mut recog.err_handler)?;

					recog.base.set_state(187);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(190);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(188);
							recog.base.match_token(Comma,&mut recog.err_handler)?;

							/*InvokeRule argumentExpressionList*/
							recog.base.set_state(189);
							recog.argumentExpressionList()?;

							}
						}

						_ => {}
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

fn primaryExpression(&self) -> Option<Rc<PrimaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn incrementThenLoad(&self) -> Option<Rc<IncrementThenLoadContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn loadThenIncrement(&self) -> Option<Rc<LoadThenIncrementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn functionCallExpression(&self) -> Option<Rc<FunctionCallExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(202);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule primaryExpression*/
					recog.base.set_state(194);
					recog.primaryExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule incrementThenLoad*/
					recog.base.set_state(195);
					recog.incrementThenLoad()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule loadThenIncrement*/
					recog.base.set_state(196);
					recog.loadThenIncrement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(197);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(198);
					recog.conditionalExpression()?;

					recog.base.set_state(199);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule functionCallExpression*/
					recog.base.set_state(201);
					recog.functionCallExpression()?;

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
//------------------- incrementThenLoad ----------------
pub type IncrementThenLoadContextAll<'input> = IncrementThenLoadContext<'input>;


pub type IncrementThenLoadContext<'input> = BaseParserRuleContext<'input,IncrementThenLoadContextExt<'input>>;

#[derive(Clone)]
pub struct IncrementThenLoadContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for IncrementThenLoadContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for IncrementThenLoadContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_incrementThenLoad(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_incrementThenLoad(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for IncrementThenLoadContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_incrementThenLoad(self);
	}
}

impl<'input> CustomRuleContext<'input> for IncrementThenLoadContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_incrementThenLoad }
	//fn type_rule_index() -> usize where Self: Sized { RULE_incrementThenLoad }
}
antlr_rust::type_id!{IncrementThenLoadContextExt<'a>}

impl<'input> IncrementThenLoadContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IncrementThenLoadContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IncrementThenLoadContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IncrementThenLoadContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<IncrementThenLoadContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IncrementOp
/// Returns `None` if there is no child corresponding to token IncrementOp
fn IncrementOp(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(IncrementOp, 0)
}
/// Retrieves first TerminalNode corresponding to token DecrementOp
/// Returns `None` if there is no child corresponding to token DecrementOp
fn DecrementOp(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DecrementOp, 0)
}

}

impl<'input> IncrementThenLoadContextAttrs<'input> for IncrementThenLoadContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn incrementThenLoad(&mut self,)
	-> Result<Rc<IncrementThenLoadContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IncrementThenLoadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_incrementThenLoad);
        let mut _localctx: Rc<IncrementThenLoadContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(204);
			_la = recog.base.input.la(1);
			if { !(_la==DecrementOp || _la==IncrementOp) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule variable*/
			recog.base.set_state(205);
			recog.variable()?;

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
//------------------- loadThenIncrement ----------------
pub type LoadThenIncrementContextAll<'input> = LoadThenIncrementContext<'input>;


pub type LoadThenIncrementContext<'input> = BaseParserRuleContext<'input,LoadThenIncrementContextExt<'input>>;

#[derive(Clone)]
pub struct LoadThenIncrementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for LoadThenIncrementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for LoadThenIncrementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_loadThenIncrement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_loadThenIncrement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for LoadThenIncrementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_loadThenIncrement(self);
	}
}

impl<'input> CustomRuleContext<'input> for LoadThenIncrementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_loadThenIncrement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_loadThenIncrement }
}
antlr_rust::type_id!{LoadThenIncrementContextExt<'a>}

impl<'input> LoadThenIncrementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LoadThenIncrementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LoadThenIncrementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LoadThenIncrementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<LoadThenIncrementContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IncrementOp
/// Returns `None` if there is no child corresponding to token IncrementOp
fn IncrementOp(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(IncrementOp, 0)
}
/// Retrieves first TerminalNode corresponding to token DecrementOp
/// Returns `None` if there is no child corresponding to token DecrementOp
fn DecrementOp(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DecrementOp, 0)
}

}

impl<'input> LoadThenIncrementContextAttrs<'input> for LoadThenIncrementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn loadThenIncrement(&mut self,)
	-> Result<Rc<LoadThenIncrementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LoadThenIncrementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_loadThenIncrement);
        let mut _localctx: Rc<LoadThenIncrementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variable*/
			recog.base.set_state(207);
			recog.variable()?;

			recog.base.set_state(208);
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

fn conditionalExpression_all(&self) ->  Vec<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn conditionalExpression(&self, i: usize) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Comma)
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
        recog.base.enter_rule(_localctx.clone(), 12, RULE_argumentExpressionList);
        let mut _localctx: Rc<ArgumentExpressionListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalExpression*/
			recog.base.set_state(210);
			recog.conditionalExpression()?;

			recog.base.set_state(215);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(211);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(212);
					recog.conditionalExpression()?;

					}
					} 
				}
				recog.base.set_state(217);
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
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
        recog.base.enter_rule(_localctx.clone(), 14, RULE_unaryExpression);
        let mut _localctx: Rc<UnaryExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(222);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule postfixExpression*/
					recog.base.set_state(218);
					recog.postfixExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule unaryOperator*/
					recog.base.set_state(219);
					recog.unaryOperator()?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(220);
					recog.unaryExpression()?;

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

/// Retrieves first TerminalNode corresponding to token BitAnd
/// Returns `None` if there is no child corresponding to token BitAnd
fn BitAnd(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(BitAnd, 0)
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
        recog.base.enter_rule(_localctx.clone(), 16, RULE_unaryOperator);
        let mut _localctx: Rc<UnaryOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(224);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << Star) | (1usize << Tilde) | (1usize << BitAnd) | (1usize << Minus) | (1usize << Plus) | (1usize << Bang))) != 0)) } {
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

fn unaryExpression_all(&self) ->  Vec<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unaryExpression(&self, i: usize) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn multiplicativeOperator_all(&self) ->  Vec<Rc<MultiplicativeOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplicativeOperator(&self, i: usize) -> Option<Rc<MultiplicativeOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
        recog.base.enter_rule(_localctx.clone(), 18, RULE_multiplicativeExpression);
        let mut _localctx: Rc<MultiplicativeExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unaryExpression*/
			recog.base.set_state(226);
			recog.unaryExpression()?;

			recog.base.set_state(232);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule multiplicativeOperator*/
					recog.base.set_state(227);
					recog.multiplicativeOperator()?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(228);
					recog.unaryExpression()?;

					}
					} 
				}
				recog.base.set_state(234);
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
//------------------- multiplicativeOperator ----------------
pub type MultiplicativeOperatorContextAll<'input> = MultiplicativeOperatorContext<'input>;


pub type MultiplicativeOperatorContext<'input> = BaseParserRuleContext<'input,MultiplicativeOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicativeOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for MultiplicativeOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for MultiplicativeOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_multiplicativeOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_multiplicativeOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for MultiplicativeOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_multiplicativeOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeOperator }
}
antlr_rust::type_id!{MultiplicativeOperatorContextExt<'a>}

impl<'input> MultiplicativeOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiplicativeOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicativeOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplicativeOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<MultiplicativeOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Star, 0)
}
/// Retrieves first TerminalNode corresponding to token Slash
/// Returns `None` if there is no child corresponding to token Slash
fn Slash(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Slash, 0)
}
/// Retrieves first TerminalNode corresponding to token Percent
/// Returns `None` if there is no child corresponding to token Percent
fn Percent(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Percent, 0)
}

}

impl<'input> MultiplicativeOperatorContextAttrs<'input> for MultiplicativeOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiplicativeOperator(&mut self,)
	-> Result<Rc<MultiplicativeOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicativeOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_multiplicativeOperator);
        let mut _localctx: Rc<MultiplicativeOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(235);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << Percent) | (1usize << Star) | (1usize << Slash))) != 0)) } {
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
fn additiveOperator_all(&self) ->  Vec<Rc<AdditiveOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn additiveOperator(&self, i: usize) -> Option<Rc<AdditiveOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
        recog.base.enter_rule(_localctx.clone(), 22, RULE_additiveExpression);
        let mut _localctx: Rc<AdditiveExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule multiplicativeExpression*/
			recog.base.set_state(237);
			recog.multiplicativeExpression()?;

			recog.base.set_state(243);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule additiveOperator*/
					recog.base.set_state(238);
					recog.additiveOperator()?;

					/*InvokeRule multiplicativeExpression*/
					recog.base.set_state(239);
					recog.multiplicativeExpression()?;

					}
					} 
				}
				recog.base.set_state(245);
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
//------------------- additiveOperator ----------------
pub type AdditiveOperatorContextAll<'input> = AdditiveOperatorContext<'input>;


pub type AdditiveOperatorContext<'input> = BaseParserRuleContext<'input,AdditiveOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for AdditiveOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for AdditiveOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_additiveOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_additiveOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for AdditiveOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_additiveOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveOperator }
}
antlr_rust::type_id!{AdditiveOperatorContextExt<'a>}

impl<'input> AdditiveOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AdditiveOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AdditiveOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<AdditiveOperatorContextExt<'input>>{

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

}

impl<'input> AdditiveOperatorContextAttrs<'input> for AdditiveOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn additiveOperator(&mut self,)
	-> Result<Rc<AdditiveOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditiveOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_additiveOperator);
        let mut _localctx: Rc<AdditiveOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(246);
			_la = recog.base.input.la(1);
			if { !(_la==Minus || _la==Plus) } {
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
fn shiftOperator_all(&self) ->  Vec<Rc<ShiftOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shiftOperator(&self, i: usize) -> Option<Rc<ShiftOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
        recog.base.enter_rule(_localctx.clone(), 26, RULE_shiftExpression);
        let mut _localctx: Rc<ShiftExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule additiveExpression*/
			recog.base.set_state(248);
			recog.additiveExpression()?;

			recog.base.set_state(254);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule shiftOperator*/
					recog.base.set_state(249);
					recog.shiftOperator()?;

					/*InvokeRule additiveExpression*/
					recog.base.set_state(250);
					recog.additiveExpression()?;

					}
					} 
				}
				recog.base.set_state(256);
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
//------------------- shiftOperator ----------------
pub type ShiftOperatorContextAll<'input> = ShiftOperatorContext<'input>;


pub type ShiftOperatorContext<'input> = BaseParserRuleContext<'input,ShiftOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct ShiftOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ShiftOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ShiftOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shiftOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_shiftOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ShiftOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_shiftOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShiftOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shiftOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shiftOperator }
}
antlr_rust::type_id!{ShiftOperatorContextExt<'a>}

impl<'input> ShiftOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShiftOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShiftOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShiftOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ShiftOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DoubleLeftCaret
/// Returns `None` if there is no child corresponding to token DoubleLeftCaret
fn DoubleLeftCaret(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleLeftCaret, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleRightCaret
/// Returns `None` if there is no child corresponding to token DoubleRightCaret
fn DoubleRightCaret(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleRightCaret, 0)
}

}

impl<'input> ShiftOperatorContextAttrs<'input> for ShiftOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shiftOperator(&mut self,)
	-> Result<Rc<ShiftOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShiftOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_shiftOperator);
        let mut _localctx: Rc<ShiftOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(257);
			_la = recog.base.input.la(1);
			if { !(_la==DoubleLeftCaret || _la==DoubleRightCaret) } {
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
fn relationalOperator_all(&self) ->  Vec<Rc<RelationalOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relationalOperator(&self, i: usize) -> Option<Rc<RelationalOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
        recog.base.enter_rule(_localctx.clone(), 30, RULE_relationalExpression);
        let mut _localctx: Rc<RelationalExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule shiftExpression*/
			recog.base.set_state(259);
			recog.shiftExpression()?;

			recog.base.set_state(265);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule relationalOperator*/
					recog.base.set_state(260);
					recog.relationalOperator()?;

					/*InvokeRule shiftExpression*/
					recog.base.set_state(261);
					recog.shiftExpression()?;

					}
					} 
				}
				recog.base.set_state(267);
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
//------------------- relationalOperator ----------------
pub type RelationalOperatorContextAll<'input> = RelationalOperatorContext<'input>;


pub type RelationalOperatorContext<'input> = BaseParserRuleContext<'input,RelationalOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct RelationalOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for RelationalOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for RelationalOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationalOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_relationalOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for RelationalOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_relationalOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationalOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relationalOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relationalOperator }
}
antlr_rust::type_id!{RelationalOperatorContextExt<'a>}

impl<'input> RelationalOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationalOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationalOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationalOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<RelationalOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftCaret
/// Returns `None` if there is no child corresponding to token LeftCaret
fn LeftCaret(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftCaret, 0)
}
/// Retrieves first TerminalNode corresponding to token RightCaret
/// Returns `None` if there is no child corresponding to token RightCaret
fn RightCaret(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightCaret, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftCaretEqual
/// Returns `None` if there is no child corresponding to token LeftCaretEqual
fn LeftCaretEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftCaretEqual, 0)
}
/// Retrieves first TerminalNode corresponding to token RightCaretEqual
/// Returns `None` if there is no child corresponding to token RightCaretEqual
fn RightCaretEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightCaretEqual, 0)
}

}

impl<'input> RelationalOperatorContextAttrs<'input> for RelationalOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relationalOperator(&mut self,)
	-> Result<Rc<RelationalOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationalOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_relationalOperator);
        let mut _localctx: Rc<RelationalOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(268);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftCaret) | (1usize << LeftCaretEqual) | (1usize << RightCaret) | (1usize << RightCaretEqual))) != 0)) } {
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
fn equalityOperator_all(&self) ->  Vec<Rc<EqualityOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equalityOperator(&self, i: usize) -> Option<Rc<EqualityOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
        recog.base.enter_rule(_localctx.clone(), 34, RULE_equalityExpression);
        let mut _localctx: Rc<EqualityExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relationalExpression*/
			recog.base.set_state(270);
			recog.relationalExpression()?;

			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule equalityOperator*/
					recog.base.set_state(271);
					recog.equalityOperator()?;

					/*InvokeRule relationalExpression*/
					recog.base.set_state(272);
					recog.relationalExpression()?;

					}
					} 
				}
				recog.base.set_state(278);
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
//------------------- equalityOperator ----------------
pub type EqualityOperatorContextAll<'input> = EqualityOperatorContext<'input>;


pub type EqualityOperatorContext<'input> = BaseParserRuleContext<'input,EqualityOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for EqualityOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for EqualityOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_equalityOperator(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_equalityOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for EqualityOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_equalityOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityOperator }
}
antlr_rust::type_id!{EqualityOperatorContextExt<'a>}

impl<'input> EqualityOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqualityOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityOperatorContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<EqualityOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DoubleEqual
/// Returns `None` if there is no child corresponding to token DoubleEqual
fn DoubleEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DoubleEqual, 0)
}
/// Retrieves first TerminalNode corresponding to token BangEqual
/// Returns `None` if there is no child corresponding to token BangEqual
fn BangEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(BangEqual, 0)
}

}

impl<'input> EqualityOperatorContextAttrs<'input> for EqualityOperatorContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equalityOperator(&mut self,)
	-> Result<Rc<EqualityOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_equalityOperator);
        let mut _localctx: Rc<EqualityOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(279);
			_la = recog.base.input.la(1);
			if { !(_la==BangEqual || _la==DoubleEqual) } {
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
/// Retrieves all `TerminalNode`s corresponding to token BitAnd in current rule
fn BitAnd_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(BitAnd)
}
/// Retrieves 'i's TerminalNode corresponding to token BitAnd, starting from 0.
/// Returns `None` if number of children corresponding to token BitAnd is less or equal than `i`.
fn BitAnd(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(BitAnd, i)
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
        recog.base.enter_rule(_localctx.clone(), 38, RULE_andExpression);
        let mut _localctx: Rc<AndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule equalityExpression*/
			recog.base.set_state(281);
			recog.equalityExpression()?;

			recog.base.set_state(286);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(282);
					recog.base.match_token(BitAnd,&mut recog.err_handler)?;

					/*InvokeRule equalityExpression*/
					recog.base.set_state(283);
					recog.equalityExpression()?;

					}
					} 
				}
				recog.base.set_state(288);
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
        recog.base.enter_rule(_localctx.clone(), 40, RULE_exclusiveOrExpression);
        let mut _localctx: Rc<ExclusiveOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule andExpression*/
			recog.base.set_state(289);
			recog.andExpression()?;

			recog.base.set_state(294);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(290);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule andExpression*/
					recog.base.set_state(291);
					recog.andExpression()?;

					}
					} 
				}
				recog.base.set_state(296);
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
/// Retrieves all `TerminalNode`s corresponding to token BitOr in current rule
fn BitOr_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(BitOr)
}
/// Retrieves 'i's TerminalNode corresponding to token BitOr, starting from 0.
/// Returns `None` if number of children corresponding to token BitOr is less or equal than `i`.
fn BitOr(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(BitOr, i)
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
        recog.base.enter_rule(_localctx.clone(), 42, RULE_inclusiveOrExpression);
        let mut _localctx: Rc<InclusiveOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule exclusiveOrExpression*/
			recog.base.set_state(297);
			recog.exclusiveOrExpression()?;

			recog.base.set_state(302);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(298);
					recog.base.match_token(BitOr,&mut recog.err_handler)?;

					/*InvokeRule exclusiveOrExpression*/
					recog.base.set_state(299);
					recog.exclusiveOrExpression()?;

					}
					} 
				}
				recog.base.set_state(304);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
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
	self.children_of_given_type(AndOp)
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
        recog.base.enter_rule(_localctx.clone(), 44, RULE_logicalAndExpression);
        let mut _localctx: Rc<LogicalAndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inclusiveOrExpression*/
			recog.base.set_state(305);
			recog.inclusiveOrExpression()?;

			recog.base.set_state(310);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(306);
					recog.base.match_token(AndOp,&mut recog.err_handler)?;

					/*InvokeRule inclusiveOrExpression*/
					recog.base.set_state(307);
					recog.inclusiveOrExpression()?;

					}
					} 
				}
				recog.base.set_state(312);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
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
	self.children_of_given_type(OrOp)
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
        recog.base.enter_rule(_localctx.clone(), 46, RULE_logicalOrExpression);
        let mut _localctx: Rc<LogicalOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logicalAndExpression*/
			recog.base.set_state(313);
			recog.logicalAndExpression()?;

			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(22,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(314);
					recog.base.match_token(OrOp,&mut recog.err_handler)?;

					/*InvokeRule logicalAndExpression*/
					recog.base.set_state(315);
					recog.logicalAndExpression()?;

					}
					} 
				}
				recog.base.set_state(320);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(22,&mut recog.base)?;
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
fn conditionalExpression_all(&self) ->  Vec<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn conditionalExpression(&self, i: usize) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Colon, 0)
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
        recog.base.enter_rule(_localctx.clone(), 48, RULE_conditionalExpression);
        let mut _localctx: Rc<ConditionalExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logicalOrExpression*/
			recog.base.set_state(321);
			recog.logicalOrExpression()?;

			recog.base.set_state(327);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(322);
					recog.base.match_token(QuestionMark,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(323);
					recog.conditionalExpression()?;

					recog.base.set_state(324);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(325);
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
fn assignmentLeftExpression_all(&self) ->  Vec<Rc<AssignmentLeftExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentLeftExpression(&self, i: usize) -> Option<Rc<AssignmentLeftExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn assignmentOperator_all(&self) ->  Vec<Rc<AssignmentOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentOperator(&self, i: usize) -> Option<Rc<AssignmentOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Set
/// Returns `None` if there is no child corresponding to token Set
fn Set(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Set, 0)
}
fn functionCallExpression(&self) -> Option<Rc<FunctionCallExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Comma)
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
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
/// Retrieves first TerminalNode corresponding to token Setarray
/// Returns `None` if there is no child corresponding to token Setarray
fn Setarray(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Setarray, 0)
}
fn argumentExpressionList(&self) -> Option<Rc<ArgumentExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Copyarray
/// Returns `None` if there is no child corresponding to token Copyarray
fn Copyarray(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Copyarray, 0)
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
        recog.base.enter_rule(_localctx.clone(), 50, RULE_assignmentExpression);
        let mut _localctx: Rc<AssignmentExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(394);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(332); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule assignmentLeftExpression*/
							recog.base.set_state(329);
							recog.assignmentLeftExpression()?;

							/*InvokeRule assignmentOperator*/
							recog.base.set_state(330);
							recog.assignmentOperator()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(334); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(24,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(336);
					recog.conditionalExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(338);
					recog.base.match_token(Set,&mut recog.err_handler)?;

					recog.base.set_state(340);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(339);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule functionCallExpression*/
					recog.base.set_state(342);
					recog.functionCallExpression()?;

					recog.base.set_state(343);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(344);
					recog.conditionalExpression()?;

					recog.base.set_state(346);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(345);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(348);
					recog.base.match_token(Set,&mut recog.err_handler)?;

					recog.base.set_state(350);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(349);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule assignmentLeftExpression*/
					recog.base.set_state(352);
					recog.assignmentLeftExpression()?;

					recog.base.set_state(353);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(354);
					recog.conditionalExpression()?;

					recog.base.set_state(356);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(355);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(358);
					recog.base.match_token(Setarray,&mut recog.err_handler)?;

					recog.base.set_state(360);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(359);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule functionCallExpression*/
					recog.base.set_state(362);
					recog.functionCallExpression()?;

					recog.base.set_state(363);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(364);
					recog.conditionalExpression()?;

					recog.base.set_state(366);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(365);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(368);
					recog.base.match_token(Setarray,&mut recog.err_handler)?;

					recog.base.set_state(370);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(369);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule assignmentLeftExpression*/
					recog.base.set_state(372);
					recog.assignmentLeftExpression()?;

					recog.base.set_state(373);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(374);
					recog.conditionalExpression()?;

					recog.base.set_state(377);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Comma {
						{
						recog.base.set_state(375);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						/*InvokeRule argumentExpressionList*/
						recog.base.set_state(376);
						recog.argumentExpressionList()?;

						}
					}

					recog.base.set_state(380);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(379);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(382);
					recog.base.match_token(Copyarray,&mut recog.err_handler)?;

					recog.base.set_state(384);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LeftParen {
						{
						recog.base.set_state(383);
						recog.base.match_token(LeftParen,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule assignmentLeftExpression*/
					recog.base.set_state(386);
					recog.assignmentLeftExpression()?;

					recog.base.set_state(387);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(388);
					recog.conditionalExpression()?;

					recog.base.set_state(389);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule argumentExpressionList*/
					recog.base.set_state(390);
					recog.argumentExpressionList()?;

					recog.base.set_state(392);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(391);
							recog.base.match_token(RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
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
        recog.base.enter_rule(_localctx.clone(), 52, RULE_assignmentLeftExpression);
        let mut _localctx: Rc<AssignmentLeftExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variable*/
			recog.base.set_state(396);
			recog.variable()?;

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
/// Retrieves first TerminalNode corresponding to token MultiplyEqual
/// Returns `None` if there is no child corresponding to token MultiplyEqual
fn MultiplyEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(MultiplyEqual, 0)
}
/// Retrieves first TerminalNode corresponding to token DivideEqual
/// Returns `None` if there is no child corresponding to token DivideEqual
fn DivideEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(DivideEqual, 0)
}
/// Retrieves first TerminalNode corresponding to token PercentEqual
/// Returns `None` if there is no child corresponding to token PercentEqual
fn PercentEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(PercentEqual, 0)
}
/// Retrieves first TerminalNode corresponding to token PlusEqual
/// Returns `None` if there is no child corresponding to token PlusEqual
fn PlusEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(PlusEqual, 0)
}
/// Retrieves first TerminalNode corresponding to token MinusEqual
/// Returns `None` if there is no child corresponding to token MinusEqual
fn MinusEqual(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(MinusEqual, 0)
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
        recog.base.enter_rule(_localctx.clone(), 54, RULE_assignmentOperator);
        let mut _localctx: Rc<AssignmentOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(398);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__1) | (1usize << T__2) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << Equal) | (1usize << PlusEqual) | (1usize << MinusEqual) | (1usize << MultiplyEqual) | (1usize << DivideEqual) | (1usize << PercentEqual))) != 0)) } {
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
        recog.base.enter_rule(_localctx.clone(), 56, RULE_constantExpression);
        let mut _localctx: Rc<ConstantExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalExpression*/
			recog.base.set_state(400);
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

fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn commandStatement(&self) -> Option<Rc<CommandStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
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
fn labeledStatement(&self) -> Option<Rc<LabeledStatementContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 58, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(411);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule compoundStatement*/
					recog.base.set_state(402);
					recog.compoundStatement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule commandStatement*/
					recog.base.set_state(403);
					recog.commandStatement()?;

					recog.base.set_state(404);
					recog.base.match_token(SemiColon,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expressionStatement*/
					recog.base.set_state(406);
					recog.expressionStatement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule selectionStatement*/
					recog.base.set_state(407);
					recog.selectionStatement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule iterationStatement*/
					recog.base.set_state(408);
					recog.iterationStatement()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule jumpStatement*/
					recog.base.set_state(409);
					recog.jumpStatement()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule labeledStatement*/
					recog.base.set_state(410);
					recog.labeledStatement()?;

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

/// Retrieves first TerminalNode corresponding to token Menu
/// Returns `None` if there is no child corresponding to token Menu
fn Menu(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Menu, 0)
}
fn menuOptionText_all(&self) ->  Vec<Rc<MenuOptionTextContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn menuOptionText(&self, i: usize) -> Option<Rc<MenuOptionTextContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Comma)
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}
fn menuLabel_all(&self) ->  Vec<Rc<MenuLabelContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn menuLabel(&self, i: usize) -> Option<Rc<MenuLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
/// Retrieves first TerminalNode corresponding to token CloseShop
/// Returns `None` if there is no child corresponding to token CloseShop
fn CloseShop(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(CloseShop, 0)
}
/// Retrieves first TerminalNode corresponding to token Next
/// Returns `None` if there is no child corresponding to token Next
fn Next(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Next, 0)
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
        recog.base.enter_rule(_localctx.clone(), 60, RULE_commandStatement);
        let mut _localctx: Rc<CommandStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(431);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Menu 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(413);
					recog.base.match_token(Menu,&mut recog.err_handler)?;

					/*InvokeRule menuOptionText*/
					recog.base.set_state(414);
					recog.menuOptionText()?;

					recog.base.set_state(415);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule menuLabel*/
					recog.base.set_state(416);
					recog.menuLabel()?;

					recog.base.set_state(424);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==Comma {
						{
						{
						recog.base.set_state(417);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						/*InvokeRule menuOptionText*/
						recog.base.set_state(418);
						recog.menuOptionText()?;

						recog.base.set_state(419);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						/*InvokeRule menuLabel*/
						recog.base.set_state(420);
						recog.menuLabel()?;

						}
						}
						recog.base.set_state(426);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 Close 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(427);
					recog.base.match_token(Close,&mut recog.err_handler)?;

					}
				}

			 Close2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(428);
					recog.base.match_token(Close2,&mut recog.err_handler)?;

					}
				}

			 CloseShop 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(429);
					recog.base.match_token(CloseShop,&mut recog.err_handler)?;

					}
				}

			 Next 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(430);
					recog.base.match_token(Next,&mut recog.err_handler)?;

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
//------------------- menuOptionText ----------------
pub type MenuOptionTextContextAll<'input> = MenuOptionTextContext<'input>;


pub type MenuOptionTextContext<'input> = BaseParserRuleContext<'input,MenuOptionTextContextExt<'input>>;

#[derive(Clone)]
pub struct MenuOptionTextContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for MenuOptionTextContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for MenuOptionTextContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_menuOptionText(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_menuOptionText(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for MenuOptionTextContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_menuOptionText(self);
	}
}

impl<'input> CustomRuleContext<'input> for MenuOptionTextContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_menuOptionText }
	//fn type_rule_index() -> usize where Self: Sized { RULE_menuOptionText }
}
antlr_rust::type_id!{MenuOptionTextContextExt<'a>}

impl<'input> MenuOptionTextContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MenuOptionTextContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MenuOptionTextContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MenuOptionTextContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<MenuOptionTextContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token String
/// Returns `None` if there is no child corresponding to token String
fn String(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(String, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MenuOptionTextContextAttrs<'input> for MenuOptionTextContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn menuOptionText(&mut self,)
	-> Result<Rc<MenuOptionTextContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MenuOptionTextContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_menuOptionText);
        let mut _localctx: Rc<MenuOptionTextContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(435);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(433);
					recog.base.match_token(String,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(434);
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
//------------------- menuLabel ----------------
pub type MenuLabelContextAll<'input> = MenuLabelContext<'input>;


pub type MenuLabelContext<'input> = BaseParserRuleContext<'input,MenuLabelContextExt<'input>>;

#[derive(Clone)]
pub struct MenuLabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for MenuLabelContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for MenuLabelContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_menuLabel(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_menuLabel(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for MenuLabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_menuLabel(self);
	}
}

impl<'input> CustomRuleContext<'input> for MenuLabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_menuLabel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_menuLabel }
}
antlr_rust::type_id!{MenuLabelContextExt<'a>}

impl<'input> MenuLabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MenuLabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MenuLabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MenuLabelContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<MenuLabelContextExt<'input>>{

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

impl<'input> MenuLabelContextAttrs<'input> for MenuLabelContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn menuLabel(&mut self,)
	-> Result<Rc<MenuLabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MenuLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_menuLabel);
        let mut _localctx: Rc<MenuLabelContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(437);
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

/// Retrieves first TerminalNode corresponding to token Label
/// Returns `None` if there is no child corresponding to token Label
fn Label(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Label, 0)
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
        recog.base.enter_rule(_localctx.clone(), 66, RULE_labeledStatement);
        let mut _localctx: Rc<LabeledStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(439);
			recog.base.match_token(Label,&mut recog.err_handler)?;

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
        recog.base.enter_rule(_localctx.clone(), 68, RULE_compoundStatement);
        let mut _localctx: Rc<CompoundStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(441);
			recog.base.match_token(LeftBrace,&mut recog.err_handler)?;

			recog.base.set_state(443);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << LeftParen) | (1usize << LeftBrace) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << If) | (1usize << End) | (1usize << Set))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (For - 64)) | (1usize << (While - 64)) | (1usize << (Do - 64)) | (1usize << (Goto - 64)) | (1usize << (Return - 64)) | (1usize << (Switch - 64)) | (1usize << (Function - 64)) | (1usize << (Break - 64)) | (1usize << (Callfunc - 64)) | (1usize << (Callsub - 64)) | (1usize << (Setarray - 64)) | (1usize << (Copyarray - 64)) | (1usize << (True - 64)) | (1usize << (False - 64)) | (1usize << (Menu - 64)) | (1usize << (Close - 64)) | (1usize << (CloseShop - 64)) | (1usize << (Close2 - 64)) | (1usize << (Next - 64)) | (1usize << (Identifier - 64)) | (1usize << (Label - 64)) | (1usize << (String - 64)) | (1usize << (Number - 64)))) != 0) {
				{
				/*InvokeRule blockItemList*/
				recog.base.set_state(442);
				recog.blockItemList()?;

				}
			}

			recog.base.set_state(445);
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
        recog.base.enter_rule(_localctx.clone(), 70, RULE_blockItemList);
        let mut _localctx: Rc<BlockItemListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(448); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule blockItem*/
				recog.base.set_state(447);
				recog.blockItem()?;

				}
				}
				recog.base.set_state(450); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << LeftParen) | (1usize << LeftBrace) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << If) | (1usize << End) | (1usize << Set))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (For - 64)) | (1usize << (While - 64)) | (1usize << (Do - 64)) | (1usize << (Goto - 64)) | (1usize << (Return - 64)) | (1usize << (Switch - 64)) | (1usize << (Function - 64)) | (1usize << (Break - 64)) | (1usize << (Callfunc - 64)) | (1usize << (Callsub - 64)) | (1usize << (Setarray - 64)) | (1usize << (Copyarray - 64)) | (1usize << (True - 64)) | (1usize << (False - 64)) | (1usize << (Menu - 64)) | (1usize << (Close - 64)) | (1usize << (CloseShop - 64)) | (1usize << (Close2 - 64)) | (1usize << (Next - 64)) | (1usize << (Identifier - 64)) | (1usize << (Label - 64)) | (1usize << (String - 64)) | (1usize << (Number - 64)))) != 0)) {break}
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
        recog.base.enter_rule(_localctx.clone(), 72, RULE_blockItem);
        let mut _localctx: Rc<BlockItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(454);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__6 | LeftParen | LeftBrace | At | Star | Tilde | Quote | Underscore |
			 BitAnd | Sharp | DoubleSharp | Minus | DecrementOp | Plus | IncrementOp |
			 Dot | DotAt | Dollar | DollarAt | Bang | If | End | Set | For | While |
			 Do | Goto | Return | Switch | Break | Callfunc | Callsub | Setarray |
			 Copyarray | True | False | Menu | Close | CloseShop | Close2 | Next |
			 Identifier | Label | String | Number 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule statement*/
					recog.base.set_state(452);
					recog.statement()?;

					}
				}

			 Function 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(453);
					recog.functionDefinition()?;

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

fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 74, RULE_expressionStatement);
        let mut _localctx: Rc<ExpressionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(462);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(44,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assignmentExpression*/
					recog.base.set_state(456);
					recog.assignmentExpression()?;

					recog.base.set_state(457);
					recog.base.match_token(SemiColon,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(459);
					recog.conditionalExpression()?;

					recog.base.set_state(460);
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
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
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
fn switchStatement(&self) -> Option<Rc<SwitchStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
        recog.base.enter_rule(_localctx.clone(), 76, RULE_selectionStatement);
        let mut _localctx: Rc<SelectionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(474);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 If 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(464);
					recog.base.match_token(If,&mut recog.err_handler)?;

					recog.base.set_state(465);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(466);
					recog.conditionalExpression()?;

					recog.base.set_state(467);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(468);
					recog.statement()?;

					recog.base.set_state(471);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(469);
							recog.base.match_token(Else,&mut recog.err_handler)?;

							/*InvokeRule statement*/
							recog.base.set_state(470);
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
					/*InvokeRule switchStatement*/
					recog.base.set_state(473);
					recog.switchStatement()?;

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
//------------------- switchStatement ----------------
pub type SwitchStatementContextAll<'input> = SwitchStatementContext<'input>;


pub type SwitchStatementContext<'input> = BaseParserRuleContext<'input,SwitchStatementContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SwitchStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SwitchStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_switchStatement(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_switchStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SwitchStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_switchStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for SwitchStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchStatement }
}
antlr_rust::type_id!{SwitchStatementContextExt<'a>}

impl<'input> SwitchStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchStatementContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SwitchStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Switch
/// Returns `None` if there is no child corresponding to token Switch
fn Switch(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Switch, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftParen, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightParen, 0)
}
fn switchBlock(&self) -> Option<Rc<SwitchBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SwitchStatementContextAttrs<'input> for SwitchStatementContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchStatement(&mut self,)
	-> Result<Rc<SwitchStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_switchStatement);
        let mut _localctx: Rc<SwitchStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(476);
			recog.base.match_token(Switch,&mut recog.err_handler)?;

			recog.base.set_state(477);
			recog.base.match_token(LeftParen,&mut recog.err_handler)?;

			/*InvokeRule conditionalExpression*/
			recog.base.set_state(478);
			recog.conditionalExpression()?;

			recog.base.set_state(479);
			recog.base.match_token(RightParen,&mut recog.err_handler)?;

			/*InvokeRule switchBlock*/
			recog.base.set_state(480);
			recog.switchBlock()?;

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
//------------------- switchBlock ----------------
pub type SwitchBlockContextAll<'input> = SwitchBlockContext<'input>;


pub type SwitchBlockContext<'input> = BaseParserRuleContext<'input,SwitchBlockContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SwitchBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SwitchBlockContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_switchBlock(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_switchBlock(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SwitchBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_switchBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for SwitchBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchBlock }
}
antlr_rust::type_id!{SwitchBlockContextExt<'a>}

impl<'input> SwitchBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchBlockContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SwitchBlockContextExt<'input>>{

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
fn switchBlockStatementGroup_all(&self) ->  Vec<Rc<SwitchBlockStatementGroupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn switchBlockStatementGroup(&self, i: usize) -> Option<Rc<SwitchBlockStatementGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SwitchBlockContextAttrs<'input> for SwitchBlockContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchBlock(&mut self,)
	-> Result<Rc<SwitchBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_switchBlock);
        let mut _localctx: Rc<SwitchBlockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(482);
			recog.base.match_token(LeftBrace,&mut recog.err_handler)?;

			recog.base.set_state(486);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Case || _la==Default {
				{
				{
				/*InvokeRule switchBlockStatementGroup*/
				recog.base.set_state(483);
				recog.switchBlockStatementGroup()?;

				}
				}
				recog.base.set_state(488);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(489);
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
//------------------- switchBlockStatementGroup ----------------
pub type SwitchBlockStatementGroupContextAll<'input> = SwitchBlockStatementGroupContext<'input>;


pub type SwitchBlockStatementGroupContext<'input> = BaseParserRuleContext<'input,SwitchBlockStatementGroupContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchBlockStatementGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SwitchBlockStatementGroupContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SwitchBlockStatementGroupContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_switchBlockStatementGroup(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_switchBlockStatementGroup(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SwitchBlockStatementGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_switchBlockStatementGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for SwitchBlockStatementGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchBlockStatementGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchBlockStatementGroup }
}
antlr_rust::type_id!{SwitchBlockStatementGroupContextExt<'a>}

impl<'input> SwitchBlockStatementGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchBlockStatementGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchBlockStatementGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchBlockStatementGroupContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SwitchBlockStatementGroupContextExt<'input>>{

fn switchLabels(&self) -> Option<Rc<SwitchLabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blockItemList(&self) -> Option<Rc<BlockItemListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SwitchBlockStatementGroupContextAttrs<'input> for SwitchBlockStatementGroupContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchBlockStatementGroup(&mut self,)
	-> Result<Rc<SwitchBlockStatementGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchBlockStatementGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_switchBlockStatementGroup);
        let mut _localctx: Rc<SwitchBlockStatementGroupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule switchLabels*/
			recog.base.set_state(491);
			recog.switchLabels()?;

			/*InvokeRule blockItemList*/
			recog.base.set_state(492);
			recog.blockItemList()?;

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
//------------------- switchLabels ----------------
pub type SwitchLabelsContextAll<'input> = SwitchLabelsContext<'input>;


pub type SwitchLabelsContext<'input> = BaseParserRuleContext<'input,SwitchLabelsContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchLabelsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SwitchLabelsContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SwitchLabelsContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_switchLabels(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_switchLabels(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SwitchLabelsContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_switchLabels(self);
	}
}

impl<'input> CustomRuleContext<'input> for SwitchLabelsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchLabels }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchLabels }
}
antlr_rust::type_id!{SwitchLabelsContextExt<'a>}

impl<'input> SwitchLabelsContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchLabelsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchLabelsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchLabelsContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SwitchLabelsContextExt<'input>>{

fn switchLabel_all(&self) ->  Vec<Rc<SwitchLabelContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn switchLabel(&self, i: usize) -> Option<Rc<SwitchLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SwitchLabelsContextAttrs<'input> for SwitchLabelsContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchLabels(&mut self,)
	-> Result<Rc<SwitchLabelsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchLabelsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_switchLabels);
        let mut _localctx: Rc<SwitchLabelsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule switchLabel*/
			recog.base.set_state(494);
			recog.switchLabel()?;

			recog.base.set_state(498);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Case || _la==Default {
				{
				{
				/*InvokeRule switchLabel*/
				recog.base.set_state(495);
				recog.switchLabel()?;

				}
				}
				recog.base.set_state(500);
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
//------------------- switchLabel ----------------
pub type SwitchLabelContextAll<'input> = SwitchLabelContext<'input>;


pub type SwitchLabelContext<'input> = BaseParserRuleContext<'input,SwitchLabelContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchLabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for SwitchLabelContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for SwitchLabelContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_switchLabel(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_switchLabel(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for SwitchLabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_switchLabel(self);
	}
}

impl<'input> CustomRuleContext<'input> for SwitchLabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchLabel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchLabel }
}
antlr_rust::type_id!{SwitchLabelContextExt<'a>}

impl<'input> SwitchLabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchLabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchLabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchLabelContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<SwitchLabelContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Case
/// Returns `None` if there is no child corresponding to token Case
fn Case(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Case, 0)
}
/// Retrieves first TerminalNode corresponding to token Label
/// Returns `None` if there is no child corresponding to token Label
fn Label(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Label, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Colon, 0)
}
/// Retrieves first TerminalNode corresponding to token Default
/// Returns `None` if there is no child corresponding to token Default
fn Default(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Default, 0)
}

}

impl<'input> SwitchLabelContextAttrs<'input> for SwitchLabelContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchLabel(&mut self,)
	-> Result<Rc<SwitchLabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_switchLabel);
        let mut _localctx: Rc<SwitchLabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(509);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Case 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(501);
					recog.base.match_token(Case,&mut recog.err_handler)?;

					recog.base.set_state(506);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 Label 
						=> {
							{
							recog.base.set_state(502);
							recog.base.match_token(Label,&mut recog.err_handler)?;

							}
						}

					 LeftParen | At | Star | Tilde | Quote | Underscore | BitAnd | Sharp |
					 DoubleSharp | Minus | DecrementOp | Plus | IncrementOp | Dot | DotAt |
					 Dollar | DollarAt | Bang | Callfunc | Callsub | True | False | Menu |
					 Next | Identifier | String | Number 
						=> {
							{
							/*InvokeRule constantExpression*/
							recog.base.set_state(503);
							recog.constantExpression()?;

							recog.base.set_state(504);
							recog.base.match_token(Colon,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 Default 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(508);
					recog.base.match_token(Default,&mut recog.err_handler)?;

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
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 88, RULE_iterationStatement);
        let mut _localctx: Rc<IterationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(531);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 While 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(511);
					recog.base.match_token(While,&mut recog.err_handler)?;

					recog.base.set_state(512);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(513);
					recog.conditionalExpression()?;

					recog.base.set_state(514);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(515);
					recog.statement()?;

					}
				}

			 Do 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(517);
					recog.base.match_token(Do,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(518);
					recog.statement()?;

					recog.base.set_state(519);
					recog.base.match_token(While,&mut recog.err_handler)?;

					recog.base.set_state(520);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule conditionalExpression*/
					recog.base.set_state(521);
					recog.conditionalExpression()?;

					recog.base.set_state(522);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					recog.base.set_state(523);
					recog.base.match_token(SemiColon,&mut recog.err_handler)?;

					}
				}

			 For 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(525);
					recog.base.match_token(For,&mut recog.err_handler)?;

					recog.base.set_state(526);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule forCondition*/
					recog.base.set_state(527);
					recog.forCondition()?;

					recog.base.set_state(528);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(529);
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

fn forDeclaration(&self) -> Option<Rc<ForDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SemiColon in current rule
fn SemiColon_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(SemiColon)
}
/// Retrieves 'i's TerminalNode corresponding to token SemiColon, starting from 0.
/// Returns `None` if number of children corresponding to token SemiColon is less or equal than `i`.
fn SemiColon(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, i)
}
fn forStopExpression(&self) -> Option<Rc<ForStopExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forExpression(&self) -> Option<Rc<ForExpressionContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 90, RULE_forCondition);
        let mut _localctx: Rc<ForConditionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule forDeclaration*/
			recog.base.set_state(533);
			recog.forDeclaration()?;

			recog.base.set_state(534);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			recog.base.set_state(536);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftParen) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << Set))) != 0) || ((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & ((1usize << (Callfunc - 74)) | (1usize << (Callsub - 74)) | (1usize << (Setarray - 74)) | (1usize << (Copyarray - 74)) | (1usize << (True - 74)) | (1usize << (False - 74)) | (1usize << (Menu - 74)) | (1usize << (Next - 74)) | (1usize << (Identifier - 74)) | (1usize << (String - 74)) | (1usize << (Number - 74)))) != 0) {
				{
				/*InvokeRule forStopExpression*/
				recog.base.set_state(535);
				recog.forStopExpression()?;

				}
			}

			recog.base.set_state(538);
			recog.base.match_token(SemiColon,&mut recog.err_handler)?;

			recog.base.set_state(540);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftParen) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang) | (1usize << Set))) != 0) || ((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & ((1usize << (Callfunc - 74)) | (1usize << (Callsub - 74)) | (1usize << (Setarray - 74)) | (1usize << (Copyarray - 74)) | (1usize << (True - 74)) | (1usize << (False - 74)) | (1usize << (Menu - 74)) | (1usize << (Next - 74)) | (1usize << (Identifier - 74)) | (1usize << (String - 74)) | (1usize << (Number - 74)))) != 0) {
				{
				/*InvokeRule forExpression*/
				recog.base.set_state(539);
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

fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 92, RULE_forDeclaration);
        let mut _localctx: Rc<ForDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << At) | (1usize << Quote) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Set))) != 0) || ((((_la - 77)) & !0x3f) == 0 && ((1usize << (_la - 77)) & ((1usize << (Setarray - 77)) | (1usize << (Copyarray - 77)) | (1usize << (Menu - 77)) | (1usize << (Next - 77)) | (1usize << (Identifier - 77)))) != 0) {
				{
				/*InvokeRule assignmentExpression*/
				recog.base.set_state(542);
				recog.assignmentExpression()?;

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
//------------------- forStopExpression ----------------
pub type ForStopExpressionContextAll<'input> = ForStopExpressionContext<'input>;


pub type ForStopExpressionContext<'input> = BaseParserRuleContext<'input,ForStopExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ForStopExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ForStopExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ForStopExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forStopExpression(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_forStopExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ForStopExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_forStopExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForStopExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forStopExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forStopExpression }
}
antlr_rust::type_id!{ForStopExpressionContextExt<'a>}

impl<'input> ForStopExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForStopExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForStopExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForStopExpressionContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ForStopExpressionContextExt<'input>>{

fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForStopExpressionContextAttrs<'input> for ForStopExpressionContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forStopExpression(&mut self,)
	-> Result<Rc<ForStopExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForStopExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_forStopExpression);
        let mut _localctx: Rc<ForStopExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(547);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assignmentExpression*/
					recog.base.set_state(545);
					recog.assignmentExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(546);
					recog.conditionalExpression()?;

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

fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
        recog.base.enter_rule(_localctx.clone(), 96, RULE_forExpression);
        let mut _localctx: Rc<ForExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(551);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assignmentExpression*/
					recog.base.set_state(549);
					recog.assignmentExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(550);
					recog.conditionalExpression()?;

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
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 98, RULE_jumpStatement);
        let mut _localctx: Rc<JumpStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(560);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Goto 
				=> {
					{
					recog.base.set_state(553);
					recog.base.match_token(Goto,&mut recog.err_handler)?;

					recog.base.set_state(554);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

			 T__6 | End | Break 
				=> {
					{
					recog.base.set_state(555);
					_la = recog.base.input.la(1);
					if { !(_la==T__6 || _la==End || _la==Break) } {
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
					recog.base.set_state(556);
					recog.base.match_token(Return,&mut recog.err_handler)?;

					recog.base.set_state(558);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LeftParen) | (1usize << At) | (1usize << Star) | (1usize << Tilde) | (1usize << Quote) | (1usize << Underscore) | (1usize << BitAnd) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Minus) | (1usize << DecrementOp) | (1usize << Plus) | (1usize << IncrementOp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt) | (1usize << Bang))) != 0) || ((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & ((1usize << (Callfunc - 74)) | (1usize << (Callsub - 74)) | (1usize << (True - 74)) | (1usize << (False - 74)) | (1usize << (Menu - 74)) | (1usize << (Next - 74)) | (1usize << (Identifier - 74)) | (1usize << (String - 74)) | (1usize << (Number - 74)))) != 0) {
						{
						/*InvokeRule conditionalExpression*/
						recog.base.set_state(557);
						recog.conditionalExpression()?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(562);
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
        recog.base.enter_rule(_localctx.clone(), 100, RULE_translationUnit);
        let mut _localctx: Rc<TranslationUnitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(565); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule externalDeclaration*/
				recog.base.set_state(564);
				recog.externalDeclaration()?;

				}
				}
				recog.base.set_state(567); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==SemiColon || _la==Minus || _la==Function || _la==Identifier) {break}
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
fn scriptInitialization(&self) -> Option<Rc<ScriptInitializationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn npcInitialization(&self) -> Option<Rc<NpcInitializationContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 102, RULE_externalDeclaration);
        let mut _localctx: Rc<ExternalDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(573);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(60,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(569);
					recog.functionDefinition()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule scriptInitialization*/
					recog.base.set_state(570);
					recog.scriptInitialization()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule npcInitialization*/
					recog.base.set_state(571);
					recog.npcInitialization()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(572);
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
/// Retrieves first TerminalNode corresponding to token Script
/// Returns `None` if there is no child corresponding to token Script
fn Script(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Script, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SemiColon
/// Returns `None` if there is no child corresponding to token SemiColon
fn SemiColon(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(SemiColon, 0)
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
        recog.base.enter_rule(_localctx.clone(), 104, RULE_functionDefinition);
        let mut _localctx: Rc<FunctionDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(585);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(61,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(575);
					recog.base.match_token(Function,&mut recog.err_handler)?;

					recog.base.set_state(576);
					recog.base.match_token(Script,&mut recog.err_handler)?;

					recog.base.set_state(577);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					/*InvokeRule compoundStatement*/
					recog.base.set_state(578);
					recog.compoundStatement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(579);
					recog.base.match_token(Function,&mut recog.err_handler)?;

					recog.base.set_state(580);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					/*InvokeRule compoundStatement*/
					recog.base.set_state(581);
					recog.compoundStatement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(582);
					recog.base.match_token(Function,&mut recog.err_handler)?;

					recog.base.set_state(583);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(584);
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
/// Retrieves first TerminalNode corresponding to token Script
/// Returns `None` if there is no child corresponding to token Script
fn Script(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Script, 0)
}
fn scriptName(&self) -> Option<Rc<ScriptNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Comma)
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptSprite_all(&self) ->  Vec<Rc<ScriptSpriteContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn scriptSprite(&self, i: usize) -> Option<Rc<ScriptSpriteContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn scriptLocation(&self) -> Option<Rc<ScriptLocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptXPos(&self) -> Option<Rc<ScriptXPosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptYPos(&self) -> Option<Rc<ScriptYPosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptDir(&self) -> Option<Rc<ScriptDirContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
        recog.base.enter_rule(_localctx.clone(), 106, RULE_scriptInitialization);
        let mut _localctx: Rc<ScriptInitializationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(621);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Minus 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(587);
					recog.base.match_token(Minus,&mut recog.err_handler)?;

					recog.base.set_state(588);
					recog.base.match_token(Script,&mut recog.err_handler)?;

					/*InvokeRule scriptName*/
					recog.base.set_state(589);
					recog.scriptName()?;

					{
					/*InvokeRule scriptSprite*/
					recog.base.set_state(590);
					recog.scriptSprite()?;

					}
					recog.base.set_state(595);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(591);
							recog.base.match_token(Comma,&mut recog.err_handler)?;

							{
							/*InvokeRule scriptSprite*/
							recog.base.set_state(592);
							recog.scriptSprite()?;

							}
							}
							} 
						}
						recog.base.set_state(597);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
					}
					recog.base.set_state(598);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule compoundStatement*/
					recog.base.set_state(599);
					recog.compoundStatement()?;

					}
				}

			 Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule scriptLocation*/
					recog.base.set_state(601);
					recog.scriptLocation()?;

					recog.base.set_state(602);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptXPos*/
					recog.base.set_state(603);
					recog.scriptXPos()?;

					recog.base.set_state(604);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptYPos*/
					recog.base.set_state(605);
					recog.scriptYPos()?;

					recog.base.set_state(606);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptDir*/
					recog.base.set_state(607);
					recog.scriptDir()?;

					recog.base.set_state(608);
					recog.base.match_token(Script,&mut recog.err_handler)?;

					/*InvokeRule scriptName*/
					recog.base.set_state(609);
					recog.scriptName()?;

					{
					/*InvokeRule scriptSprite*/
					recog.base.set_state(610);
					recog.scriptSprite()?;

					}
					recog.base.set_state(615);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(63,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(611);
							recog.base.match_token(Comma,&mut recog.err_handler)?;

							{
							/*InvokeRule scriptSprite*/
							recog.base.set_state(612);
							recog.scriptSprite()?;

							}
							}
							} 
						}
						recog.base.set_state(617);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(63,&mut recog.base)?;
					}
					recog.base.set_state(618);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule compoundStatement*/
					recog.base.set_state(619);
					recog.compoundStatement()?;

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
//------------------- scriptLocation ----------------
pub type ScriptLocationContextAll<'input> = ScriptLocationContext<'input>;


pub type ScriptLocationContext<'input> = BaseParserRuleContext<'input,ScriptLocationContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptLocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptLocationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptLocationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptLocation(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptLocation(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptLocationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptLocation(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptLocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptLocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptLocation }
}
antlr_rust::type_id!{ScriptLocationContextExt<'a>}

impl<'input> ScriptLocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptLocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptLocationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptLocationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptLocationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> ScriptLocationContextAttrs<'input> for ScriptLocationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptLocation(&mut self,)
	-> Result<Rc<ScriptLocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptLocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_scriptLocation);
        let mut _localctx: Rc<ScriptLocationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(623);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

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
//------------------- scriptXPos ----------------
pub type ScriptXPosContextAll<'input> = ScriptXPosContext<'input>;


pub type ScriptXPosContext<'input> = BaseParserRuleContext<'input,ScriptXPosContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptXPosContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptXPosContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptXPosContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptXPos(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptXPos(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptXPosContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptXPos(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptXPosContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptXPos }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptXPos }
}
antlr_rust::type_id!{ScriptXPosContextExt<'a>}

impl<'input> ScriptXPosContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptXPosContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptXPosContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptXPosContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptXPosContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> ScriptXPosContextAttrs<'input> for ScriptXPosContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptXPos(&mut self,)
	-> Result<Rc<ScriptXPosContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptXPosContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_scriptXPos);
        let mut _localctx: Rc<ScriptXPosContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(625);
			recog.base.match_token(Number,&mut recog.err_handler)?;

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
//------------------- scriptYPos ----------------
pub type ScriptYPosContextAll<'input> = ScriptYPosContext<'input>;


pub type ScriptYPosContext<'input> = BaseParserRuleContext<'input,ScriptYPosContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptYPosContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptYPosContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptYPosContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptYPos(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptYPos(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptYPosContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptYPos(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptYPosContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptYPos }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptYPos }
}
antlr_rust::type_id!{ScriptYPosContextExt<'a>}

impl<'input> ScriptYPosContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptYPosContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptYPosContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptYPosContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptYPosContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> ScriptYPosContextAttrs<'input> for ScriptYPosContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptYPos(&mut self,)
	-> Result<Rc<ScriptYPosContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptYPosContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_scriptYPos);
        let mut _localctx: Rc<ScriptYPosContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(627);
			recog.base.match_token(Number,&mut recog.err_handler)?;

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
//------------------- scriptDir ----------------
pub type ScriptDirContextAll<'input> = ScriptDirContext<'input>;


pub type ScriptDirContext<'input> = BaseParserRuleContext<'input,ScriptDirContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptDirContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptDirContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptDirContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptDir(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptDir(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptDirContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptDir(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptDirContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptDir }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptDir }
}
antlr_rust::type_id!{ScriptDirContextExt<'a>}

impl<'input> ScriptDirContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptDirContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptDirContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptDirContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptDirContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> ScriptDirContextAttrs<'input> for ScriptDirContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptDir(&mut self,)
	-> Result<Rc<ScriptDirContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptDirContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_scriptDir);
        let mut _localctx: Rc<ScriptDirContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(629);
			recog.base.match_token(Number,&mut recog.err_handler)?;

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
//------------------- scriptSprite ----------------
pub type ScriptSpriteContextAll<'input> = ScriptSpriteContext<'input>;


pub type ScriptSpriteContext<'input> = BaseParserRuleContext<'input,ScriptSpriteContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptSpriteContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptSpriteContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptSpriteContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptSprite(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptSprite(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptSpriteContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptSprite(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptSpriteContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptSprite }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptSprite }
}
antlr_rust::type_id!{ScriptSpriteContextExt<'a>}

impl<'input> ScriptSpriteContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptSpriteContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptSpriteContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptSpriteContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptSpriteContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
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

impl<'input> ScriptSpriteContextAttrs<'input> for ScriptSpriteContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptSprite(&mut self,)
	-> Result<Rc<ScriptSpriteContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptSpriteContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_scriptSprite);
        let mut _localctx: Rc<ScriptSpriteContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(636);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Minus | Number 
				=> {
					{
					recog.base.set_state(632);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Minus {
						{
						recog.base.set_state(631);
						recog.base.match_token(Minus,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(634);
					recog.base.match_token(Number,&mut recog.err_handler)?;

					}
				}

			 Identifier 
				=> {
					{
					recog.base.set_state(635);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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
//------------------- npcInitialization ----------------
pub type NpcInitializationContextAll<'input> = NpcInitializationContext<'input>;


pub type NpcInitializationContext<'input> = BaseParserRuleContext<'input,NpcInitializationContextExt<'input>>;

#[derive(Clone)]
pub struct NpcInitializationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for NpcInitializationContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for NpcInitializationContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_npcInitialization(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_npcInitialization(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for NpcInitializationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_npcInitialization(self);
	}
}

impl<'input> CustomRuleContext<'input> for NpcInitializationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_npcInitialization }
	//fn type_rule_index() -> usize where Self: Sized { RULE_npcInitialization }
}
antlr_rust::type_id!{NpcInitializationContextExt<'a>}

impl<'input> NpcInitializationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NpcInitializationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NpcInitializationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NpcInitializationContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<NpcInitializationContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Identifier)
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
fn scriptName_all(&self) ->  Vec<Rc<ScriptNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn scriptName(&self, i: usize) -> Option<Rc<ScriptNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Number in current rule
fn Number_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Number)
}
/// Retrieves 'i's TerminalNode corresponding to token Number, starting from 0.
/// Returns `None` if number of children corresponding to token Number is less or equal than `i`.
fn Number(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Comma)
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}
fn scriptLocation(&self) -> Option<Rc<ScriptLocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptXPos(&self) -> Option<Rc<ScriptXPosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptYPos(&self) -> Option<Rc<ScriptYPosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scriptDir(&self) -> Option<Rc<ScriptDirContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
fn scriptSprite_all(&self) ->  Vec<Rc<ScriptSpriteContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn scriptSprite(&self, i: usize) -> Option<Rc<ScriptSpriteContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn npcShopItem_all(&self) ->  Vec<Rc<NpcShopItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn npcShopItem(&self, i: usize) -> Option<Rc<NpcShopItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Colon in current rule
fn Colon_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Colon)
}
/// Retrieves 'i's TerminalNode corresponding to token Colon, starting from 0.
/// Returns `None` if number of children corresponding to token Colon is less or equal than `i`.
fn Colon(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Colon, i)
}
fn npcShopPrice_all(&self) ->  Vec<Rc<NpcShopPriceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn npcShopPrice(&self, i: usize) -> Option<Rc<NpcShopPriceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn npcShopDiscount(&self) -> Option<Rc<NpcShopDiscountContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NpcInitializationContextAttrs<'input> for NpcInitializationContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn npcInitialization(&mut self,)
	-> Result<Rc<NpcInitializationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NpcInitializationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_npcInitialization);
        let mut _localctx: Rc<NpcInitializationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(707);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(73,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(638);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(643); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(639);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						recog.base.set_state(641);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==Number {
							{
							recog.base.set_state(640);
							recog.base.match_token(Number,&mut recog.err_handler)?;

							}
						}

						}
						}
						recog.base.set_state(645); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==Comma) {break}
					}
					recog.base.set_state(647);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					/*InvokeRule scriptName*/
					recog.base.set_state(648);
					recog.scriptName()?;

					recog.base.set_state(649);
					_la = recog.base.input.la(1);
					if { !(_la==Identifier || _la==Number) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(654);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==Comma {
						{
						{
						recog.base.set_state(650);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						recog.base.set_state(651);
						_la = recog.base.input.la(1);
						if { !(_la==Identifier || _la==Number) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
						}
						recog.base.set_state(656);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule scriptLocation*/
					recog.base.set_state(657);
					recog.scriptLocation()?;

					recog.base.set_state(658);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptXPos*/
					recog.base.set_state(659);
					recog.scriptXPos()?;

					recog.base.set_state(660);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptYPos*/
					recog.base.set_state(661);
					recog.scriptYPos()?;

					recog.base.set_state(662);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptDir*/
					recog.base.set_state(663);
					recog.scriptDir()?;

					recog.base.set_state(664);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					recog.base.set_state(665);
					recog.base.match_token(LeftParen,&mut recog.err_handler)?;

					/*InvokeRule scriptName*/
					recog.base.set_state(666);
					recog.scriptName()?;

					recog.base.set_state(667);
					recog.base.match_token(RightParen,&mut recog.err_handler)?;

					/*InvokeRule scriptName*/
					recog.base.set_state(668);
					recog.scriptName()?;

					{
					/*InvokeRule scriptSprite*/
					recog.base.set_state(669);
					recog.scriptSprite()?;

					}
					recog.base.set_state(674);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==Comma {
						{
						{
						recog.base.set_state(670);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						{
						/*InvokeRule scriptSprite*/
						recog.base.set_state(671);
						recog.scriptSprite()?;

						}
						}
						}
						recog.base.set_state(676);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule scriptLocation*/
					recog.base.set_state(677);
					recog.scriptLocation()?;

					recog.base.set_state(678);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptXPos*/
					recog.base.set_state(679);
					recog.scriptXPos()?;

					recog.base.set_state(680);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptYPos*/
					recog.base.set_state(681);
					recog.scriptYPos()?;

					recog.base.set_state(682);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule scriptDir*/
					recog.base.set_state(683);
					recog.scriptDir()?;

					recog.base.set_state(684);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule scriptName*/
					recog.base.set_state(685);
					recog.scriptName()?;

					/*InvokeRule scriptSprite*/
					recog.base.set_state(686);
					recog.scriptSprite()?;

					recog.base.set_state(687);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					recog.base.set_state(691);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule npcShopDiscount*/
							recog.base.set_state(688);
							recog.npcShopDiscount()?;

							recog.base.set_state(689);
							recog.base.match_token(Comma,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					{
					/*InvokeRule npcShopItem*/
					recog.base.set_state(693);
					recog.npcShopItem()?;

					recog.base.set_state(694);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule npcShopPrice*/
					recog.base.set_state(695);
					recog.npcShopPrice()?;

					}
					recog.base.set_state(704);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==Comma {
						{
						{
						recog.base.set_state(697);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						/*InvokeRule npcShopItem*/
						recog.base.set_state(698);
						recog.npcShopItem()?;

						recog.base.set_state(699);
						recog.base.match_token(Colon,&mut recog.err_handler)?;

						/*InvokeRule npcShopPrice*/
						recog.base.set_state(700);
						recog.npcShopPrice()?;

						}
						}
						recog.base.set_state(706);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
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
//------------------- scriptName ----------------
pub type ScriptNameContextAll<'input> = ScriptNameContext<'input>;


pub type ScriptNameContext<'input> = BaseParserRuleContext<'input,ScriptNameContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for ScriptNameContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for ScriptNameContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scriptName(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_scriptName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for ScriptNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_scriptName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scriptName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scriptName }
}
antlr_rust::type_id!{ScriptNameContextExt<'a>}

impl<'input> ScriptNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptNameContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<ScriptNameContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Identifier)
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Colon in current rule
fn Colon_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Colon)
}
/// Retrieves 'i's TerminalNode corresponding to token Colon, starting from 0.
/// Returns `None` if number of children corresponding to token Colon is less or equal than `i`.
fn Colon(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Colon, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Sharp in current rule
fn Sharp_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Sharp)
}
/// Retrieves 'i's TerminalNode corresponding to token Sharp, starting from 0.
/// Returns `None` if number of children corresponding to token Sharp is less or equal than `i`.
fn Sharp(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Sharp, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Label in current rule
fn Label_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Label)
}
/// Retrieves 'i's TerminalNode corresponding to token Label, starting from 0.
/// Returns `None` if number of children corresponding to token Label is less or equal than `i`.
fn Label(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Label, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Number in current rule
fn Number_all(&self) -> Vec<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>>  where Self:Sized{
	self.children_of_given_type(Number)
}
/// Retrieves 'i's TerminalNode corresponding to token Number, starting from 0.
/// Returns `None` if number of children corresponding to token Number is less or equal than `i`.
fn Number(&self, i: usize) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, i)
}

}

impl<'input> ScriptNameContextAttrs<'input> for ScriptNameContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scriptName(&mut self,)
	-> Result<Rc<ScriptNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_scriptName);
        let mut _localctx: Rc<ScriptNameContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(712);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(74,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(709);
					_la = recog.base.input.la(1);
					if { !(_la==Colon || _la==Sharp || ((((_la - 87)) & !0x3f) == 0 && ((1usize << (_la - 87)) & ((1usize << (Identifier - 87)) | (1usize << (Label - 87)) | (1usize << (Number - 87)))) != 0)) } {
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
				recog.base.set_state(714);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(74,&mut recog.base)?;
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
//------------------- npcShopDiscount ----------------
pub type NpcShopDiscountContextAll<'input> = NpcShopDiscountContext<'input>;


pub type NpcShopDiscountContext<'input> = BaseParserRuleContext<'input,NpcShopDiscountContextExt<'input>>;

#[derive(Clone)]
pub struct NpcShopDiscountContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for NpcShopDiscountContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for NpcShopDiscountContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_npcShopDiscount(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_npcShopDiscount(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for NpcShopDiscountContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_npcShopDiscount(self);
	}
}

impl<'input> CustomRuleContext<'input> for NpcShopDiscountContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_npcShopDiscount }
	//fn type_rule_index() -> usize where Self: Sized { RULE_npcShopDiscount }
}
antlr_rust::type_id!{NpcShopDiscountContextExt<'a>}

impl<'input> NpcShopDiscountContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NpcShopDiscountContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NpcShopDiscountContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NpcShopDiscountContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<NpcShopDiscountContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> NpcShopDiscountContextAttrs<'input> for NpcShopDiscountContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn npcShopDiscount(&mut self,)
	-> Result<Rc<NpcShopDiscountContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NpcShopDiscountContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_npcShopDiscount);
        let mut _localctx: Rc<NpcShopDiscountContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(715);
			recog.base.match_token(Number,&mut recog.err_handler)?;

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
//------------------- npcShopItem ----------------
pub type NpcShopItemContextAll<'input> = NpcShopItemContext<'input>;


pub type NpcShopItemContext<'input> = BaseParserRuleContext<'input,NpcShopItemContextExt<'input>>;

#[derive(Clone)]
pub struct NpcShopItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for NpcShopItemContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for NpcShopItemContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_npcShopItem(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_npcShopItem(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for NpcShopItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_npcShopItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for NpcShopItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_npcShopItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_npcShopItem }
}
antlr_rust::type_id!{NpcShopItemContextExt<'a>}

impl<'input> NpcShopItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NpcShopItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NpcShopItemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NpcShopItemContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<NpcShopItemContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}

}

impl<'input> NpcShopItemContextAttrs<'input> for NpcShopItemContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn npcShopItem(&mut self,)
	-> Result<Rc<NpcShopItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NpcShopItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_npcShopItem);
        let mut _localctx: Rc<NpcShopItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(717);
			recog.base.match_token(Number,&mut recog.err_handler)?;

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
//------------------- npcShopPrice ----------------
pub type NpcShopPriceContextAll<'input> = NpcShopPriceContext<'input>;


pub type NpcShopPriceContext<'input> = BaseParserRuleContext<'input,NpcShopPriceContextExt<'input>>;

#[derive(Clone)]
pub struct NpcShopPriceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RathenaScriptLangParserContext<'input> for NpcShopPriceContext<'input>{}

impl<'input,'a> Listenable<dyn RathenaScriptLangListener<'input> + 'a> for NpcShopPriceContext<'input>{
	fn enter(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_npcShopPrice(self);
	}
	fn exit(&self,listener: &mut (dyn RathenaScriptLangListener<'input> + 'a)) {
		listener.exit_npcShopPrice(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn RathenaScriptLangVisitor<'input> + 'a> for NpcShopPriceContext<'input>{
	fn accept(&self,visitor: &mut (dyn RathenaScriptLangVisitor<'input> + 'a)) {
		visitor.visit_npcShopPrice(self);
	}
}

impl<'input> CustomRuleContext<'input> for NpcShopPriceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RathenaScriptLangParserContextType;
	fn get_rule_index(&self) -> usize { RULE_npcShopPrice }
	//fn type_rule_index() -> usize where Self: Sized { RULE_npcShopPrice }
}
antlr_rust::type_id!{NpcShopPriceContextExt<'a>}

impl<'input> NpcShopPriceContextExt<'input>{
	fn new(parent: Option<Rc<dyn RathenaScriptLangParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NpcShopPriceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NpcShopPriceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NpcShopPriceContextAttrs<'input>: RathenaScriptLangParserContext<'input> + BorrowMut<NpcShopPriceContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Minus, 0)
}

}

impl<'input> NpcShopPriceContextAttrs<'input> for NpcShopPriceContext<'input>{}

impl<'input, I, H> RathenaScriptLangParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn npcShopPrice(&mut self,)
	-> Result<Rc<NpcShopPriceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NpcShopPriceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_npcShopPrice);
        let mut _localctx: Rc<NpcShopPriceContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(720);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Minus {
				{
				recog.base.set_state(719);
				recog.base.match_token(Minus,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(722);
			recog.base.match_token(Number,&mut recog.err_handler)?;

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
        recog.base.enter_rule(_localctx.clone(), 128, RULE_scope_specifier);
        let mut _localctx: Rc<Scope_specifierContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(724);
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

fn variable_name(&self) -> Option<Rc<Variable_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scope_specifier(&self) -> Option<Rc<Scope_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Dollar
/// Returns `None` if there is no child corresponding to token Dollar
fn Dollar(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Dollar, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(LeftBracket, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(RightBracket, 0)
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
        recog.base.enter_rule(_localctx.clone(), 130, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(727);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << At) | (1usize << Quote) | (1usize << Sharp) | (1usize << DoubleSharp) | (1usize << Dot) | (1usize << DotAt) | (1usize << Dollar) | (1usize << DollarAt))) != 0) {
				{
				/*InvokeRule scope_specifier*/
				recog.base.set_state(726);
				recog.scope_specifier()?;

				}
			}

			/*InvokeRule variable_name*/
			recog.base.set_state(729);
			recog.variable_name()?;

			recog.base.set_state(731);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Dollar {
				{
				recog.base.set_state(730);
				recog.base.match_token(Dollar,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(737);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LeftBracket {
				{
				recog.base.set_state(733);
				recog.base.match_token(LeftBracket,&mut recog.err_handler)?;

				/*InvokeRule conditionalExpression*/
				recog.base.set_state(734);
				recog.conditionalExpression()?;

				recog.base.set_state(735);
				recog.base.match_token(RightBracket,&mut recog.err_handler)?;

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
/// Retrieves first TerminalNode corresponding to token Next
/// Returns `None` if there is no child corresponding to token Next
fn Next(&self) -> Option<Rc<TerminalNode<'input,RathenaScriptLangParserContextType>>> where Self:Sized{
	self.get_token(Next, 0)
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
        recog.base.enter_rule(_localctx.clone(), 132, RULE_variable_name);
        let mut _localctx: Rc<Variable_nameContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(739);
			_la = recog.base.input.la(1);
			if { !(((((_la - 81)) & !0x3f) == 0 && ((1usize << (_la - 81)) & ((1usize << (Menu - 81)) | (1usize << (Next - 81)) | (1usize << (Identifier - 81)))) != 0)) } {
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
	\x60\u{2e8}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
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
	\x44\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x05\x03\u{93}\x0a\x03\x03\x04\x03\x04\x03\x04\x05\x04\u{98}\
	\x0a\x04\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\u{9e}\x0a\x04\x03\x04\x03\
	\x04\x03\x04\x03\x04\x03\x04\x05\x04\u{a5}\x0a\x04\x03\x04\x03\x04\x03\x04\
	\x05\x04\u{aa}\x0a\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\u{b1}\
	\x0a\x04\x03\x04\x03\x04\x05\x04\u{b5}\x0a\x04\x03\x04\x03\x04\x03\x04\x05\
	\x04\u{ba}\x0a\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\u{c1}\
	\x0a\x04\x05\x04\u{c3}\x0a\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x05\x05\u{cd}\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x07\
	\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x07\x08\u{d8}\x0a\x08\x0c\x08\x0e\
	\x08\u{db}\x0b\x08\x03\x09\x03\x09\x03\x09\x03\x09\x05\x09\u{e1}\x0a\x09\
	\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x07\x0b\u{e9}\x0a\x0b\x0c\
	\x0b\x0e\x0b\u{ec}\x0b\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x07\x0d\u{f4}\x0a\x0d\x0c\x0d\x0e\x0d\u{f7}\x0b\x0d\x03\x0e\x03\x0e\x03\
	\x0f\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{ff}\x0a\x0f\x0c\x0f\x0e\x0f\u{102}\
	\x0b\x0f\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x11\x07\x11\u{10a}\
	\x0a\x11\x0c\x11\x0e\x11\u{10d}\x0b\x11\x03\x12\x03\x12\x03\x13\x03\x13\
	\x03\x13\x03\x13\x07\x13\u{115}\x0a\x13\x0c\x13\x0e\x13\u{118}\x0b\x13\x03\
	\x14\x03\x14\x03\x15\x03\x15\x03\x15\x07\x15\u{11f}\x0a\x15\x0c\x15\x0e\
	\x15\u{122}\x0b\x15\x03\x16\x03\x16\x03\x16\x07\x16\u{127}\x0a\x16\x0c\x16\
	\x0e\x16\u{12a}\x0b\x16\x03\x17\x03\x17\x03\x17\x07\x17\u{12f}\x0a\x17\x0c\
	\x17\x0e\x17\u{132}\x0b\x17\x03\x18\x03\x18\x03\x18\x07\x18\u{137}\x0a\x18\
	\x0c\x18\x0e\x18\u{13a}\x0b\x18\x03\x19\x03\x19\x03\x19\x07\x19\u{13f}\x0a\
	\x19\x0c\x19\x0e\x19\u{142}\x0b\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x03\x1a\x05\x1a\u{14a}\x0a\x1a\x03\x1b\x03\x1b\x03\x1b\x06\x1b\u{14f}\
	\x0a\x1b\x0d\x1b\x0e\x1b\u{150}\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\
	\u{157}\x0a\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{15d}\x0a\x1b\x03\
	\x1b\x03\x1b\x05\x1b\u{161}\x0a\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\
	\x1b\u{167}\x0a\x1b\x03\x1b\x03\x1b\x05\x1b\u{16b}\x0a\x1b\x03\x1b\x03\x1b\
	\x03\x1b\x03\x1b\x05\x1b\u{171}\x0a\x1b\x03\x1b\x03\x1b\x05\x1b\u{175}\x0a\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{17c}\x0a\x1b\x03\
	\x1b\x05\x1b\u{17f}\x0a\x1b\x03\x1b\x03\x1b\x05\x1b\u{183}\x0a\x1b\x03\x1b\
	\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{18b}\x0a\x1b\x05\x1b\
	\u{18d}\x0a\x1b\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\
	\u{19e}\x0a\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x07\x20\u{1a9}\x0a\x20\x0c\x20\x0e\x20\u{1ac}\x0b\x20\x03\
	\x20\x03\x20\x03\x20\x03\x20\x05\x20\u{1b2}\x0a\x20\x03\x21\x03\x21\x05\
	\x21\u{1b6}\x0a\x21\x03\x22\x03\x22\x03\x23\x03\x23\x03\x24\x03\x24\x05\
	\x24\u{1be}\x0a\x24\x03\x24\x03\x24\x03\x25\x06\x25\u{1c3}\x0a\x25\x0d\x25\
	\x0e\x25\u{1c4}\x03\x26\x03\x26\x05\x26\u{1c9}\x0a\x26\x03\x27\x03\x27\x03\
	\x27\x03\x27\x03\x27\x03\x27\x05\x27\u{1d1}\x0a\x27\x03\x28\x03\x28\x03\
	\x28\x03\x28\x03\x28\x03\x28\x03\x28\x05\x28\u{1da}\x0a\x28\x03\x28\x05\
	\x28\u{1dd}\x0a\x28\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x2a\x03\x2a\x07\x2a\u{1e7}\x0a\x2a\x0c\x2a\x0e\x2a\u{1ea}\x0b\x2a\x03\x2a\
	\x03\x2a\x03\x2b\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x07\x2c\u{1f3}\x0a\x2c\
	\x0c\x2c\x0e\x2c\u{1f6}\x0b\x2c\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x05\x2d\u{1fd}\x0a\x2d\x03\x2d\x05\x2d\u{200}\x0a\x2d\x03\x2e\x03\x2e\x03\
	\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\
	\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x05\
	\x2e\u{216}\x0a\x2e\x03\x2f\x03\x2f\x03\x2f\x05\x2f\u{21b}\x0a\x2f\x03\x2f\
	\x03\x2f\x05\x2f\u{21f}\x0a\x2f\x03\x30\x05\x30\u{222}\x0a\x30\x03\x31\x03\
	\x31\x05\x31\u{226}\x0a\x31\x03\x32\x03\x32\x05\x32\u{22a}\x0a\x32\x03\x33\
	\x03\x33\x03\x33\x03\x33\x03\x33\x05\x33\u{231}\x0a\x33\x05\x33\u{233}\x0a\
	\x33\x03\x33\x03\x33\x03\x34\x06\x34\u{238}\x0a\x34\x0d\x34\x0e\x34\u{239}\
	\x03\x35\x03\x35\x03\x35\x03\x35\x05\x35\u{240}\x0a\x35\x03\x36\x03\x36\
	\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x05\x36\
	\u{24c}\x0a\x36\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x07\x37\
	\u{254}\x0a\x37\x0c\x37\x0e\x37\u{257}\x0b\x37\x03\x37\x03\x37\x03\x37\x03\
	\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\
	\x37\x03\x37\x03\x37\x07\x37\u{268}\x0a\x37\x0c\x37\x0e\x37\u{26b}\x0b\x37\
	\x03\x37\x03\x37\x03\x37\x05\x37\u{270}\x0a\x37\x03\x38\x03\x38\x03\x39\
	\x03\x39\x03\x3a\x03\x3a\x03\x3b\x03\x3b\x03\x3c\x05\x3c\u{27b}\x0a\x3c\
	\x03\x3c\x03\x3c\x05\x3c\u{27f}\x0a\x3c\x03\x3d\x03\x3d\x03\x3d\x05\x3d\
	\u{284}\x0a\x3d\x06\x3d\u{286}\x0a\x3d\x0d\x3d\x0e\x3d\u{287}\x03\x3d\x03\
	\x3d\x03\x3d\x03\x3d\x03\x3d\x07\x3d\u{28f}\x0a\x3d\x0c\x3d\x0e\x3d\u{292}\
	\x0b\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x07\x3d\u{2a3}\
	\x0a\x3d\x0c\x3d\x0e\x3d\u{2a6}\x0b\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x03\x3d\x05\x3d\u{2b6}\x0a\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x07\x3d\u{2c1}\x0a\x3d\x0c\x3d\x0e\x3d\
	\u{2c4}\x0b\x3d\x05\x3d\u{2c6}\x0a\x3d\x03\x3e\x07\x3e\u{2c9}\x0a\x3e\x0c\
	\x3e\x0e\x3e\u{2cc}\x0b\x3e\x03\x3f\x03\x3f\x03\x40\x03\x40\x03\x41\x05\
	\x41\u{2d3}\x0a\x41\x03\x41\x03\x41\x03\x42\x03\x42\x03\x43\x05\x43\u{2da}\
	\x0a\x43\x03\x43\x03\x43\x05\x43\u{2de}\x0a\x43\x03\x43\x03\x43\x03\x43\
	\x03\x43\x05\x43\u{2e4}\x0a\x43\x03\x44\x03\x44\x03\x44\x02\x02\x45\x02\
	\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\
	\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\
	\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\
	\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\x02\x10\x04\x02\
	\x28\x28\x2a\x2a\x07\x02\x17\x18\x1f\x1f\x27\x27\x29\x29\x2f\x2f\x04\x02\
	\x16\x17\x21\x21\x04\x02\x27\x27\x29\x29\x04\x02\x34\x34\x37\x37\x05\x02\
	\x33\x33\x35\x36\x38\x38\x04\x02\x30\x30\x32\x32\x05\x02\x04\x08\x31\x31\
	\x39\x3d\x04\x02\x27\x27\x59\x59\x05\x02\x09\x09\x40\x40\x4b\x4b\x04\x02\
	\x59\x59\x5c\x5c\x06\x02\x14\x14\x25\x25\x59\x5a\x5c\x5c\x06\x02\x13\x13\
	\x1a\x1a\x25\x26\x2b\x2e\x05\x02\x53\x53\x57\x57\x59\x59\x02\u{312}\x02\
	\u{88}\x03\x02\x02\x02\x04\u{92}\x03\x02\x02\x02\x06\u{c2}\x03\x02\x02\x02\
	\x08\u{cc}\x03\x02\x02\x02\x0a\u{ce}\x03\x02\x02\x02\x0c\u{d1}\x03\x02\x02\
	\x02\x0e\u{d4}\x03\x02\x02\x02\x10\u{e0}\x03\x02\x02\x02\x12\u{e2}\x03\x02\
	\x02\x02\x14\u{e4}\x03\x02\x02\x02\x16\u{ed}\x03\x02\x02\x02\x18\u{ef}\x03\
	\x02\x02\x02\x1a\u{f8}\x03\x02\x02\x02\x1c\u{fa}\x03\x02\x02\x02\x1e\u{103}\
	\x03\x02\x02\x02\x20\u{105}\x03\x02\x02\x02\x22\u{10e}\x03\x02\x02\x02\x24\
	\u{110}\x03\x02\x02\x02\x26\u{119}\x03\x02\x02\x02\x28\u{11b}\x03\x02\x02\
	\x02\x2a\u{123}\x03\x02\x02\x02\x2c\u{12b}\x03\x02\x02\x02\x2e\u{133}\x03\
	\x02\x02\x02\x30\u{13b}\x03\x02\x02\x02\x32\u{143}\x03\x02\x02\x02\x34\u{18c}\
	\x03\x02\x02\x02\x36\u{18e}\x03\x02\x02\x02\x38\u{190}\x03\x02\x02\x02\x3a\
	\u{192}\x03\x02\x02\x02\x3c\u{19d}\x03\x02\x02\x02\x3e\u{1b1}\x03\x02\x02\
	\x02\x40\u{1b5}\x03\x02\x02\x02\x42\u{1b7}\x03\x02\x02\x02\x44\u{1b9}\x03\
	\x02\x02\x02\x46\u{1bb}\x03\x02\x02\x02\x48\u{1c2}\x03\x02\x02\x02\x4a\u{1c8}\
	\x03\x02\x02\x02\x4c\u{1d0}\x03\x02\x02\x02\x4e\u{1dc}\x03\x02\x02\x02\x50\
	\u{1de}\x03\x02\x02\x02\x52\u{1e4}\x03\x02\x02\x02\x54\u{1ed}\x03\x02\x02\
	\x02\x56\u{1f0}\x03\x02\x02\x02\x58\u{1ff}\x03\x02\x02\x02\x5a\u{215}\x03\
	\x02\x02\x02\x5c\u{217}\x03\x02\x02\x02\x5e\u{221}\x03\x02\x02\x02\x60\u{225}\
	\x03\x02\x02\x02\x62\u{229}\x03\x02\x02\x02\x64\u{232}\x03\x02\x02\x02\x66\
	\u{237}\x03\x02\x02\x02\x68\u{23f}\x03\x02\x02\x02\x6a\u{24b}\x03\x02\x02\
	\x02\x6c\u{26f}\x03\x02\x02\x02\x6e\u{271}\x03\x02\x02\x02\x70\u{273}\x03\
	\x02\x02\x02\x72\u{275}\x03\x02\x02\x02\x74\u{277}\x03\x02\x02\x02\x76\u{27e}\
	\x03\x02\x02\x02\x78\u{2c5}\x03\x02\x02\x02\x7a\u{2ca}\x03\x02\x02\x02\x7c\
	\u{2cd}\x03\x02\x02\x02\x7e\u{2cf}\x03\x02\x02\x02\u{80}\u{2d2}\x03\x02\
	\x02\x02\u{82}\u{2d6}\x03\x02\x02\x02\u{84}\u{2d9}\x03\x02\x02\x02\u{86}\
	\u{2e5}\x03\x02\x02\x02\u{88}\u{89}\x05\x66\x34\x02\u{89}\u{8a}\x07\x02\
	\x02\x03\u{8a}\x03\x03\x02\x02\x02\u{8b}\u{93}\x05\u{84}\x43\x02\u{8c}\u{93}\
	\x07\x59\x02\x02\u{8d}\u{93}\x07\x51\x02\x02\u{8e}\u{93}\x07\x52\x02\x02\
	\u{8f}\u{93}\x07\x5c\x02\x02\u{90}\u{93}\x07\x5b\x02\x02\u{91}\u{93}\x07\
	\x27\x02\x02\u{92}\u{8b}\x03\x02\x02\x02\u{92}\u{8c}\x03\x02\x02\x02\u{92}\
	\u{8d}\x03\x02\x02\x02\u{92}\u{8e}\x03\x02\x02\x02\u{92}\u{8f}\x03\x02\x02\
	\x02\u{92}\u{90}\x03\x02\x02\x02\u{92}\u{91}\x03\x02\x02\x02\u{93}\x05\x03\
	\x02\x02\x02\u{94}\u{95}\x07\x59\x02\x02\u{95}\u{97}\x07\x0c\x02\x02\u{96}\
	\u{98}\x05\x0e\x08\x02\u{97}\u{96}\x03\x02\x02\x02\u{97}\u{98}\x03\x02\x02\
	\x02\u{98}\u{99}\x03\x02\x02\x02\u{99}\u{c3}\x07\x0d\x02\x02\u{9a}\u{9b}\
	\x07\x1c\x02\x02\u{9b}\u{9d}\x07\x0c\x02\x02\u{9c}\u{9e}\x05\x0e\x08\x02\
	\u{9d}\u{9c}\x03\x02\x02\x02\u{9d}\u{9e}\x03\x02\x02\x02\u{9e}\u{9f}\x03\
	\x02\x02\x02\u{9f}\u{c3}\x07\x0d\x02\x02\u{a0}\u{a1}\x07\x59\x02\x02\u{a1}\
	\u{c3}\x05\x0e\x08\x02\u{a2}\u{a4}\x07\x4c\x02\x02\u{a3}\u{a5}\x07\x0c\x02\
	\x02\u{a4}\u{a3}\x03\x02\x02\x02\u{a4}\u{a5}\x03\x02\x02\x02\u{a5}\u{a6}\
	\x03\x02\x02\x02\u{a6}\u{a9}\x07\x5b\x02\x02\u{a7}\u{a8}\x07\x12\x02\x02\
	\u{a8}\u{aa}\x05\x0e\x08\x02\u{a9}\u{a7}\x03\x02\x02\x02\u{a9}\u{aa}\x03\
	\x02\x02\x02\u{aa}\u{ab}\x03\x02\x02\x02\u{ab}\u{c3}\x07\x0d\x02\x02\u{ac}\
	\u{ad}\x07\x4c\x02\x02\u{ad}\u{b0}\x07\x5b\x02\x02\u{ae}\u{af}\x07\x12\x02\
	\x02\u{af}\u{b1}\x05\x0e\x08\x02\u{b0}\u{ae}\x03\x02\x02\x02\u{b0}\u{b1}\
	\x03\x02\x02\x02\u{b1}\u{c3}\x03\x02\x02\x02\u{b2}\u{b4}\x07\x4d\x02\x02\
	\u{b3}\u{b5}\x07\x0c\x02\x02\u{b4}\u{b3}\x03\x02\x02\x02\u{b4}\u{b5}\x03\
	\x02\x02\x02\u{b5}\u{b6}\x03\x02\x02\x02\u{b6}\u{b9}\x07\x59\x02\x02\u{b7}\
	\u{b8}\x07\x12\x02\x02\u{b8}\u{ba}\x05\x0e\x08\x02\u{b9}\u{b7}\x03\x02\x02\
	\x02\u{b9}\u{ba}\x03\x02\x02\x02\u{ba}\u{bb}\x03\x02\x02\x02\u{bb}\u{c3}\
	\x07\x0d\x02\x02\u{bc}\u{bd}\x07\x4d\x02\x02\u{bd}\u{c0}\x07\x59\x02\x02\
	\u{be}\u{bf}\x07\x12\x02\x02\u{bf}\u{c1}\x05\x0e\x08\x02\u{c0}\u{be}\x03\
	\x02\x02\x02\u{c0}\u{c1}\x03\x02\x02\x02\u{c1}\u{c3}\x03\x02\x02\x02\u{c2}\
	\u{94}\x03\x02\x02\x02\u{c2}\u{9a}\x03\x02\x02\x02\u{c2}\u{a0}\x03\x02\x02\
	\x02\u{c2}\u{a2}\x03\x02\x02\x02\u{c2}\u{ac}\x03\x02\x02\x02\u{c2}\u{b2}\
	\x03\x02\x02\x02\u{c2}\u{bc}\x03\x02\x02\x02\u{c3}\x07\x03\x02\x02\x02\u{c4}\
	\u{cd}\x05\x04\x03\x02\u{c5}\u{cd}\x05\x0a\x06\x02\u{c6}\u{cd}\x05\x0c\x07\
	\x02\u{c7}\u{c8}\x07\x0c\x02\x02\u{c8}\u{c9}\x05\x32\x1a\x02\u{c9}\u{ca}\
	\x07\x0d\x02\x02\u{ca}\u{cd}\x03\x02\x02\x02\u{cb}\u{cd}\x05\x06\x04\x02\
	\u{cc}\u{c4}\x03\x02\x02\x02\u{cc}\u{c5}\x03\x02\x02\x02\u{cc}\u{c6}\x03\
	\x02\x02\x02\u{cc}\u{c7}\x03\x02\x02\x02\u{cc}\u{cb}\x03\x02\x02\x02\u{cd}\
	\x09\x03\x02\x02\x02\u{ce}\u{cf}\x09\x02\x02\x02\u{cf}\u{d0}\x05\u{84}\x43\
	\x02\u{d0}\x0b\x03\x02\x02\x02\u{d1}\u{d2}\x05\u{84}\x43\x02\u{d2}\u{d3}\
	\x09\x02\x02\x02\u{d3}\x0d\x03\x02\x02\x02\u{d4}\u{d9}\x05\x32\x1a\x02\u{d5}\
	\u{d6}\x07\x12\x02\x02\u{d6}\u{d8}\x05\x32\x1a\x02\u{d7}\u{d5}\x03\x02\x02\
	\x02\u{d8}\u{db}\x03\x02\x02\x02\u{d9}\u{d7}\x03\x02\x02\x02\u{d9}\u{da}\
	\x03\x02\x02\x02\u{da}\x0f\x03\x02\x02\x02\u{db}\u{d9}\x03\x02\x02\x02\u{dc}\
	\u{e1}\x05\x08\x05\x02\u{dd}\u{de}\x05\x12\x0a\x02\u{de}\u{df}\x05\x10\x09\
	\x02\u{df}\u{e1}\x03\x02\x02\x02\u{e0}\u{dc}\x03\x02\x02\x02\u{e0}\u{dd}\
	\x03\x02\x02\x02\u{e1}\x11\x03\x02\x02\x02\u{e2}\u{e3}\x09\x03\x02\x02\u{e3}\
	\x13\x03\x02\x02\x02\u{e4}\u{ea}\x05\x10\x09\x02\u{e5}\u{e6}\x05\x16\x0c\
	\x02\u{e6}\u{e7}\x05\x10\x09\x02\u{e7}\u{e9}\x03\x02\x02\x02\u{e8}\u{e5}\
	\x03\x02\x02\x02\u{e9}\u{ec}\x03\x02\x02\x02\u{ea}\u{e8}\x03\x02\x02\x02\
	\u{ea}\u{eb}\x03\x02\x02\x02\u{eb}\x15\x03\x02\x02\x02\u{ec}\u{ea}\x03\x02\
	\x02\x02\u{ed}\u{ee}\x09\x04\x02\x02\u{ee}\x17\x03\x02\x02\x02\u{ef}\u{f5}\
	\x05\x14\x0b\x02\u{f0}\u{f1}\x05\x1a\x0e\x02\u{f1}\u{f2}\x05\x14\x0b\x02\
	\u{f2}\u{f4}\x03\x02\x02\x02\u{f3}\u{f0}\x03\x02\x02\x02\u{f4}\u{f7}\x03\
	\x02\x02\x02\u{f5}\u{f3}\x03\x02\x02\x02\u{f5}\u{f6}\x03\x02\x02\x02\u{f6}\
	\x19\x03\x02\x02\x02\u{f7}\u{f5}\x03\x02\x02\x02\u{f8}\u{f9}\x09\x05\x02\
	\x02\u{f9}\x1b\x03\x02\x02\x02\u{fa}\u{100}\x05\x18\x0d\x02\u{fb}\u{fc}\
	\x05\x1e\x10\x02\u{fc}\u{fd}\x05\x18\x0d\x02\u{fd}\u{ff}\x03\x02\x02\x02\
	\u{fe}\u{fb}\x03\x02\x02\x02\u{ff}\u{102}\x03\x02\x02\x02\u{100}\u{fe}\x03\
	\x02\x02\x02\u{100}\u{101}\x03\x02\x02\x02\u{101}\x1d\x03\x02\x02\x02\u{102}\
	\u{100}\x03\x02\x02\x02\u{103}\u{104}\x09\x06\x02\x02\u{104}\x1f\x03\x02\
	\x02\x02\u{105}\u{10b}\x05\x1c\x0f\x02\u{106}\u{107}\x05\x22\x12\x02\u{107}\
	\u{108}\x05\x1c\x0f\x02\u{108}\u{10a}\x03\x02\x02\x02\u{109}\u{106}\x03\
	\x02\x02\x02\u{10a}\u{10d}\x03\x02\x02\x02\u{10b}\u{109}\x03\x02\x02\x02\
	\u{10b}\u{10c}\x03\x02\x02\x02\u{10c}\x21\x03\x02\x02\x02\u{10d}\u{10b}\
	\x03\x02\x02\x02\u{10e}\u{10f}\x09\x07\x02\x02\u{10f}\x23\x03\x02\x02\x02\
	\u{110}\u{116}\x05\x20\x11\x02\u{111}\u{112}\x05\x26\x14\x02\u{112}\u{113}\
	\x05\x20\x11\x02\u{113}\u{115}\x03\x02\x02\x02\u{114}\u{111}\x03\x02\x02\
	\x02\u{115}\u{118}\x03\x02\x02\x02\u{116}\u{114}\x03\x02\x02\x02\u{116}\
	\u{117}\x03\x02\x02\x02\u{117}\x25\x03\x02\x02\x02\u{118}\u{116}\x03\x02\
	\x02\x02\u{119}\u{11a}\x09\x08\x02\x02\u{11a}\x27\x03\x02\x02\x02\u{11b}\
	\u{120}\x05\x24\x13\x02\u{11c}\u{11d}\x07\x1f\x02\x02\u{11d}\u{11f}\x05\
	\x24\x13\x02\u{11e}\u{11c}\x03\x02\x02\x02\u{11f}\u{122}\x03\x02\x02\x02\
	\u{120}\u{11e}\x03\x02\x02\x02\u{120}\u{121}\x03\x02\x02\x02\u{121}\x29\
	\x03\x02\x02\x02\u{122}\u{120}\x03\x02\x02\x02\u{123}\u{128}\x05\x28\x15\
	\x02\u{124}\u{125}\x07\x03\x02\x02\u{125}\u{127}\x05\x28\x15\x02\u{126}\
	\u{124}\x03\x02\x02\x02\u{127}\u{12a}\x03\x02\x02\x02\u{128}\u{126}\x03\
	\x02\x02\x02\u{128}\u{129}\x03\x02\x02\x02\u{129}\x2b\x03\x02\x02\x02\u{12a}\
	\u{128}\x03\x02\x02\x02\u{12b}\u{130}\x05\x2a\x16\x02\u{12c}\u{12d}\x07\
	\x1d\x02\x02\u{12d}\u{12f}\x05\x2a\x16\x02\u{12e}\u{12c}\x03\x02\x02\x02\
	\u{12f}\u{132}\x03\x02\x02\x02\u{130}\u{12e}\x03\x02\x02\x02\u{130}\u{131}\
	\x03\x02\x02\x02\u{131}\x2d\x03\x02\x02\x02\u{132}\u{130}\x03\x02\x02\x02\
	\u{133}\u{138}\x05\x2c\x17\x02\u{134}\u{135}\x07\x20\x02\x02\u{135}\u{137}\
	\x05\x2c\x17\x02\u{136}\u{134}\x03\x02\x02\x02\u{137}\u{13a}\x03\x02\x02\
	\x02\u{138}\u{136}\x03\x02\x02\x02\u{138}\u{139}\x03\x02\x02\x02\u{139}\
	\x2f\x03\x02\x02\x02\u{13a}\u{138}\x03\x02\x02\x02\u{13b}\u{140}\x05\x2e\
	\x18\x02\u{13c}\u{13d}\x07\x1e\x02\x02\u{13d}\u{13f}\x05\x2e\x18\x02\u{13e}\
	\u{13c}\x03\x02\x02\x02\u{13f}\u{142}\x03\x02\x02\x02\u{140}\u{13e}\x03\
	\x02\x02\x02\u{140}\u{141}\x03\x02\x02\x02\u{141}\x31\x03\x02\x02\x02\u{142}\
	\u{140}\x03\x02\x02\x02\u{143}\u{149}\x05\x30\x19\x02\u{144}\u{145}\x07\
	\x19\x02\x02\u{145}\u{146}\x05\x32\x1a\x02\u{146}\u{147}\x07\x14\x02\x02\
	\u{147}\u{148}\x05\x32\x1a\x02\u{148}\u{14a}\x03\x02\x02\x02\u{149}\u{144}\
	\x03\x02\x02\x02\u{149}\u{14a}\x03\x02\x02\x02\u{14a}\x33\x03\x02\x02\x02\
	\u{14b}\u{14c}\x05\x36\x1c\x02\u{14c}\u{14d}\x05\x38\x1d\x02\u{14d}\u{14f}\
	\x03\x02\x02\x02\u{14e}\u{14b}\x03\x02\x02\x02\u{14f}\u{150}\x03\x02\x02\
	\x02\u{150}\u{14e}\x03\x02\x02\x02\u{150}\u{151}\x03\x02\x02\x02\u{151}\
	\u{152}\x03\x02\x02\x02\u{152}\u{153}\x05\x32\x1a\x02\u{153}\u{18d}\x03\
	\x02\x02\x02\u{154}\u{156}\x07\x41\x02\x02\u{155}\u{157}\x07\x0c\x02\x02\
	\u{156}\u{155}\x03\x02\x02\x02\u{156}\u{157}\x03\x02\x02\x02\u{157}\u{158}\
	\x03\x02\x02\x02\u{158}\u{159}\x05\x06\x04\x02\u{159}\u{15a}\x07\x12\x02\
	\x02\u{15a}\u{15c}\x05\x32\x1a\x02\u{15b}\u{15d}\x07\x0d\x02\x02\u{15c}\
	\u{15b}\x03\x02\x02\x02\u{15c}\u{15d}\x03\x02\x02\x02\u{15d}\u{18d}\x03\
	\x02\x02\x02\u{15e}\u{160}\x07\x41\x02\x02\u{15f}\u{161}\x07\x0c\x02\x02\
	\u{160}\u{15f}\x03\x02\x02\x02\u{160}\u{161}\x03\x02\x02\x02\u{161}\u{162}\
	\x03\x02\x02\x02\u{162}\u{163}\x05\x36\x1c\x02\u{163}\u{164}\x07\x12\x02\
	\x02\u{164}\u{166}\x05\x32\x1a\x02\u{165}\u{167}\x07\x0d\x02\x02\u{166}\
	\u{165}\x03\x02\x02\x02\u{166}\u{167}\x03\x02\x02\x02\u{167}\u{18d}\x03\
	\x02\x02\x02\u{168}\u{16a}\x07\x4f\x02\x02\u{169}\u{16b}\x07\x0c\x02\x02\
	\u{16a}\u{169}\x03\x02\x02\x02\u{16a}\u{16b}\x03\x02\x02\x02\u{16b}\u{16c}\
	\x03\x02\x02\x02\u{16c}\u{16d}\x05\x06\x04\x02\u{16d}\u{16e}\x07\x12\x02\
	\x02\u{16e}\u{170}\x05\x32\x1a\x02\u{16f}\u{171}\x07\x0d\x02\x02\u{170}\
	\u{16f}\x03\x02\x02\x02\u{170}\u{171}\x03\x02\x02\x02\u{171}\u{18d}\x03\
	\x02\x02\x02\u{172}\u{174}\x07\x4f\x02\x02\u{173}\u{175}\x07\x0c\x02\x02\
	\u{174}\u{173}\x03\x02\x02\x02\u{174}\u{175}\x03\x02\x02\x02\u{175}\u{176}\
	\x03\x02\x02\x02\u{176}\u{177}\x05\x36\x1c\x02\u{177}\u{178}\x07\x12\x02\
	\x02\u{178}\u{17b}\x05\x32\x1a\x02\u{179}\u{17a}\x07\x12\x02\x02\u{17a}\
	\u{17c}\x05\x0e\x08\x02\u{17b}\u{179}\x03\x02\x02\x02\u{17b}\u{17c}\x03\
	\x02\x02\x02\u{17c}\u{17e}\x03\x02\x02\x02\u{17d}\u{17f}\x07\x0d\x02\x02\
	\u{17e}\u{17d}\x03\x02\x02\x02\u{17e}\u{17f}\x03\x02\x02\x02\u{17f}\u{18d}\
	\x03\x02\x02\x02\u{180}\u{182}\x07\x50\x02\x02\u{181}\u{183}\x07\x0c\x02\
	\x02\u{182}\u{181}\x03\x02\x02\x02\u{182}\u{183}\x03\x02\x02\x02\u{183}\
	\u{184}\x03\x02\x02\x02\u{184}\u{185}\x05\x36\x1c\x02\u{185}\u{186}\x07\
	\x12\x02\x02\u{186}\u{187}\x05\x32\x1a\x02\u{187}\u{188}\x07\x12\x02\x02\
	\u{188}\u{18a}\x05\x0e\x08\x02\u{189}\u{18b}\x07\x0d\x02\x02\u{18a}\u{189}\
	\x03\x02\x02\x02\u{18a}\u{18b}\x03\x02\x02\x02\u{18b}\u{18d}\x03\x02\x02\
	\x02\u{18c}\u{14e}\x03\x02\x02\x02\u{18c}\u{154}\x03\x02\x02\x02\u{18c}\
	\u{15e}\x03\x02\x02\x02\u{18c}\u{168}\x03\x02\x02\x02\u{18c}\u{172}\x03\
	\x02\x02\x02\u{18c}\u{180}\x03\x02\x02\x02\u{18d}\x35\x03\x02\x02\x02\u{18e}\
	\u{18f}\x05\u{84}\x43\x02\u{18f}\x37\x03\x02\x02\x02\u{190}\u{191}\x09\x09\
	\x02\x02\u{191}\x39\x03\x02\x02\x02\u{192}\u{193}\x05\x32\x1a\x02\u{193}\
	\x3b\x03\x02\x02\x02\u{194}\u{19e}\x05\x46\x24\x02\u{195}\u{196}\x05\x3e\
	\x20\x02\u{196}\u{197}\x07\x15\x02\x02\u{197}\u{19e}\x03\x02\x02\x02\u{198}\
	\u{19e}\x05\x4c\x27\x02\u{199}\u{19e}\x05\x4e\x28\x02\u{19a}\u{19e}\x05\
	\x5a\x2e\x02\u{19b}\u{19e}\x05\x64\x33\x02\u{19c}\u{19e}\x05\x44\x23\x02\
	\u{19d}\u{194}\x03\x02\x02\x02\u{19d}\u{195}\x03\x02\x02\x02\u{19d}\u{198}\
	\x03\x02\x02\x02\u{19d}\u{199}\x03\x02\x02\x02\u{19d}\u{19a}\x03\x02\x02\
	\x02\u{19d}\u{19b}\x03\x02\x02\x02\u{19d}\u{19c}\x03\x02\x02\x02\u{19e}\
	\x3d\x03\x02\x02\x02\u{19f}\u{1a0}\x07\x53\x02\x02\u{1a0}\u{1a1}\x05\x40\
	\x21\x02\u{1a1}\u{1a2}\x07\x12\x02\x02\u{1a2}\u{1aa}\x05\x42\x22\x02\u{1a3}\
	\u{1a4}\x07\x12\x02\x02\u{1a4}\u{1a5}\x05\x40\x21\x02\u{1a5}\u{1a6}\x07\
	\x12\x02\x02\u{1a6}\u{1a7}\x05\x42\x22\x02\u{1a7}\u{1a9}\x03\x02\x02\x02\
	\u{1a8}\u{1a3}\x03\x02\x02\x02\u{1a9}\u{1ac}\x03\x02\x02\x02\u{1aa}\u{1a8}\
	\x03\x02\x02\x02\u{1aa}\u{1ab}\x03\x02\x02\x02\u{1ab}\u{1b2}\x03\x02\x02\
	\x02\u{1ac}\u{1aa}\x03\x02\x02\x02\u{1ad}\u{1b2}\x07\x54\x02\x02\u{1ae}\
	\u{1b2}\x07\x56\x02\x02\u{1af}\u{1b2}\x07\x55\x02\x02\u{1b0}\u{1b2}\x07\
	\x57\x02\x02\u{1b1}\u{19f}\x03\x02\x02\x02\u{1b1}\u{1ad}\x03\x02\x02\x02\
	\u{1b1}\u{1ae}\x03\x02\x02\x02\u{1b1}\u{1af}\x03\x02\x02\x02\u{1b1}\u{1b0}\
	\x03\x02\x02\x02\u{1b2}\x3f\x03\x02\x02\x02\u{1b3}\u{1b6}\x07\x5b\x02\x02\
	\u{1b4}\u{1b6}\x05\x32\x1a\x02\u{1b5}\u{1b3}\x03\x02\x02\x02\u{1b5}\u{1b4}\
	\x03\x02\x02\x02\u{1b6}\x41\x03\x02\x02\x02\u{1b7}\u{1b8}\x09\x0a\x02\x02\
	\u{1b8}\x43\x03\x02\x02\x02\u{1b9}\u{1ba}\x07\x5a\x02\x02\u{1ba}\x45\x03\
	\x02\x02\x02\u{1bb}\u{1bd}\x07\x0e\x02\x02\u{1bc}\u{1be}\x05\x48\x25\x02\
	\u{1bd}\u{1bc}\x03\x02\x02\x02\u{1bd}\u{1be}\x03\x02\x02\x02\u{1be}\u{1bf}\
	\x03\x02\x02\x02\u{1bf}\u{1c0}\x07\x0f\x02\x02\u{1c0}\x47\x03\x02\x02\x02\
	\u{1c1}\u{1c3}\x05\x4a\x26\x02\u{1c2}\u{1c1}\x03\x02\x02\x02\u{1c3}\u{1c4}\
	\x03\x02\x02\x02\u{1c4}\u{1c2}\x03\x02\x02\x02\u{1c4}\u{1c5}\x03\x02\x02\
	\x02\u{1c5}\x49\x03\x02\x02\x02\u{1c6}\u{1c9}\x05\x3c\x1f\x02\u{1c7}\u{1c9}\
	\x05\x6a\x36\x02\u{1c8}\u{1c6}\x03\x02\x02\x02\u{1c8}\u{1c7}\x03\x02\x02\
	\x02\u{1c9}\x4b\x03\x02\x02\x02\u{1ca}\u{1cb}\x05\x34\x1b\x02\u{1cb}\u{1cc}\
	\x07\x15\x02\x02\u{1cc}\u{1d1}\x03\x02\x02\x02\u{1cd}\u{1ce}\x05\x32\x1a\
	\x02\u{1ce}\u{1cf}\x07\x15\x02\x02\u{1cf}\u{1d1}\x03\x02\x02\x02\u{1d0}\
	\u{1ca}\x03\x02\x02\x02\u{1d0}\u{1cd}\x03\x02\x02\x02\u{1d1}\x4d\x03\x02\
	\x02\x02\u{1d2}\u{1d3}\x07\x3e\x02\x02\u{1d3}\u{1d4}\x07\x0c\x02\x02\u{1d4}\
	\u{1d5}\x05\x32\x1a\x02\u{1d5}\u{1d6}\x07\x0d\x02\x02\u{1d6}\u{1d9}\x05\
	\x3c\x1f\x02\u{1d7}\u{1d8}\x07\x3f\x02\x02\u{1d8}\u{1da}\x05\x3c\x1f\x02\
	\u{1d9}\u{1d7}\x03\x02\x02\x02\u{1d9}\u{1da}\x03\x02\x02\x02\u{1da}\u{1dd}\
	\x03\x02\x02\x02\u{1db}\u{1dd}\x05\x50\x29\x02\u{1dc}\u{1d2}\x03\x02\x02\
	\x02\u{1dc}\u{1db}\x03\x02\x02\x02\u{1dd}\x4f\x03\x02\x02\x02\u{1de}\u{1df}\
	\x07\x47\x02\x02\u{1df}\u{1e0}\x07\x0c\x02\x02\u{1e0}\u{1e1}\x05\x32\x1a\
	\x02\u{1e1}\u{1e2}\x07\x0d\x02\x02\u{1e2}\u{1e3}\x05\x52\x2a\x02\u{1e3}\
	\x51\x03\x02\x02\x02\u{1e4}\u{1e8}\x07\x0e\x02\x02\u{1e5}\u{1e7}\x05\x54\
	\x2b\x02\u{1e6}\u{1e5}\x03\x02\x02\x02\u{1e7}\u{1ea}\x03\x02\x02\x02\u{1e8}\
	\u{1e6}\x03\x02\x02\x02\u{1e8}\u{1e9}\x03\x02\x02\x02\u{1e9}\u{1eb}\x03\
	\x02\x02\x02\u{1ea}\u{1e8}\x03\x02\x02\x02\u{1eb}\u{1ec}\x07\x0f\x02\x02\
	\u{1ec}\x53\x03\x02\x02\x02\u{1ed}\u{1ee}\x05\x56\x2c\x02\u{1ee}\u{1ef}\
	\x05\x48\x25\x02\u{1ef}\x55\x03\x02\x02\x02\u{1f0}\u{1f4}\x05\x58\x2d\x02\
	\u{1f1}\u{1f3}\x05\x58\x2d\x02\u{1f2}\u{1f1}\x03\x02\x02\x02\u{1f3}\u{1f6}\
	\x03\x02\x02\x02\u{1f4}\u{1f2}\x03\x02\x02\x02\u{1f4}\u{1f5}\x03\x02\x02\
	\x02\u{1f5}\x57\x03\x02\x02\x02\u{1f6}\u{1f4}\x03\x02\x02\x02\u{1f7}\u{1fc}\
	\x07\x48\x02\x02\u{1f8}\u{1fd}\x07\x5a\x02\x02\u{1f9}\u{1fa}\x05\x3a\x1e\
	\x02\u{1fa}\u{1fb}\x07\x14\x02\x02\u{1fb}\u{1fd}\x03\x02\x02\x02\u{1fc}\
	\u{1f8}\x03\x02\x02\x02\u{1fc}\u{1f9}\x03\x02\x02\x02\u{1fd}\u{200}\x03\
	\x02\x02\x02\u{1fe}\u{200}\x07\x49\x02\x02\u{1ff}\u{1f7}\x03\x02\x02\x02\
	\u{1ff}\u{1fe}\x03\x02\x02\x02\u{200}\x59\x03\x02\x02\x02\u{201}\u{202}\
	\x07\x43\x02\x02\u{202}\u{203}\x07\x0c\x02\x02\u{203}\u{204}\x05\x32\x1a\
	\x02\u{204}\u{205}\x07\x0d\x02\x02\u{205}\u{206}\x05\x3c\x1f\x02\u{206}\
	\u{216}\x03\x02\x02\x02\u{207}\u{208}\x07\x44\x02\x02\u{208}\u{209}\x05\
	\x3c\x1f\x02\u{209}\u{20a}\x07\x43\x02\x02\u{20a}\u{20b}\x07\x0c\x02\x02\
	\u{20b}\u{20c}\x05\x32\x1a\x02\u{20c}\u{20d}\x07\x0d\x02\x02\u{20d}\u{20e}\
	\x07\x15\x02\x02\u{20e}\u{216}\x03\x02\x02\x02\u{20f}\u{210}\x07\x42\x02\
	\x02\u{210}\u{211}\x07\x0c\x02\x02\u{211}\u{212}\x05\x5c\x2f\x02\u{212}\
	\u{213}\x07\x0d\x02\x02\u{213}\u{214}\x05\x3c\x1f\x02\u{214}\u{216}\x03\
	\x02\x02\x02\u{215}\u{201}\x03\x02\x02\x02\u{215}\u{207}\x03\x02\x02\x02\
	\u{215}\u{20f}\x03\x02\x02\x02\u{216}\x5b\x03\x02\x02\x02\u{217}\u{218}\
	\x05\x5e\x30\x02\u{218}\u{21a}\x07\x15\x02\x02\u{219}\u{21b}\x05\x60\x31\
	\x02\u{21a}\u{219}\x03\x02\x02\x02\u{21a}\u{21b}\x03\x02\x02\x02\u{21b}\
	\u{21c}\x03\x02\x02\x02\u{21c}\u{21e}\x07\x15\x02\x02\u{21d}\u{21f}\x05\
	\x62\x32\x02\u{21e}\u{21d}\x03\x02\x02\x02\u{21e}\u{21f}\x03\x02\x02\x02\
	\u{21f}\x5d\x03\x02\x02\x02\u{220}\u{222}\x05\x34\x1b\x02\u{221}\u{220}\
	\x03\x02\x02\x02\u{221}\u{222}\x03\x02\x02\x02\u{222}\x5f\x03\x02\x02\x02\
	\u{223}\u{226}\x05\x34\x1b\x02\u{224}\u{226}\x05\x32\x1a\x02\u{225}\u{223}\
	\x03\x02\x02\x02\u{225}\u{224}\x03\x02\x02\x02\u{226}\x61\x03\x02\x02\x02\
	\u{227}\u{22a}\x05\x34\x1b\x02\u{228}\u{22a}\x05\x32\x1a\x02\u{229}\u{227}\
	\x03\x02\x02\x02\u{229}\u{228}\x03\x02\x02\x02\u{22a}\x63\x03\x02\x02\x02\
	\u{22b}\u{22c}\x07\x45\x02\x02\u{22c}\u{233}\x07\x59\x02\x02\u{22d}\u{233}\
	\x09\x0b\x02\x02\u{22e}\u{230}\x07\x46\x02\x02\u{22f}\u{231}\x05\x32\x1a\
	\x02\u{230}\u{22f}\x03\x02\x02\x02\u{230}\u{231}\x03\x02\x02\x02\u{231}\
	\u{233}\x03\x02\x02\x02\u{232}\u{22b}\x03\x02\x02\x02\u{232}\u{22d}\x03\
	\x02\x02\x02\u{232}\u{22e}\x03\x02\x02\x02\u{233}\u{234}\x03\x02\x02\x02\
	\u{234}\u{235}\x07\x15\x02\x02\u{235}\x65\x03\x02\x02\x02\u{236}\u{238}\
	\x05\x68\x35\x02\u{237}\u{236}\x03\x02\x02\x02\u{238}\u{239}\x03\x02\x02\
	\x02\u{239}\u{237}\x03\x02\x02\x02\u{239}\u{23a}\x03\x02\x02\x02\u{23a}\
	\x67\x03\x02\x02\x02\u{23b}\u{240}\x05\x6a\x36\x02\u{23c}\u{240}\x05\x6c\
	\x37\x02\u{23d}\u{240}\x05\x78\x3d\x02\u{23e}\u{240}\x07\x15\x02\x02\u{23f}\
	\u{23b}\x03\x02\x02\x02\u{23f}\u{23c}\x03\x02\x02\x02\u{23f}\u{23d}\x03\
	\x02\x02\x02\u{23f}\u{23e}\x03\x02\x02\x02\u{240}\x69\x03\x02\x02\x02\u{241}\
	\u{242}\x07\x4a\x02\x02\u{242}\u{243}\x07\x58\x02\x02\u{243}\u{244}\x07\
	\x59\x02\x02\u{244}\u{24c}\x05\x46\x24\x02\u{245}\u{246}\x07\x4a\x02\x02\
	\u{246}\u{247}\x07\x59\x02\x02\u{247}\u{24c}\x05\x46\x24\x02\u{248}\u{249}\
	\x07\x4a\x02\x02\u{249}\u{24a}\x07\x59\x02\x02\u{24a}\u{24c}\x07\x15\x02\
	\x02\u{24b}\u{241}\x03\x02\x02\x02\u{24b}\u{245}\x03\x02\x02\x02\u{24b}\
	\u{248}\x03\x02\x02\x02\u{24c}\x6b\x03\x02\x02\x02\u{24d}\u{24e}\x07\x27\
	\x02\x02\u{24e}\u{24f}\x07\x58\x02\x02\u{24f}\u{250}\x05\x7a\x3e\x02\u{250}\
	\u{255}\x05\x76\x3c\x02\u{251}\u{252}\x07\x12\x02\x02\u{252}\u{254}\x05\
	\x76\x3c\x02\u{253}\u{251}\x03\x02\x02\x02\u{254}\u{257}\x03\x02\x02\x02\
	\u{255}\u{253}\x03\x02\x02\x02\u{255}\u{256}\x03\x02\x02\x02\u{256}\u{258}\
	\x03\x02\x02\x02\u{257}\u{255}\x03\x02\x02\x02\u{258}\u{259}\x07\x12\x02\
	\x02\u{259}\u{25a}\x05\x46\x24\x02\u{25a}\u{270}\x03\x02\x02\x02\u{25b}\
	\u{25c}\x05\x6e\x38\x02\u{25c}\u{25d}\x07\x12\x02\x02\u{25d}\u{25e}\x05\
	\x70\x39\x02\u{25e}\u{25f}\x07\x12\x02\x02\u{25f}\u{260}\x05\x72\x3a\x02\
	\u{260}\u{261}\x07\x12\x02\x02\u{261}\u{262}\x05\x74\x3b\x02\u{262}\u{263}\
	\x07\x58\x02\x02\u{263}\u{264}\x05\x7a\x3e\x02\u{264}\u{269}\x05\x76\x3c\
	\x02\u{265}\u{266}\x07\x12\x02\x02\u{266}\u{268}\x05\x76\x3c\x02\u{267}\
	\u{265}\x03\x02\x02\x02\u{268}\u{26b}\x03\x02\x02\x02\u{269}\u{267}\x03\
	\x02\x02\x02\u{269}\u{26a}\x03\x02\x02\x02\u{26a}\u{26c}\x03\x02\x02\x02\
	\u{26b}\u{269}\x03\x02\x02\x02\u{26c}\u{26d}\x07\x12\x02\x02\u{26d}\u{26e}\
	\x05\x46\x24\x02\u{26e}\u{270}\x03\x02\x02\x02\u{26f}\u{24d}\x03\x02\x02\
	\x02\u{26f}\u{25b}\x03\x02\x02\x02\u{270}\x6d\x03\x02\x02\x02\u{271}\u{272}\
	\x07\x59\x02\x02\u{272}\x6f\x03\x02\x02\x02\u{273}\u{274}\x07\x5c\x02\x02\
	\u{274}\x71\x03\x02\x02\x02\u{275}\u{276}\x07\x5c\x02\x02\u{276}\x73\x03\
	\x02\x02\x02\u{277}\u{278}\x07\x5c\x02\x02\u{278}\x75\x03\x02\x02\x02\u{279}\
	\u{27b}\x07\x27\x02\x02\u{27a}\u{279}\x03\x02\x02\x02\u{27a}\u{27b}\x03\
	\x02\x02\x02\u{27b}\u{27c}\x03\x02\x02\x02\u{27c}\u{27f}\x07\x5c\x02\x02\
	\u{27d}\u{27f}\x07\x59\x02\x02\u{27e}\u{27a}\x03\x02\x02\x02\u{27e}\u{27d}\
	\x03\x02\x02\x02\u{27f}\x77\x03\x02\x02\x02\u{280}\u{285}\x07\x59\x02\x02\
	\u{281}\u{283}\x07\x12\x02\x02\u{282}\u{284}\x07\x5c\x02\x02\u{283}\u{282}\
	\x03\x02\x02\x02\u{283}\u{284}\x03\x02\x02\x02\u{284}\u{286}\x03\x02\x02\
	\x02\u{285}\u{281}\x03\x02\x02\x02\u{286}\u{287}\x03\x02\x02\x02\u{287}\
	\u{285}\x03\x02\x02\x02\u{287}\u{288}\x03\x02\x02\x02\u{288}\u{289}\x03\
	\x02\x02\x02\u{289}\u{28a}\x07\x59\x02\x02\u{28a}\u{28b}\x05\x7a\x3e\x02\
	\u{28b}\u{290}\x09\x0c\x02\x02\u{28c}\u{28d}\x07\x12\x02\x02\u{28d}\u{28f}\
	\x09\x0c\x02\x02\u{28e}\u{28c}\x03\x02\x02\x02\u{28f}\u{292}\x03\x02\x02\
	\x02\u{290}\u{28e}\x03\x02\x02\x02\u{290}\u{291}\x03\x02\x02\x02\u{291}\
	\u{2c6}\x03\x02\x02\x02\u{292}\u{290}\x03\x02\x02\x02\u{293}\u{294}\x05\
	\x6e\x38\x02\u{294}\u{295}\x07\x12\x02\x02\u{295}\u{296}\x05\x70\x39\x02\
	\u{296}\u{297}\x07\x12\x02\x02\u{297}\u{298}\x05\x72\x3a\x02\u{298}\u{299}\
	\x07\x12\x02\x02\u{299}\u{29a}\x05\x74\x3b\x02\u{29a}\u{29b}\x07\x0a\x02\
	\x02\u{29b}\u{29c}\x07\x0c\x02\x02\u{29c}\u{29d}\x05\x7a\x3e\x02\u{29d}\
	\u{29e}\x07\x0d\x02\x02\u{29e}\u{29f}\x05\x7a\x3e\x02\u{29f}\u{2a4}\x05\
	\x76\x3c\x02\u{2a0}\u{2a1}\x07\x12\x02\x02\u{2a1}\u{2a3}\x05\x76\x3c\x02\
	\u{2a2}\u{2a0}\x03\x02\x02\x02\u{2a3}\u{2a6}\x03\x02\x02\x02\u{2a4}\u{2a2}\
	\x03\x02\x02\x02\u{2a4}\u{2a5}\x03\x02\x02\x02\u{2a5}\u{2c6}\x03\x02\x02\
	\x02\u{2a6}\u{2a4}\x03\x02\x02\x02\u{2a7}\u{2a8}\x05\x6e\x38\x02\u{2a8}\
	\u{2a9}\x07\x12\x02\x02\u{2a9}\u{2aa}\x05\x70\x39\x02\u{2aa}\u{2ab}\x07\
	\x12\x02\x02\u{2ab}\u{2ac}\x05\x72\x3a\x02\u{2ac}\u{2ad}\x07\x12\x02\x02\
	\u{2ad}\u{2ae}\x05\x74\x3b\x02\u{2ae}\u{2af}\x07\x0b\x02\x02\u{2af}\u{2b0}\
	\x05\x7a\x3e\x02\u{2b0}\u{2b1}\x05\x76\x3c\x02\u{2b1}\u{2b5}\x07\x12\x02\
	\x02\u{2b2}\u{2b3}\x05\x7c\x3f\x02\u{2b3}\u{2b4}\x07\x12\x02\x02\u{2b4}\
	\u{2b6}\x03\x02\x02\x02\u{2b5}\u{2b2}\x03\x02\x02\x02\u{2b5}\u{2b6}\x03\
	\x02\x02\x02\u{2b6}\u{2b7}\x03\x02\x02\x02\u{2b7}\u{2b8}\x05\x7e\x40\x02\
	\u{2b8}\u{2b9}\x07\x14\x02\x02\u{2b9}\u{2ba}\x05\u{80}\x41\x02\u{2ba}\u{2c2}\
	\x03\x02\x02\x02\u{2bb}\u{2bc}\x07\x12\x02\x02\u{2bc}\u{2bd}\x05\x7e\x40\
	\x02\u{2bd}\u{2be}\x07\x14\x02\x02\u{2be}\u{2bf}\x05\u{80}\x41\x02\u{2bf}\
	\u{2c1}\x03\x02\x02\x02\u{2c0}\u{2bb}\x03\x02\x02\x02\u{2c1}\u{2c4}\x03\
	\x02\x02\x02\u{2c2}\u{2c0}\x03\x02\x02\x02\u{2c2}\u{2c3}\x03\x02\x02\x02\
	\u{2c3}\u{2c6}\x03\x02\x02\x02\u{2c4}\u{2c2}\x03\x02\x02\x02\u{2c5}\u{280}\
	\x03\x02\x02\x02\u{2c5}\u{293}\x03\x02\x02\x02\u{2c5}\u{2a7}\x03\x02\x02\
	\x02\u{2c6}\x79\x03\x02\x02\x02\u{2c7}\u{2c9}\x09\x0d\x02\x02\u{2c8}\u{2c7}\
	\x03\x02\x02\x02\u{2c9}\u{2cc}\x03\x02\x02\x02\u{2ca}\u{2c8}\x03\x02\x02\
	\x02\u{2ca}\u{2cb}\x03\x02\x02\x02\u{2cb}\x7b\x03\x02\x02\x02\u{2cc}\u{2ca}\
	\x03\x02\x02\x02\u{2cd}\u{2ce}\x07\x5c\x02\x02\u{2ce}\x7d\x03\x02\x02\x02\
	\u{2cf}\u{2d0}\x07\x5c\x02\x02\u{2d0}\x7f\x03\x02\x02\x02\u{2d1}\u{2d3}\
	\x07\x27\x02\x02\u{2d2}\u{2d1}\x03\x02\x02\x02\u{2d2}\u{2d3}\x03\x02\x02\
	\x02\u{2d3}\u{2d4}\x03\x02\x02\x02\u{2d4}\u{2d5}\x07\x5c\x02\x02\u{2d5}\
	\u{81}\x03\x02\x02\x02\u{2d6}\u{2d7}\x09\x0e\x02\x02\u{2d7}\u{83}\x03\x02\
	\x02\x02\u{2d8}\u{2da}\x05\u{82}\x42\x02\u{2d9}\u{2d8}\x03\x02\x02\x02\u{2d9}\
	\u{2da}\x03\x02\x02\x02\u{2da}\u{2db}\x03\x02\x02\x02\u{2db}\u{2dd}\x05\
	\u{86}\x44\x02\u{2dc}\u{2de}\x07\x2d\x02\x02\u{2dd}\u{2dc}\x03\x02\x02\x02\
	\u{2dd}\u{2de}\x03\x02\x02\x02\u{2de}\u{2e3}\x03\x02\x02\x02\u{2df}\u{2e0}\
	\x07\x10\x02\x02\u{2e0}\u{2e1}\x05\x32\x1a\x02\u{2e1}\u{2e2}\x07\x11\x02\
	\x02\u{2e2}\u{2e4}\x03\x02\x02\x02\u{2e3}\u{2df}\x03\x02\x02\x02\u{2e3}\
	\u{2e4}\x03\x02\x02\x02\u{2e4}\u{85}\x03\x02\x02\x02\u{2e5}\u{2e6}\x09\x0f\
	\x02\x02\u{2e6}\u{87}\x03\x02\x02\x02\x51\u{92}\u{97}\u{9d}\u{a4}\u{a9}\
	\u{b0}\u{b4}\u{b9}\u{c0}\u{c2}\u{cc}\u{d9}\u{e0}\u{ea}\u{f5}\u{100}\u{10b}\
	\u{116}\u{120}\u{128}\u{130}\u{138}\u{140}\u{149}\u{150}\u{156}\u{15c}\u{160}\
	\u{166}\u{16a}\u{170}\u{174}\u{17b}\u{17e}\u{182}\u{18a}\u{18c}\u{19d}\u{1aa}\
	\u{1b1}\u{1b5}\u{1bd}\u{1c4}\u{1c8}\u{1d0}\u{1d9}\u{1dc}\u{1e8}\u{1f4}\u{1fc}\
	\u{1ff}\u{215}\u{21a}\u{21e}\u{221}\u{225}\u{229}\u{230}\u{232}\u{239}\u{23f}\
	\u{24b}\u{255}\u{269}\u{26f}\u{27a}\u{27e}\u{283}\u{287}\u{290}\u{2a4}\u{2b5}\
	\u{2c2}\u{2c5}\u{2ca}\u{2d2}\u{2d9}\u{2dd}\u{2e3}";

