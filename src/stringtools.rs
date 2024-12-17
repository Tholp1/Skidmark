use core::fmt;
use std::{fmt::Arguments, ops::Index, process::exit};

use super::DELIMITERS;
use crate::types::Token;

pub fn collect_arguments(tokens: &[Token]) -> (Vec<String>, usize) {
    //let mut output = Vec::new();
    let mut split_tokens = Vec::new();
    for tok in tokens {
        for s in split_keep_delimiters(tok.contents.clone()) {
            split_tokens.push(s);
        }
    }

    let mut quoted: bool = false;
    let mut entered: bool = false;
    let mut arg = "".to_string();
    let mut args: Vec<String> = Vec::new();

    let mut in_token_count = 0;

    for tok in split_tokens {
        in_token_count += 1;
        if tok.starts_with([' ', '\t']) && !quoted {
            continue;
        }

        if !entered && tok.starts_with('(') {
            entered = true;
            continue;
        }

        if !entered {
            continue;
        }

        if !quoted && tok.starts_with(')') {
            break;
        }

        let mut i = 0;
        while i < tok.len() {
            let c = tok.chars().nth(i).unwrap();
            i += 1;

            if c == '\"' {
                quoted = !quoted;
                continue;
            }

            arg.push(c);
        }

        if !quoted {
            args.push(arg.clone());
            arg.clear();
        }
    }

    return (args, in_token_count);
}

// Theres no std function to have the delimiters be their own element in the out vector
// clean it up a bit here
pub fn split_keep_delimiters(instr: String) -> Vec<String> {
    let split: Vec<&str> = instr.split_inclusive(DELIMITERS).collect();
    let mut output = Vec::new();

    for s in split {
        if s.ends_with(DELIMITERS) {
            let (token, ending) = s.split_at(s.len() - 1);
            if token.len() > 0 {
                output.push(token.to_string());
            }
            output.push(ending.to_string());
        } else {
            output.push(s.to_string());
        }
    }
    return output;
}

pub fn strings_to_tokens(instrings: Vec<String>, origin_file: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut linecount: u32 = 1;

    for str in instrings {
        let currentline = linecount;
        for char in str.chars() {
            if char == '\n' {
                linecount += 1;
            }
        }
        let token: Token = Token::new(str, origin_file.clone(), currentline);
        tokens.push(token);
    }

    return tokens;
}
pub fn next_nonwhitespace_token(tokens: &Vec<Token>, index: usize) -> (bool, usize) {
    while index < tokens.len() {
        if tokens[index].contents.starts_with([' ', '\t', '\n']) {
            continue;
        }
        return (true, index);
    }
    return (false, 0);
}

pub trait IsDelimiter {
    fn is_delimiter(&self) -> bool;
}

impl IsDelimiter for char {
    fn is_delimiter(&self) -> bool {
        for d in DELIMITERS {
            if *self == d {
                return true;
            }
        }
        return false;
    }
}
