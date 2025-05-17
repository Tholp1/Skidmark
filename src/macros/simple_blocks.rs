// This file for implementations of short blocks, im qualifying that as less than 30ish lines

use crate::{
    projectparse::ProjectContext,
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

pub fn macro_skip(
    _file: &mut InputFile,
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut ProjectContext,
    _args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    Vec::new()
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
