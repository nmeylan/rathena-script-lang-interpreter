use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;
use std::path::Path;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Tree};

use crate::lang::chunk::ClassFile;
use crate::lang::compiler::{Compiler, parse_number};
use crate::lang::error::CompilationError;
use crate::parser::rathenascriptlanglexer::RathenaScriptLangLexer;
use crate::parser::rathenascriptlangparser::{NpcInitializationContext, NpcInitializationContextAttrs, RathenaScriptLangParser, RathenaScriptLangParserContextType, ScriptDirContextAttrs, ScriptInitializationContext, ScriptInitializationContextAttrs, ScriptSpriteContextAttrs, ScriptXPosContextAttrs, ScriptYPosContextAttrs};
use crate::parser::rathenascriptlangvisitor::RathenaScriptLangVisitor;

pub struct Script {
    pub name: String,
    pub x_pos: usize,
    pub y_pos: usize,
    pub dir: usize,
    pub map: String,
    pub sprite: String,
    pub x_size: usize,
    pub y_size: usize,
    pub class_name: String,
    pub class_reference: u64,
}

pub struct ScriptVisitor {
    scripts: Vec<Script>,
    pub file_content: Vec<String>,
}

pub fn compile(paths: Vec<String>, native_function_list_file_path: &str, debug_flag: u64) -> (Vec<Script>, Vec<ClassFile>, HashMap<String, Vec<CompilationError>>) {
    let mut compiler = Compiler::new(Default::default(), Default::default(), native_function_list_file_path, debug_flag);
    let mut scripts = Vec::<Script>::new();
    for path in paths.iter() {
        let path = Path::new(path);
        scripts.extend(visit(path));
        compiler.compile_file_and_keep_state(path);
    }
    compiler.file_name = String::from("server_internal");
    compiler.compile_content_and_keep_state(r#"- script ErrorScript -1,{ mes "^FF0000 This NPC has a compilation error.", "Check server logs for error details.^000000"; close;}"#.to_string());
    let (mut class_files, errors) = compiler.end_compilation();
    let classes_in_error = errors.iter().map(|(k, _)| k.clone()).collect::<Vec<String>>();
    classes_in_error.iter().for_each(|k| println!("in error -> {}", k));
    class_files.iter().for_each(|k| println!("class file -> {}", k.name));
    class_files = class_files.drain(..).filter(|class| !classes_in_error.contains(&class.name)).collect::<Vec<ClassFile>>();
    class_files.iter().for_each(|k| println!("class file after filter -> {}", k.name));
    class_files.iter_mut().for_each(|class_file| class_file.set_reference());
    let mut class_references = HashMap::<String, u64>::new();
    class_files.iter().for_each(|class| { class_references.insert(class.name.clone(), class.reference); });
    scripts.iter_mut().for_each(|script| {
        if class_references.get(&script.class_name).is_some() {
            script.class_reference = *class_references.get(&script.class_name).unwrap();
        } else {
            script.class_name = String::from("ErrorScript");
            script.class_reference = *class_references.get(&String::from("ErrorScript")).unwrap();
        }
    });
    (scripts, class_files, errors)
}

pub fn visit(path: &Path) -> Vec<Script> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content).unwrap();
    let lexer = RathenaScriptLangLexer::new(InputStream::new(file_content.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = RathenaScriptLangParser::new(token_stream);
    let tree = parser.compilationUnit();
    let mut script_visitor = ScriptVisitor { scripts: vec![], file_content: file_content.split('\n').map(|l| l.to_string()).collect::<Vec<String>>() };
    script_visitor.visit_compilationUnit(tree.as_ref().unwrap());
    mem::take(&mut script_visitor.scripts)
}

impl<'input> ParseTreeVisitor<'input, RathenaScriptLangParserContextType> for ScriptVisitor {}

impl<'input> RathenaScriptLangVisitor<'input> for ScriptVisitor {
    fn visit_scriptInitialization(&mut self, ctx: &ScriptInitializationContext<'input>) {
        if ctx.scriptLocation().is_some() {
            let script_declaration = self.file_content[ctx.start().line as usize - 1].clone();
            let script_name = script_declaration.split('\t').collect::<Vec<&str>>()[2].to_string();
            let sprite = if ctx.scriptSprite_all().get(0).unwrap().Number().is_some() {
                ctx.scriptSprite_all().get(0).unwrap().Number().unwrap().symbol.text.to_string()
            } else {
                ctx.scriptSprite_all().get(0).unwrap().Identifier().unwrap().symbol.text.to_string()
            };
            self.scripts.push(Script {
                name: script_name,
                x_pos: parse_number(ctx.scriptXPos().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                y_pos: parse_number(ctx.scriptYPos().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                dir: parse_number(ctx.scriptDir().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                map: ctx.scriptLocation().unwrap().get_text(),
                sprite,
                x_size: 0,
                y_size: 0,
                class_name: ctx.scriptName().unwrap().get_text(),
                class_reference: 0,
            });
        }
    }
    fn visit_npcInitialization(&mut self, ctx: &NpcInitializationContext<'input>) {
        if ctx.scriptLocation().is_some() {
            let script_declaration = self.file_content[ctx.start().line as usize - 1].clone();
            let script_name = script_declaration.split('\t').collect::<Vec<&str>>()[2].to_string();
            let sprite = ctx.scriptSprite_all().get(0).unwrap().get_child(0).as_ref().unwrap().get_text();
            self.scripts.push(Script {
                name: script_name,
                x_pos: parse_number(ctx.scriptXPos().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                y_pos: parse_number(ctx.scriptYPos().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                dir: parse_number(ctx.scriptDir().unwrap().Number().unwrap().symbol.text.clone()) as usize,
                map: ctx.scriptLocation().unwrap().get_text(),
                sprite,
                x_size: 0,
                y_size: 0,
                class_name: ctx.scriptName_all().get(0).unwrap().get_text(),
                class_reference: 0,
            });
        }
    }
}