use std::path::PathBuf;

use crate::{
    console::error_skid,
    macros::{simple_blocks::macro_comment, template::SkidTemplate},
    project::Project,
};

pub struct Token {
    //pub contents: String,
    pub contents: char,
    pub origin_index: usize,
    pub template_origin: usize,
    pub origin_line: usize,
    pub section_index: usize,
    pub pre_proccessed: bool,
}

impl PartialEq<char> for Token {
    fn eq(&self, other: &char) -> bool {
        self.contents == *other
    }
}

pub struct InputFile {
    pub file_input: PathBuf,
    pub file_skidout: PathBuf,
    pub file_out: PathBuf,
}

pub struct SkidContext {
    pub templates: Vec<SkidTemplate>,
    pub file_index: usize,
}

impl SkidContext {
    pub fn new(file_index: usize) -> SkidContext {
        SkidContext {
            templates: Vec::new(),
            file_index,
        }
    }
}

type MacroExpansion =
    fn(usize, usize, &mut Project, &mut SkidContext, &Vec<String>, &[Token]) -> Vec<Token>;
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
    pub takes_block: bool, //takes blocks of text input as well as parameters using [[{}]]
    pub min_args: usize,
    pub max_args: usize,
}

pub trait MacroExpand {
    fn expand(
        &self,
        origin_index: usize,
        origin_line: usize,
        context: &mut Project,
        skid_context: &mut SkidContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token>;
}

pub trait IsScoped {
    fn is_scoped(&self) -> bool;
}

impl Macro {
    fn default() -> Macro {
        Macro {
            symbol: "default_symbol",
            expansion: macro_comment,
            takes_block: true,
            min_args: 0,
            max_args: usize::max_value(),
        }
    }
}

impl MacroExpand for Macro {
    fn expand(
        &self,
        origin_index: usize,
        origin_line: usize,
        proj_context: &mut Project,
        skid_context: &mut SkidContext,
        args: &Vec<String>,
        block: &[Token],
    ) -> Vec<Token> {
        if (args.len() > self.max_args) || (args.len() < self.min_args) {
            error_skid(proj_context, origin_index, origin_line, &format!("Macro \'{}\' was given a number of arguments ({}) not in its acceptable range ({}-{})",
        self.symbol, args.len(), self.min_args, if self.max_args == usize::max_value() {"No Limit".to_string()} else {format!("{}", self.max_args)}));
            Vec::new()
        } else {
            (self.expansion)(
                origin_index,
                origin_line,
                proj_context,
                skid_context,
                args,
                block,
            )
        }
    }
}

impl IsScoped for Macro {
    fn is_scoped(&self) -> bool {
        self.takes_block
    }
}

impl InputFile {
    pub fn new() -> InputFile {
        InputFile {
            file_input: "".into(),
            file_skidout: "".into(),
            file_out: "".into(),
        }
    }
}

impl Token {
    pub fn new(contents: char, origin_file: usize, line_number: usize) -> Token {
        Token {
            contents,
            origin_index: origin_file,
            template_origin: origin_file,
            origin_line: line_number,
            section_index: 0,
            pre_proccessed: false,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        let mut t = Token::new(
            self.contents.clone(),
            self.origin_index.clone(),
            self.origin_line,
        );
        t.section_index = self.section_index;
        t.template_origin = self.template_origin;
        t.pre_proccessed = self.pre_proccessed;
        return t;
    }
}
