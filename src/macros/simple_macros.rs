// This file for implementations of short macros, im qualifying that as less than 30ish lines
use std::process::exit;

use chrono::Local;

use crate::{
    console::{error_skid, reminder_skid},
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
        error_skid(
            context,
            origin_index,
            origin_line,
            &format!(
                "Time only accepts 1 argument, got given {} ({:?})",
                args.len(),
                args
            ),
        );
        exit(1);
    }

    return split_to_tokens(t.format(&args[0]).to_string(), origin_index);
}

pub fn macro_filename(
    _file: &mut InputFile,
    origin_index: usize,
    _origin_line: usize,
    context: &mut ProjectContext,
    _args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    return split_to_tokens(
        context
            .file_for_index(origin_index)
            .unwrap()
            .to_str()
            .unwrap()
            .into(),
        origin_index,
    );
}

pub fn macro_filename_canonical(
    _file: &mut InputFile,
    origin_index: usize,
    _origin_line: usize,
    context: &mut ProjectContext,
    _args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    return split_to_tokens(
        context
            .file_for_index_canonical(origin_index)
            .unwrap()
            .to_str()
            .unwrap()
            .into(),
        origin_index,
    );
}

pub fn macro_reminder(
    _file: &mut InputFile,
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    reminder_skid(context, origin_index, origin_line, &args[0]);
    Vec::new()
}
