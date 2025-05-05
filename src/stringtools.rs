use core::fmt;
use std::{ascii::escape_default, fmt::Arguments, ops::Index, process::exit, thread::sleep};

use super::DELIMITERS;
use crate::types::Token;

pub fn collect_arguments(tokens: &[Token]) -> (Vec<String>, usize) {
    // Arguments vec and number of tokens consumed
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
        in_token_count += 1; // This could be a problem if it something got split above..
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

pub fn collect_block(tokens: &[Token]) -> (Vec<Token>, usize) {
    let mut entered = false;
    let mut tokens_consumed: usize = 0;
    let mut entering_bracket_count = 0;
    let mut exiting_bracket_count = 0;
    let mut scope_count = 0; //incremented by '{{{', decremented by '}}}'
    let mut escaped = false;

    let mut block: Vec<Token> = Vec::new();

    // We dont really care about doing anything that in the block right now
    // maybe have the Token struct contain scope level later?
    for tok in tokens {
        tokens_consumed += 1;
        if !entered {
            if tok.contents.is_only_whitespace() {
                continue;
            }
            if tok.contents != "{"
            // Expected block start, got garbage
            {
                return (Vec::new(), 0);
            }
        }

        if escaped {
            escaped = false;
            entering_bracket_count = 0;
            exiting_bracket_count = 0;
            block.push(tok.clone());
            continue;
        }

        // Scope Start
        if tok.contents == "{" {
            entering_bracket_count += 1;
            if entering_bracket_count == 3 {
                scope_count += 1;
                entering_bracket_count = 0;
                if !entered {
                    entered = true;
                }
            }
        } else {
            entering_bracket_count = 0;
        }
        // Scope End
        if tok.contents == "}" {
            exiting_bracket_count += 1;
            if exiting_bracket_count == 3 {
                scope_count -= 1;
                entering_bracket_count = 0;
            }
        } else {
            entering_bracket_count = 0;
        }
        if tok.contents == "\\" {
            escaped = true;
        } else {
            block.push(tok.clone());
        }
    }
    return (block, tokens_consumed);
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

pub fn strings_to_tokens(in_strings: Vec<String>, origin_file: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line_count: u32 = 1;

    for str in in_strings {
        let current_line = line_count;
        for char in str.chars() {
            if char == '\n' {
                line_count += 1;
            }
        }
        let token: Token = Token::new(str, origin_file, current_line);
        tokens.push(token);
    }

    return tokens;
}

// Need to do some special case stuff so you can macros without spaces between
// (something like "stuff!insert(..)" is split to ["stuff","!insert(..)"] so it can be acted on later)
pub fn split_to_tokens(instr: String, origin_file: usize) -> Vec<Token> {
    let split = split_keep_delimiters(instr);
    let mut new_split: Vec<String> = Vec::new();
    for s in split {
        let prefix_offset = s.find(&['!', '&']).unwrap_or(s.len() + 1);
        if prefix_offset != 0 && prefix_offset != s.len() + 1 {
            let (first, second) = s.split_at(prefix_offset);
            println!("\"{}\", \"{}\"", first, second);
            new_split.push(first.to_string());
            new_split.push(second.to_string());
        } else {
            new_split.push(s);
        }
        //sleep(std::time::Duration::from_millis(10));
    }
    return strings_to_tokens(new_split, origin_file);
}

pub fn next_nonwhitespace_token(tokens: &Vec<Token>, index: usize) -> (bool, usize) {
    while index < tokens.len() {
        if tokens[index].contents.is_only_whitespace() {
            continue;
        }
        return (true, index);
    }
    return (false, 0);
}

//trim whitespace from the ends
pub fn trim_whitespace_tokens(tokens: &[Token]) -> &[Token] {
    let mut start: usize = 0;
    let mut end: usize = tokens.len();
    for tok in tokens {
        if !tok.contents.is_only_whitespace() {
            break;
        }
        start = start + 1;
    }

    for tok in tokens.iter().rev() {
        if !tok.contents.is_only_whitespace() {
            break;
        }
        end = end - 1;
    }

    return &tokens[start..end];
}

pub trait OnlyWhitespace {
    fn is_only_whitespace(&self) -> bool;
}

impl OnlyWhitespace for String {
    fn is_only_whitespace(&self) -> bool {
        for c in self.chars() {
            if !c.is_whitespace() {
                return false;
            }
        }
        return true;
    }
}
