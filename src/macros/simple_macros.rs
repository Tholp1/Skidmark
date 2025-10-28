// This file for implementations of short macros, im qualifying that as less than 30ish lines
use std::process::exit;

use chrono::Local;

use crate::{
    console::{error_skid, reminder_skid, warn_skid},
    project::{Indexing, Project},
    stringtools::split_to_tokens,
    types::{SkidContext, Token},
};

pub fn macro_time(
    origin_index: usize,
    origin_line: usize,
    context: &mut Project,
    _skid_context: &mut SkidContext,
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
    _origin_line: usize,
    proj_context: &mut Project,
    _skid_context: &mut SkidContext,
    _args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    return split_to_tokens(
        proj_context
            .file_for_index(origin_index)
            .unwrap()
            .to_str()
            .unwrap()
            .into(),
        origin_index,
    );
}

pub fn macro_output_filename(
    origin_index: usize,
    origin_line: usize,
    proj_context: &mut Project,
    _skid_context: &mut SkidContext,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    let mut in_filepath = proj_context.input_folder.clone();
    if args.len() == 0 {
        in_filepath.push(proj_context.file_for_index(origin_index).unwrap());
    } else {
        in_filepath.push(&args[0]);
    }

    if in_filepath.exists() {
        for g in &proj_context.filegroups {
            if !g.process {
                continue;
            }
            for f in &g.files {
                if f.file_input == in_filepath {
                    let stripped = f
                        .file_out
                        .strip_prefix(&proj_context.output_folder)
                        .unwrap();
                    return split_to_tokens(stripped.to_str().unwrap().into(), origin_index);
                }
            }
        }
    }
    warn_skid(
        proj_context,
        origin_index,
        origin_line,
        &format!(
            "output_filename given a file with no matching output file ({:?}), returning empty",
            in_filepath
        ),
    );
    Vec::new()
}

pub fn macro_filename_canonical(
    origin_index: usize,
    _origin_line: usize,
    context: &mut Project,
    _skid_context: &mut SkidContext,
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
    context: &mut Project,
    _skid_context: &mut SkidContext,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    reminder_skid(context, origin_index, origin_line, &args[0]);
    Vec::new()
}
