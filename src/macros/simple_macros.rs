// This file for implementations of short macros, im qualifying that as less than 30ish lines
use std::process::exit;

use chrono::Local;

use crate::{
    console::{error_skid, reminder_skid},
    macros::template::SkidTemplate,
    project::{FileIndexing, ProjectContext},
    stringtools::split_to_tokens,
    types::Token,
};

pub fn macro_time(
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    _templates: &mut Vec<SkidTemplate>,
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
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    _templates: &mut Vec<SkidTemplate>,
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
    origin_index: usize,
    _origin_line: usize,
    context: &mut ProjectContext,
    _templates: &mut Vec<SkidTemplate>,
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
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    _templates: &mut Vec<SkidTemplate>,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    reminder_skid(context, origin_index, origin_line, &args[0]);
    Vec::new()
}
