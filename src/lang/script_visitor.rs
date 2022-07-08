use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::ByteStream;
use antlr_rust::InputStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor};
use crate::lang::compiler::parse_number;
use crate::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use crate::parser::rathenascriptlangparser::{RathenaScriptLangParser, RathenaScriptLangParserContextType, ScriptDirContextAttrs, ScriptInitializationContext, ScriptInitializationContextAttrs, ScriptSpriteContextAttrs, ScriptXPosContextAttrs, ScriptYPosContextAttrs};
use crate::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;

pub struct Script {
    pub name: String,
    pub x_pos: usize,
    pub y_pos: usize,
    pub dir: usize,
    pub map: String,
    pub sprite: isize,
    pub x_size: usize,
    pub y_size: usize,
}
pub struct ScriptVisitor {
    scripts: Vec<Script>,
}

pub fn visit(file: &File) -> Vec<Script> {
    let mut reader = BufReader::new(file);
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content).unwrap();
    let lexer = RathenaScriptLangLexer::new(InputStream::new(file_content.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = RathenaScriptLangParser::new(token_stream);
    let tree = parser.compilationUnit();
    let mut script_visitor = ScriptVisitor{ scripts: vec![] };
    script_visitor.visit_compilationUnit(tree.as_ref().unwrap());
    mem::take(&mut script_visitor.scripts)
}

impl<'input> ParseTreeVisitor<'input, RathenaScriptLangParserContextType> for ScriptVisitor {}

impl<'input> RathenaScriptLangVisitor<'input> for ScriptVisitor {
    fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) {
        if ctx.scriptLocation().is_some() {
            let sprite = parse_number(ctx.scriptSprite_all().get(0).unwrap().Number().unwrap().symbol.text.clone());
            self.scripts.push(Script {
                name: ctx.scriptName().unwrap().get_text(),
                x_pos: parse_number(ctx.scriptXPos().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                y_pos: parse_number(ctx.scriptYPos().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                dir: parse_number(ctx.scriptDir().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                map: ctx.scriptLocation().unwrap().get_text(),
                sprite: sprite as isize,
                x_size: 0,
                y_size: 0
            });
        }
    }
}