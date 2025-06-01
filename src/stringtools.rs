use core::fmt;
use std::{ascii::escape_default, error, fmt::Arguments, ops::Index, process::exit, thread::sleep};

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

        for c in tok.chars() {
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

pub fn collect_block(tokens: &[Token]) -> Option<(Vec<Token>, usize)> {
    let mut entered = false;
    let mut tokens_consumed: usize = 0;
    let mut entering_bracket_count = 0;
    let mut exiting_bracket_count = 0;
    let mut scope_count = 0; //incremented by '{{{', decremented by '}}}'
    let mut escaped = false;

    let mut block: Vec<Token> = Vec::new();

    // We dont really care about doing anything that in the block right now
    // maybe have the Token struct contain scope level later?
    let mut escaped_tok: Token = Token::new("\\".into(), 0, 0);
    for tok in tokens {
        tokens_consumed += 1;
        if !entered {
            if tok.contents.is_only_whitespace() {
                continue;
            }
            if tok.contents != "{"
            // Expected block start, got garbage
            {
                // println!("Expected block start, got {}",tok.contents);
                // for t in &block
                // {
                //     print!("{} ", t.contents);
                // }
                // exit(1);
                return None;
            }
        }

        let mut escaped_used = false;

        // Scope Start
        if tok.contents == "{" && !escaped {
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
            if escaped {
                escaped_used = true;
            }
        }
        // Scope End
        if tok.contents == "}" && !escaped {
            exiting_bracket_count += 1;
            if exiting_bracket_count == 3 {
                scope_count -= 1;
                entering_bracket_count = 0;
            }
            if scope_count == 0 {
                break;
            }
        } else {
            exiting_bracket_count = 0;
            if escaped {
                escaped_used = true;
            }
        }

        if escaped_used {
            escaped = false;
            block.push(escaped_tok.clone());
        }

        if tok.contents == "\\" {
            escaped = true;
            escaped_tok = tok.clone();
        } else {
            block.push(tok.clone());
        }
    }

    if scope_count != 0 {
        return None;
    }

    // if block.len() == 6
    // // things get ugly if its empty
    // {
    //     let mut emptyblock = Vec::new();
    //     emptyblock.push(Token::new(
    //         "".into(),
    //         tokens[0].origin_file,
    //         tokens[0].line_number,
    //     ));
    //     return (emptyblock, tokens_consumed);
    // }
    // pop brackets, bad and ugly but idgaf
    block.drain(..3);
    block.drain(block.len() - 2..);
    return Some((block, tokens_consumed));
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
            //println!("({}, {})", token.to_string(), ending.to_string())
        } else {
            output.push(s.to_string());
        }
    }
    return output;
}

pub fn strings_to_tokens(in_strings: Vec<String>, origin_file: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line_count = 1;

    for str in in_strings {
        if str.len() == 0 {
            continue;
        }

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
        let prefix_offset = s.find(&['!', '&']);
        if prefix_offset.is_some() {
            let (first, second) = s.split_at(prefix_offset.unwrap());
            //println!("\"{}\", \"{}\"", first, second);
            if first.len() > 0 {
                new_split.push(first.to_string());
            }
            if second.len() > 0 {
                new_split.push(second.to_string());
            }
        } else {
            if s.len() > 0 {
                new_split.push(s);
            }
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

pub fn find_pattern(tokens: &[Token], pat: String) -> Option<(usize, usize)> {
    // (startpoint, length)
    // FIXME: this fucks up when the begining of a pattern is repeated
    // ex. searching for "[[hello]]" in "[[[[hello]]" yeilds None
    // ALSO, this is a coarse search, operating on tokens only, not the characters within
    let split_pattern = split_to_tokens(pat, 0);
    let mut pattern_index: usize = 0;
    let mut token_index: usize = 0;
    let mut working_pattern_index: usize = 0;

    for t in tokens {
        if t.contents == split_pattern[pattern_index].contents {
            pattern_index += 1;
        } else {
            pattern_index = 0;
            working_pattern_index = token_index + 1;
        }

        if pattern_index == split_pattern.len() {
            return Some((working_pattern_index, split_pattern.len()));
        }

        token_index += 1;
    }

    None
}

pub trait WhitespaceChecks {
    fn is_only_whitespace(&self) -> bool;
    fn contains_whitespace(&self) -> bool;
}

impl WhitespaceChecks for String {
    fn is_only_whitespace(&self) -> bool {
        for c in self.chars() {
            if !c.is_whitespace() {
                return false;
            }
        }
        return true;
    }

    fn contains_whitespace(&self) -> bool {
        for c in self.chars() {
            if c.is_whitespace() {
                return true;
            }
        }
        return false;
    }
}

pub trait TokenTools {
    fn trim_whitespace(&mut self) -> &[Token];
}

impl TokenTools for Vec<Token> {
    fn trim_whitespace(&mut self) -> &[Token] {
        return trim_whitespace_tokens(&self[..]);
    }
}
