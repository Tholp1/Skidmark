use crate::{
    stringtools::{split_keep_delimiters, strings_to_tokens},
    types::{InputFile, Token},
};

pub fn macro_clear(_file: &mut InputFile, _args: &Vec<String>) -> Vec<Token> {
    _file.tokens = _file.tokens.split_off(_file.working_index);
    _file.working_index = 0;
    return Vec::new();
}
