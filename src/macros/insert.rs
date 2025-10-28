use std::{fs, path::PathBuf};

use crate::{
    console::error_skid,
    process_skid,
    project::{Indexing, Project},
    stringtools::split_to_tokens,
    types::{SkidContext, Token},
};

pub fn macro_insert(
    origin_index: usize,
    origin_line: usize,
    proj_context: &mut Project,
    skid_context: &mut SkidContext,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    let origin_file = proj_context
        .file_for_index_canonical(origin_index)
        .expect("Macro 'Insert' was given a bad origin index")
        .clone();

    let mut sections_ids_to_keep = Vec::new();

    if args.len() > 1 {
        for a in &args[1..] {
            let id = proj_context.index_of_section_name(a);
            sections_ids_to_keep.push(id);
        }
    }

    let mut arg = args[0].clone();
    let mut search_from_root = arg.starts_with("//");
    let mut ok = false;

    if search_from_root {
        arg.drain(0..2); //remove "//"
    }

    let mut include_file = "".to_string();
    if !search_from_root {
        let mut include_path = origin_file.clone();
        include_path.pop();
        include_path.push(&arg);

        if include_path.exists() && include_path.is_file() {
            ok = true;
            include_file = include_path.to_str().unwrap().to_string();
        } else {
            search_from_root = true;
        }
    }

    if search_from_root {
        let mut include_path = proj_context.input_folder.clone();
        include_path.push(&arg);

        if include_path.exists() && include_path.is_file() {
            ok = true;
            include_file = include_path.to_str().unwrap().to_string();
        }
    }

    if !ok {
        error_skid(proj_context, origin_index, origin_line, &format!("Insert was unable to find the file \"{}\" relative to its origin or in project root.", arg));
    }

    let mut output = fs::read_to_string(&include_file).expect("File unreadable or missing");
    while output.ends_with("\n") {
        output.pop();
    } //remove trailing newlines

    if sections_ids_to_keep.len() > 0 {
        let mut processed = process_skid(
            &split_to_tokens(
                output,
                proj_context.index_of_file(&PathBuf::from(&include_file)),
            ),
            proj_context,
            skid_context,
        );
        processed.retain(|t| sections_ids_to_keep.contains(&t.section_index));
        for t in &mut processed {
            t.pre_proccessed = true;
        }
        return processed;
    } else {
        return split_to_tokens(
            output,
            proj_context.index_of_file(&PathBuf::from(&include_file)),
        );
    }
}
