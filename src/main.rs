mod blocktypes;
mod macros;
mod stringtools;
mod types;

use macros::{
    include::{self, macro_include},
    MACRO_LIST,
};
use std::{
    env,
    fs::{self, File},
    io::Write,
    process::{exit, Output},
};
use stringtools::{collect_arguments, split_keep_delimiters, strings_to_tokens};
use types::{InputFile, Macro, Token};

static DELIMITERS: [char; 7] = [' ', '\n', '\t', '(', ')', '{', '}'];

fn main() {
    let mut files: Vec<types::InputFile> = Vec::new();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for file in args.iter() {
        let mut new_file = types::InputFile::new();
        new_file.filename_in = file.to_string();
        new_file.filename_out = file.to_string() + ".out";
        files.push(new_file);
    }
    println!("{:?}", args);
    for f in &mut files {
        process_file(f);
    }
}

fn process_file(file: &mut InputFile) {
    let contents = fs::read_to_string(&file.filename_in).expect("File unreadable or missing");
    //println!("{}\n {}", f.filename_out, contents);

    file.tokens = strings_to_tokens(split_keep_delimiters(contents), file.filename_in.clone());

    let mut index = 0;

    while index < file.tokens.len() {
        //look for macros or blocks
        //println!(">\"{}\"<", file.tokens[index].contents);

        if file.tokens[index].contents.starts_with(['!', '&']) {
            let mut matched = false;

            for m in &MACRO_LIST {
                if &file.tokens[index].contents.trim()[1..] == m.symbol {
                    matched = true;
                    println!("Found a macro ({})", m.symbol);
                    let mut ephemeral = false;
                    if file.tokens[index].contents.starts_with('&')
                        && file.tokens[index].origin_file != file.filename_in
                    {
                        println!("Skipping Ephermal macro from included file.");
                        ephemeral = true;
                    }

                    let (args, tokcount) = collect_arguments(&file.tokens[index..]);
                    let expansion: Vec<Token>;
                    if ephemeral {
                        expansion = Vec::new();
                    } else {
                        expansion = (m.expand)(&file, &args);
                    }
                    file.tokens.remove(index);
                    file.tokens.splice(index..(index + tokcount - 1), expansion);
                }
            }

            // for b in  &BLOCK_LIST {}

            if !matched {
                println!(
                    "Token written as a function but no such function exists \"{}\"",
                    file.tokens[index].contents.trim()
                );
            }
        }

        index += 1;
    }
    //println!("{:?}", file.tokens);
    let mut full_output: String = "".to_string();
    for t in &file.tokens {
        full_output += &t.contents;
    }
    fs::write(&file.filename_out, full_output).expect("Couldn't write to file");
}
