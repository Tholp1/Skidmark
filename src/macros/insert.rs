use std::{
    env::Args,
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use crate::{
    projectparse::{FileIndexing, ProjectContext},
    stringtools::{split_keep_delimiters, strings_to_tokens},
    types::{InputFile, Token},
};

pub fn macro_insert(
    _file: &mut InputFile,
    origin_index: usize,
    origin_line: usize,
    context: &mut ProjectContext,
    args: &Vec<String>,
    _scope: &[Token],
) -> Vec<Token> {
    let origin_file = context
        .file_for_index(origin_index)
        .expect("Macro 'Insert' was given a bad origin index")
        .clone();
    if args.len() != 1 {
        println!(
            "[ERROR] {:?}:{} ;Insert only accepts 1 argument, got given {} ({:?})",
            origin_file.to_str(),
            origin_line,
            args.len(),
            args
        );
        exit(1);
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
        let mut include_path = context.input_folder.clone();
        include_path.push(&arg);

        if include_path.exists() && include_path.is_file() {
            ok = true;
            include_file = include_path.to_str().unwrap().to_string();
        }
    }

    if !ok {
        println!("[ERROR] \"{:?}\": Insert was unable to find the file \"{}\" relative to its origin or in project root.", origin_file.to_str(), arg);
        exit(1);
    }

    let mut output = fs::read_to_string(&include_file).expect("File unreadable or missing");
    while output.ends_with("\n") {
        output.pop();
    } //remove trailing newlines

    let split_output = split_keep_delimiters(output);
    return strings_to_tokens(
        split_output,
        context.index_of_file(&PathBuf::from(&include_file)),
    );
}
