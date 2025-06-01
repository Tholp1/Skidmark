use std::path::PathBuf;

use crate::{
    console::error_skid,
    macros::{simple_blocks::macro_comment, template::SkidTemplate},
    projectparse::ProjectContext,
};

pub struct Token {
    pub contents: String,
    pub origin_file: usize,
    pub template_origin: usize,
    pub line_number: usize,
}

pub struct InputFile {
    pub file_input: PathBuf,
    pub file_skidout: PathBuf,
    pub file_out: PathBuf,
    pub tokens: Vec<Token>,
    pub working_index: usize,
    pub templates: Vec<SkidTemplate>,
}

type MacroExpansion =
    fn(&mut InputFile, usize, usize, &mut ProjectContext, &Vec<String>, &[Token]) -> Vec<Token>;
// (
//     _file: &mut InputFile,
//     origin_index: usize,
//     origin_line: usize,
//     context: &mut ProjectContext,
//     args: &Vec<String>,
//     _scope: &[Token],
// ) -> Vec<Token>

pub struct Macro {
    pub symbol: &'static str,
    pub expansion: MacroExpansion,
    pub has_scope: bool, //takes blocks of text input as well as parameters using {{...}}
    pub min_args: usize,
    pub max_args: usize,
}

pub trait Expand {
    fn expand(
        &self,
        input_file: &mut InputFile,
        origin_index: usize,
        origin_line: usize,
        context: &mut ProjectContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token>;

    fn default() -> Macro;
}

impl Expand for Macro {
    fn expand(
        &self,
        input_file: &mut InputFile,
        origin_index: usize,
        origin_line: usize,
        context: &mut ProjectContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token> {
        if (args.len() > self.max_args) || (args.len() < self.min_args) {
            error_skid(context, origin_index, origin_line, format!("Macro \'{}\' was given a number of arguments ({}) not in its acceptable range ({}-{})",
        self.symbol, args.len(), self.min_args, if self.max_args == usize::max_value() {"No Limit".to_string()} else {format!("{}", self.max_args)}));
            Vec::new()
        } else {
            (self.expansion)(input_file, origin_index, origin_line, context, args, scope)
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
            working_index: 0,
            templates: Vec::new(),
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
