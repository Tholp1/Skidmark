use std::{fmt::format, process::exit, thread::scope};

use crate::{
    console::error_skid,
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
    pub allows_trailing_args: bool,
}

impl SkidTemplate {
    pub fn new(name: String, args: &[String], tokens: &[Token]) -> SkidTemplate {
        let scoped: bool = find_pattern(&tokens, "[[{}]]".into()).is_some();
        let trailing: bool = find_pattern(&tokens, "[[..]]".into()).is_some()
            || find_pattern(&tokens, "[[\"..\"]]".into()).is_some();

        SkidTemplate {
            symbol: name,
            args: args.to_vec(),
            tokens: tokens.to_vec(),
            has_scope: scoped,
            allows_trailing_args: trailing,
        }
    }

    pub fn expand(
        &self,
        //_file: &mut InputFile,
        origin_index: usize,
        origin_line: usize,
        context: &mut ProjectContext,
        args: &Vec<String>,
        scope: &[Token],
    ) -> Vec<Token> {
        //println!("{:?}", args);

        if !self.allows_trailing_args && args.len() != self.args.len() {
            // println!(
            //     "[ERROR] {:?}:{}; Template \"{}\" requires exactly {} arguments, got given {} ({:?})",
            //     context.file_for_index(origin_index).unwrap(),
            //     origin_line,
            //     self.symbol,
            //     self.args.len(),
            //     args.len(),
            //     args
            // );
            // exit(1);

            error_skid(
                context,
                origin_index,
                origin_line,
                format!(
                    "Template \"{}\" requires exactly {} arguments, got given {} ({:?})",
                    self.symbol,
                    self.args.len(),
                    args.len(),
                    args
                ),
            );
        }
        if self.allows_trailing_args && args.len() < self.args.len() {
            error_skid(
                context,
                origin_index,
                origin_line,
                format!(
                    "Template \"{}\" requires at least {} arguments, got given {} ({:?})",
                    self.symbol,
                    self.args.len(),
                    args.len(),
                    args
                ),
            );
        }

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

        //replace [[..]] with space seperated remaining args
        let mut found_trailing_pattern = find_pattern(&output, "[[..]]".into());
        while found_trailing_pattern.is_some() {
            let (start, len) = found_trailing_pattern.unwrap();
            let mut replacement = Vec::new();
            for arg in &args[self.args.len()..] {
                replacement.append(&mut split_to_tokens(arg.clone() + " ".into(), origin_index));
            }
            output.splice(start..start + len, replacement);
            found_trailing_pattern = find_pattern(&output, "[[..]]".into());
        }

        //replace [[".."]] with space seperated quoted remaining args
        found_trailing_pattern = find_pattern(&output, "[[\"..\"]]".into());
        while found_trailing_pattern.is_some() {
            let (start, len) = found_trailing_pattern.unwrap();
            let mut replacement = Vec::new();
            for arg in &args[self.args.len()..] {
                replacement.append(&mut split_to_tokens(
                    "\"".to_string() + arg + "\" ".into(),
                    origin_index,
                ));
            }
            output.splice(start..start + len, replacement);
            found_trailing_pattern = find_pattern(&output, "[[\"..\"]]".into());
        }

        let mut found_block_pattern = find_pattern(&output, "[[{}]]".into());
        while found_block_pattern.is_some() {
            let (start, len) = found_block_pattern.unwrap();
            let replacement = scope.to_vec();
            output.splice(start..start + len, replacement);
            found_block_pattern = find_pattern(&output, "[[{}]]".into());
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
            error_skid(
                context,
                origin_index,
                origin_line,
                format!("Attempted template redefinition of \"{}\"", args[0]),
            );
        }
    }

    for t in MACRO_LIST {
        if t.symbol == args[0] {
            error_skid(
                context,
                origin_index,
                origin_line,
                format!(
                    "Attempted to make a template using a reserved name \"{}\"",
                    args[0]
                ),
            );
        }
    }

    for arg in args {
        if arg == ".." || arg == "\"..\"" {
            error_skid(
                context,
                origin_index,
                origin_line,
                format!(
                    "Attempted to make a template using a reserved parameter name \"{}\"",
                    arg
                ),
            );
        }
    }

    let mut used_params = 0;
    for param in &args[1..] {
        if find_pattern(scope, format!("[[{}]]", param)).is_some() {
            used_params += 1;
        }
        if param.contains_whitespace() {
            error_skid(
                context,
                origin_index,
                origin_line,
                format!(
                    "Attempted to make a template with a parameter that contains whitespace \"{}\"",
                    param
                ),
            );
        }
    }

    if used_params < args.len() - 1 {
        error_skid(
            context,
            origin_index,
            origin_line,
            format!(
                "Template definition of \"{}\" has {} paramters but only uses {}",
                args[0],
                args.len() - 1,
                used_params
            ),
        );
    }

    let template = SkidTemplate::new(args[0].clone(), &args[1..], scope);
    file.templates.push(template);

    return Vec::new();
}
