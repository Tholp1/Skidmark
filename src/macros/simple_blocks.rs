// This file for implementations of short blocks, im qualifying that as less than 30ish lines

use crate::{
    console::{error_skid, warn_skid},
    project::ProjectContext,
    stringtools::{find_pattern, split_to_tokens, TokenTools},
    types::{InputFile, Token},
};

pub fn macro_comment(
    _file: &mut InputFile,
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut ProjectContext,
    _args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    return Vec::new();
}

pub fn macro_section(
    _file: &mut InputFile,
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut ProjectContext,
    _args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    let mut tokens = Vec::new();
    for tok in scope {
        tokens.push(tok.clone());
    }
    return tokens;
}

pub fn macro_repeat(
    _file: &mut InputFile,
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut ProjectContext,
    args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    let mut count = 0;
    if args.len() > 0 {
        count = args[0].parse().unwrap_or(0);
    }

    let mut tokens = Vec::new();
    for _i in 0..count {
        for tok in scope {
            tokens.push(tok.clone());
        }
    }
    return tokens;
}

pub fn macro_for_each_arg(
    _file: &mut InputFile,
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    let mut output = Vec::new();
    let block: Vec<Token> = scope.into();
    let varname = &args[0];
    let real_args = &args[1..];

    let mut replacement_count: usize = 0;

    let mut replacement_pattern = find_pattern(scope, format!("[[{}..1]]", varname));

    if replacement_pattern.is_none() {
        warn_skid(
            context,
            origin_index,
            origin_line,
            &format!(
                "Macro `for_each_arg` given block with no \"[[{}..1]]\", intentional?",
                varname
            ),
        );
    }

    while replacement_pattern.is_some() {
        replacement_count += 1;
        replacement_pattern =
            find_pattern(scope, format!("[[{}..{}]]", varname, replacement_count + 1));
    }

    if replacement_count == 0 {
        for _i in 0..real_args.iter().count() {
            output.append(&mut block.clone());
        }
        return output;
    }

    if real_args.len() % replacement_count != 0 {
        error_skid(context, origin_index, origin_line,
            &format!("`for_each_var` was not given a number of arguments({}) that was a multiple of its replacement posistions({}) (got {:?})",
            real_args.len(),
            replacement_count,
            real_args));
    }

    let mut replacement_index: usize = 0;
    let mut arg_output: Vec<Token> = block.clone();
    for arg in real_args {
        let mut found_pattern = find_pattern(
            &arg_output,
            format!("[[{}..{}]]", varname, replacement_index + 1),
        );

        while found_pattern.is_some() {
            let (start, len) = found_pattern.unwrap();
            let replacement = split_to_tokens(arg.clone(), origin_index);
            arg_output.splice(start..start + len, replacement);
            found_pattern = find_pattern(
                &arg_output,
                format!("[[{}..{}]]", varname, replacement_index + 1),
            );
            //println!("{}", replacement_index + 1);
        }

        //println!("{} {}", replacement_index, replacement_count);
        replacement_index += 1;
        if replacement_index == replacement_count {
            replacement_index = 0;
            output.append(&mut arg_output.trim_whitespace().into());
            arg_output = block.clone();
            //println!("push");
        }
        //println!("test");
    }

    return output;
}
