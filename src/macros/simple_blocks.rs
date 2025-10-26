// This file for implementations of short blocks, im qualifying that as less than 30ish lines
use crate::{
    console::*,
    project::Project,
    stringtools::TokenTools,
    types::{SkidContext, Token},
};

pub fn macro_comment(
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut Project,
    _skid_context: &mut SkidContext,
    _args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    return Vec::new();
}

pub fn macro_section(
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut Project,
    _skid_context: &mut SkidContext,
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
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut Project,
    _skid_context: &mut SkidContext,
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
