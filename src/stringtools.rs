use super::DELIMITERS;
use crate::types::Token;

pub fn collect_arguments(tokens: &[Token]) -> Option<(Vec<String>, usize)> {
    // Returns arguments vec and number of tokens to be consumed
    //let mut output = Vec::new();

    let mut quoted: bool = false;
    let mut escaped: bool = false;
    let mut entered: bool = false;
    let mut arg = "".to_string();
    let mut args: Vec<String> = Vec::new();

    let mut in_token_count = 0;
    let mut exited_cleanly = false;

    for tok in tokens {
        let c = tok.contents;

        in_token_count += 1;
        if c.is_whitespace() && !entered {
            continue;
        }

        if !entered && c == '(' {
            entered = true;
            continue;
        }

        if !entered {
            break;
        }

        if !quoted && tok.contents == ')' {
            exited_cleanly = true;
            if !arg.is_empty() {
                args.push(arg.clone());
                arg.clear();
            }
            break;
        }

        if c == '\"' && !escaped {
            quoted = !quoted;
            continue;
        }

        if c == '\\' && !escaped {
            escaped = true;
            continue;
        }

        if c.is_whitespace() && !quoted {
            if !arg.is_empty() {
                args.push(arg.clone());
                arg.clear();
            }
            continue;
        }
        arg.push(c);
    }

    if !entered || !exited_cleanly {
        return None;
    }
    return Some((args, in_token_count));
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
    let mut escaped_tok: Token = Token::new('\\', 0, 0);
    for tok in tokens {
        tokens_consumed += 1;
        if !entered {
            if tok.contents.is_whitespace() {
                continue;
            }
            if tok.contents != '{'
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
        if tok.contents == '{' && !escaped {
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
        if tok.contents == '}' && !escaped {
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

        if tok.contents == '\\' {
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
        for c in str.chars() {
            let current_line = line_count;
            for char in str.chars() {
                if char == '\n' {
                    line_count += 1;
                }
            }
            let token: Token = Token::new(c, origin_file, current_line);
            tokens.push(token);
        }
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

pub fn next_nonwhitespace_token(tokens: &Vec<Token>, index: usize) -> Option<usize> {
    while index < tokens.len() {
        if tokens[index].contents.is_whitespace() {
            continue;
        }
        return Some(index);
    }
    return None;
}

//trim whitespace from the ends
pub fn trim_whitespace_tokens(tokens: &[Token]) -> &[Token] {
    let mut start: usize = 0;
    let mut end: usize = tokens.len();
    for tok in tokens {
        if !tok.contents.is_whitespace() {
            break;
        }
        start = start + 1;
    }

    for tok in tokens.iter().rev() {
        if !tok.contents.is_whitespace() {
            break;
        }
        end = end - 1;
    }

    return &tokens[start..end];
}

// Find the first instance of the pattern
pub fn find_pattern(tokens: &[Token], pat: String) -> Option<(usize, usize)> {
    // (startpoint, length)

    let split_pattern = split_to_tokens(pat, 0);
    let mut pattern_index: usize = 0;
    let mut token_index: usize = 0;

    while token_index < tokens.len() && tokens.len() - token_index >= split_pattern.len() {
        for t in &tokens[token_index..] {
            if t.contents == split_pattern[pattern_index].contents {
                pattern_index += 1;
                if pattern_index == split_pattern.len() {
                    return Some((token_index, split_pattern.len()));
                }
            } else {
                pattern_index = 0;
                token_index += 1;
                break;
            }
        }
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
