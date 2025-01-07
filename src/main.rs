mod blocktypes;
mod macros;
mod stringtools;
mod types;

use macros::MACRO_LIST;
use markdown::{to_html_with_options, CompileOptions, Options};
use std::{
    env,
    fs::{self, File},
    io::Write,
    process::{exit, Output},
};
use stringtools::{collect_arguments, split_keep_delimiters, split_to_tokens, strings_to_tokens};
use types::{InputFile, Macro, Token};

static DELIMITERS: [char; 7] = [' ', '\n', '\t', '(', ')', '{', '}'];

fn main() {
    let mut files: Vec<types::InputFile> = Vec::new();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for file in args.iter() {
        let mut new_file = types::InputFile::new();
        new_file.filename_input = file.to_string();
        new_file.filename_skidout = file.to_string() + ".skidout";
        new_file.filename_htmlout = file.to_string() + ".html";
        files.push(new_file);
    }
    println!("{:?}", args);
    for f in &mut files {
        process_file(f);
    }
}

fn process_file(file: &mut InputFile) {
    let contents = fs::read_to_string(&file.filename_input).expect("File unreadable or missing");
    //println!("{}\n {}", f.filename_out, contents);

    //file.tokens = strings_to_tokens(split_keep_delimiters(contents), file.filename_input.clone());
    file.tokens = split_to_tokens(contents, file.filename_input.clone());

    while file.working_index < file.tokens.len() {
        //look for macros or blocks
        //println!(">\"{}\"<", file.tokens[index].contents);

        if file.tokens[file.working_index]
            .contents
            .starts_with(['!', '&'])
        {
            let mut matched: bool = false;

            for m in &MACRO_LIST {
                let symbol = file.tokens[file.working_index].contents.trim();
                if symbol.len() < 2
                {
                    continue;
                }
                if &symbol[1..] == m.symbol {
                    matched = true;
                    println!("Found a macro ({})", m.symbol);
                    let mut ephemeral = false;
                    if file.tokens[file.working_index].contents.starts_with('&')
                        && file.tokens[file.working_index].origin_file != file.filename_input
                    {
                        println!("Skipping Ephermal macro from included file.");
                        ephemeral = true;
                    }

                    let (args, tokcount) = collect_arguments(&file.tokens[file.working_index..]);
                    let expansion: Vec<Token>;
                    if ephemeral {
                        expansion = Vec::new();
                    } else {
                        expansion = (m.expand)(file, &args);
                    }
                    file.tokens.remove(file.working_index);
                    file.tokens.splice(
                        file.working_index..(file.working_index + tokcount - 1),
                        expansion,
                    );
                }
            }

            // for b in  &BLOCK_LIST {}

            if !matched {
                println!(
                    "Token written as a function but no such function exists \"{}\"",
                    file.tokens[file.working_index].contents.trim()
                );
            }
        }

        file.working_index += 1;
    }
    //println!("{:?}", file.tokens);
    let mut skid_output: String = "".to_string();
    for t in &file.tokens {
        skid_output += &t.contents;
    }
    fs::write(&file.filename_skidout, &skid_output).expect("Couldn't write skid to file");

    //let html_output = markdown::to_html(&skid_output);
    let html_output = markdown::to_html_with_options(
        &skid_output,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::gfm()
            },
            ..Options::gfm()
        },
    )
    .unwrap();
    fs::write(&file.filename_htmlout, &html_output).expect("Couldn't write html to file");
    println!("{} written.", file.filename_htmlout);
}
