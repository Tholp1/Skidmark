use std::process::exit;

use chrono::{DateTime, Local};

use crate::{
    projectparse::{FileIndexing, ProjectContext},
    stringtools::split_to_tokens,
    types::{InputFile, Token},
};

pub fn macro_clear(
    _file: &mut InputFile,
    _origin_index: usize,
    _origin_line: usize,
    _context: &mut ProjectContext,
    _args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    _file.tokens = _file.tokens.split_off(_file.working_index);
    _file.working_index = 0;
    return Vec::new();
}

pub fn macro_time(
    file: &mut InputFile,
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    let t = Local::now();

    if args.len() != 1 {
        let origin_file = context
            .file_for_index(origin_index)
            .expect("Macro 'Time' was given a bad origin index")
            .clone();
        println!(
            "{:?}:{} ;Time only accepts 1 argument, got given {} ({:?})",
            origin_file.to_str(),
            origin_line,
            args.len(),
            args
        );
        exit(1);
    }

    return split_to_tokens(t.format(&args[0]).to_string(), origin_index);
}
