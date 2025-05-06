use std::path::PathBuf;

use crate::{macros::template::SkidTemplate, projectparse::ProjectContext};

pub struct Token {
    pub contents: String,
    pub origin_file: usize,
    pub line_number: usize,
}

pub struct InputFile {
    pub file_input: PathBuf,
    pub file_skidout: PathBuf,
    pub file_htmlout: PathBuf,
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

pub struct Macro<'a> {
    pub symbol: &'a str,
    pub expand: MacroExpansion,
    pub has_scope: bool, //takes blocks of text input as well as parameters using {{...}}
}

impl InputFile {
    pub fn new() -> InputFile {
        InputFile {
            file_input: "".into(),
            file_skidout: "".into(),
            file_htmlout: "".into(),
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
        return Token::new(
            self.contents.clone(),
            self.origin_file.clone(),
            self.line_number,
        );
    }
}
