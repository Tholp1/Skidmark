use std::{process::exit, thread::scope};

use crate::{
    projectparse::{FileIndexing, ProjectContext},
    stringtools::{find_pattern, split_to_tokens, strings_to_tokens, WhitespaceChecks},
    types::{InputFile, Token},
};

use super::MACRO_LIST;

pub struct SkidTemplate {
    pub symbol: String,
    pub args: Vec<String>,
    pub tokens: Vec<Token>,

    pub has_scope: bool,
}

impl SkidTemplate {
    pub fn new(name: String, args: &[String], tokens: &[Token]) -> SkidTemplate {
        let scoped: bool = find_pattern(&tokens, "[[{}]]".into()).is_some();

        SkidTemplate {
            symbol: name,
            args: args.to_vec(),
            tokens: tokens.to_vec(),
            has_scope: scoped,
        }
    }

    pub fn expand(
        &self,
        //_file: &mut InputFile,
        origin_index: usize,
        _context: &mut ProjectContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token> {
        println!("{:?}", args);

        let mut output = self.tokens.clone();

        for tok in &mut output {
            tok.origin_file = origin_index;
        }

        let mut args_index: usize = 0;
        for param in &self.args {
            let mut found_pattern = find_pattern(&output, format!("[[{}]]", param));
            while found_pattern.is_some() {
                let (start, len) = found_pattern.unwrap();
                let replacement = split_to_tokens(args[args_index].clone(), origin_index);
                output.splice(start..start + len, replacement);
                found_pattern = find_pattern(&output, format!("[[{}]]", param));
            }
            args_index += 1;
        }

        let mut found_pattern = find_pattern(&output, "[[{}]]".into());
        while found_pattern.is_some() {
            let (start, len) = found_pattern.unwrap();
            let replacement = scope.to_vec();
            output.splice(start..start + len, replacement);
            found_pattern = find_pattern(&output, "[[{}]]".into());
        }

        output
    }
}

pub fn macro_template(
    file: &mut InputFile,
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    for t in &file.templates {
        if t.symbol == args[0] {
            println!(
                "{:?}:{} ; Attempted template redefinition of \"{}\"",
                context.file_for_index(origin_index).unwrap(),
                origin_line,
                args[0]
            );
            exit(1);
        }
    }

    for t in &MACRO_LIST {
        if t.symbol == args[0] {
            println!(
                "{:?}:{} ; Attempted to make a template using a reserved name \"{}\"",
                context.file_for_index(origin_index).unwrap(),
                origin_line,
                args[0]
            );
            exit(1);
        }
    }

    let mut used_params = 0;
    for param in &args[1..] {
        if find_pattern(scope, format!("[[{}]]", param)).is_some() {
            used_params += 1;
        }
        if param.contains_whitespace() {
            println!(
                "{:?}:{} ; Attempted to make a template with a parameter that contains whitespace \"{}\"",
                context.file_for_index(origin_index).unwrap(),
                origin_line,
                param
            );
            exit(1);
        }
    }

    if used_params < args.len() - 1 {
        println!(
            "{:?}:{} ; Template definition of \"{}\" has {} paramters but only uses {}",
            context.file_for_index(origin_index).unwrap(),
            origin_line,
            args[0],
            args.len() - 1,
            used_params
        );
        exit(1);
    }

    let template = SkidTemplate::new(args[0].clone(), &args[1..], scope);
    file.templates.push(template);

    return Vec::new();
}
