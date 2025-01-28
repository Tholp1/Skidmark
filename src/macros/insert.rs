use std::{env::Args, fs};

use crate::{
    stringtools::{split_keep_delimiters, strings_to_tokens},
    types::{InputFile, Token},
};

pub fn macro_insert(_file: &mut InputFile, args: &Vec<String>, _scope: &[Token]) -> Vec<Token> {
    print!("\nargs: {:?}\n", args);
    let mut output = fs::read_to_string(args[0].clone()).expect("File unreadable or missing");
    if output.ends_with("\n") {
        output.pop();
    } //remove trailing newline

    let split_output = split_keep_delimiters(output);
    return strings_to_tokens(split_output, args[0].clone());
}
