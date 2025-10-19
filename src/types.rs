use boa_engine::Context;
use std::path::PathBuf;

use crate::{
    console::error_skid,
    macros::{simple_blocks::macro_comment, template::SkidTemplate},
    project::ProjectContext,
};

pub struct Token {
    pub contents: String,
    pub origin_file: usize,
    pub template_origin: usize,
    pub line_number: usize,
    pub section_name_index: usize,
}

pub struct InputFile {
    pub file_input: PathBuf,
    pub file_skidout: PathBuf,
    pub file_out: PathBuf,
    pub tokens: Vec<Token>,
}

pub struct SkidContext {
    pub templates: Vec<SkidTemplate>,
}

impl SkidContext {
    pub fn new() -> SkidContext {
        SkidContext {
            templates: Vec::new(),
        }
    }
}

type MacroExpansion =
    fn(usize, usize, &mut ProjectContext, &mut SkidContext, &Vec<String>, &[Token]) -> Vec<Token>;
// (
//     origin_index: usize,
//     origin_line: usize,
//     context: &mut ProjectContext,
//     templates: &mut Vec<SkidTemplate>,
//     args: &Vec<String>,
//     scope: &[Token],
// ) -> Vec<Token>

pub struct Macro {
    pub symbol: &'static str,
    pub expansion: MacroExpansion,
    pub has_scope: bool, //takes blocks of text input as well as parameters using [[{}]]
    pub min_args: usize,
    pub max_args: usize,
}

pub trait Expand {
    fn expand(
        &self,
        origin_index: usize,
        origin_line: usize,
        context: &mut ProjectContext,
        skid_context: &mut SkidContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token>;

    fn default() -> Macro;
}

impl Expand for Macro {
    fn expand(
        &self,
        origin_index: usize,
        origin_line: usize,
        context: &mut ProjectContext,
        skid_context: &mut SkidContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token> {
        if (args.len() > self.max_args) || (args.len() < self.min_args) {
            error_skid(context, origin_index, origin_line, &format!("Macro \'{}\' was given a number of arguments ({}) not in its acceptable range ({}-{})",
        self.symbol, args.len(), self.min_args, if self.max_args == usize::max_value() {"No Limit".to_string()} else {format!("{}", self.max_args)}));
            Vec::new()
        } else {
            (self.expansion)(
                origin_index,
                origin_line,
                context,
                skid_context,
                args,
                scope,
            )
        }
    }

    fn default() -> Macro {
        Macro {
            symbol: "default_symbol",
            expansion: macro_comment,
            has_scope: true,
            min_args: 0,
            max_args: usize::max_value(),
        }
    }
}

impl InputFile {
    pub fn new() -> InputFile {
        InputFile {
            file_input: "".into(),
            file_skidout: "".into(),
            file_out: "".into(),
            tokens: Vec::new(),
        }
    }
}

impl Token {
    pub fn new(contents: String, origin_file: usize, line_number: usize) -> Token {
        Token {
            contents: contents,
            origin_file: origin_file,
            template_origin: origin_file,
            line_number: line_number,
            section_name_index: 0,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return self.contents.clone();
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        let mut t = Token::new(
            self.contents.clone(),
            self.origin_file.clone(),
            self.line_number,
        );
        t.template_origin = self.template_origin;
        return t;
    }
}
