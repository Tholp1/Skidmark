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
use stringtools::{
    collect_arguments, collect_block, split_keep_delimiters, split_to_tokens, strings_to_tokens,
    trim_whitespace_tokens,
};
use types::{InputFile, Macro, Token};

static DELIMITERS: [char; 10] = [' ', '\n', '\t', '(', ')', '{', '}', '\\', '\'', '\"'];

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
    //let mut escaped = false;

    while file.working_index < file.tokens.len() {
        //look for macros or blocks
        println!(">\"{}\"<", file.tokens[file.working_index].contents);

        if file.tokens[file.working_index].contents == "\\" {
            file.working_index += 2;
            continue;
        }

        let mut matched_macro: bool = false;
        if file.tokens[file.working_index]
            .contents
            .starts_with(['!', '&'])
        {
            let mut prefix_len = 1;
            let mut symbol = file.tokens[file.working_index].contents.clone();
            symbol = symbol.trim().to_string();

            if symbol.len() > 2 {
                let mut ephemeral = false;
                let same_file = file.tokens[file.working_index].origin_file != file.filename_input;

                // Inversely Ephemeral
                if symbol.starts_with("!&") {
                    prefix_len = 2;
                    ephemeral = !same_file;
                }
                // Ephemeral
                else if symbol.starts_with("&") {
                    ephemeral = same_file;
                }

                // Check if its a macro
                for m in &MACRO_LIST {
                    if &symbol[prefix_len..] == m.symbol {
                        matched_macro = true;
                        println!("Found a macro ({})", m.symbol);

                        let (args, args_tokcount) =
                            collect_arguments(&file.tokens[file.working_index..]);
                        let expansion: Vec<Token>;
                        let block_tokcount: usize;
                        if ephemeral {
                            expansion = Vec::new();
                            block_tokcount = 0;
                        } else {
                            if m.has_scope {
                                let block: Vec<Token>;
                                (block, block_tokcount) = collect_block(
                                    &file.tokens[(file.working_index + args_tokcount)..],
                                );
                                println!("{}", block_tokcount);
                                expansion = (m.expand)(file, &args, &block[..]);
                            } else {
                                block_tokcount = 0;
                                expansion = (m.expand)(file, &args, &Vec::new()[..]);
                            }
                        }

                        let trimmed = trim_whitespace_tokens(&expansion);

                        file.tokens.remove(file.working_index);
                        file.tokens.splice(
                            file.working_index
                                ..(file.working_index + args_tokcount + block_tokcount - 1),
                            trimmed.iter().cloned(),
                        );
                        if expansion.len() == 0 && file.working_index > 0 {
                            file.working_index -= 1;
                        }
                    }
                }
                // Check if its a block
                // for b in  &BLOCK_LIST {}}
            }
            if !matched_macro {
                println!(
                    "Token written as a function but no such function exists \"{}\"",
                    file.tokens[file.working_index].contents.trim()
                );
            }
        }
        if !matched_macro {
            file.working_index += 1;
        }
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
