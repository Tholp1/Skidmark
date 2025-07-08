mod args;
mod console;
mod macros;
mod project;
mod stringtools;
mod types;

use clap::Parser;
use console::*;
use macros::MACRO_LIST;
use markdown::{CompileOptions, Constructs, Options, ParseOptions};
use project::{parse_project, FileIndexing, ProjectContext};
use std::{
    env,
    fs::{self},
    path::PathBuf,
};
use stringtools::{collect_arguments, collect_block, split_to_tokens, trim_whitespace_tokens};
use types::{InputFile, Token};

use crate::{args::ProgramArgs, types::Expand};

static DELIMITERS: &'static [char] = &[
    ' ', '\n', '\t', '(', ')', '{', '}', '[', ']', '<', '>', '\\', '\'', '\"', ';',
];

fn main() {
    // let args = ProgramArgs::parse();

    let mut project_folder = PathBuf::from(env::current_dir().unwrap().as_path());

    let mut project_path = project_folder.clone();
    project_path.push("skidmark.toml");

    while !project_path.exists() || project_path.is_dir() {
        let ok = project_folder.pop();
        if !ok {
            error_generic(
                &"No skidmark.toml project file found in this folder or ancestors.".into(),
            );
        }
        project_path = project_folder.clone();
        project_path.push("skidmark.toml");
    }
    println!("Operatting with {:?}", &project_path.as_os_str());
    assert!(env::set_current_dir(&project_folder).is_ok());

    let mut project = parse_project(&project_path);

    let mut num = 0;

    for group in &project.filegroups {
        num = num + group.files.len();
    }

    println!("Proccesing {} files.", num);
    for group in &mut project.filegroups {
        for infile in &mut group.files {
            process_file(infile, group.convert_html, &mut project.context);
        }
    }
}

fn process_file(file: &mut InputFile, convert_html: bool, context: &mut ProjectContext) {
    //}, context: &mut ProjectContext) {
    let contents = fs::read_to_string(&file.file_input).expect("File unreadable or missing");
    //println!("{}\n {}", f.filename_out, contents);

    //file.tokens = strings_to_tokens(split_keep_delimiters(contents), file.filename_input.clone());
    file.tokens = split_to_tokens(contents, context.index_of_file(&file.file_input));
    //let mut escaped = false;

    while file.working_index < file.tokens.len() {
        //look for macros or blocks
        //println!(">\"{}\"<", file.tokens[file.working_index].contents);

        if file.tokens[file.working_index].contents.len() == 0 {
            file.working_index += 1;
            continue;
        }

        if file.tokens[file.working_index].contents == "\\" {
            file.tokens[file.working_index].contents = "".into();
            file.working_index += 2;
            //println!("Hit backslash");
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
                let same_file = file.tokens[file.working_index].origin_file
                    != context.index_of_file(&file.file_input);

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
                for m in MACRO_LIST {
                    if &symbol[prefix_len..] == m.symbol {
                        matched_macro = true;
                        //println!("Found a macro ({})", m.symbol);

                        let (args, args_tokcount) =
                            collect_arguments(&file.tokens[file.working_index..]);
                        let expansion: Vec<Token>;
                        let block_tokcount: usize;
                        if m.has_scope {
                            //println!("is scoped.");

                            let block_opt =
                                collect_block(&file.tokens[(file.working_index + args_tokcount)..]);
                            if block_opt.is_none() {
                                error_skid(
                                    context,
                                    file.tokens[file.working_index].template_origin,
                                    file.tokens[file.working_index].line_number,
                                    &"Malformed Block".into(),
                                );
                            }
                            let block: Vec<Token>;
                            (block, block_tokcount) = block_opt.unwrap();

                            if ephemeral {
                                expansion = Vec::new();
                            } else {
                                expansion = m.expand(
                                    file,
                                    file.tokens[file.working_index].origin_file,
                                    file.tokens[file.working_index].line_number,
                                    context,
                                    &args,
                                    &block,
                                );
                            }
                        } else {
                            block_tokcount = 0;

                            if ephemeral {
                                expansion = Vec::new();
                            } else {
                                expansion = m.expand(
                                    file,
                                    file.tokens[file.working_index].origin_file,
                                    file.tokens[file.working_index].line_number,
                                    context,
                                    &args,
                                    &Vec::new()[..],
                                );
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

                // check for templates
                // todo maybe deduplicate this
                for m in &mut file.templates {
                    if &symbol[prefix_len..] == m.symbol {
                        matched_macro = true;
                        //println!("Found a macro ({})", m.symbol);

                        let (args, args_tokcount) =
                            collect_arguments(&file.tokens[file.working_index..]);
                        let expansion: Vec<Token>;
                        let block_tokcount: usize;

                        if m.has_scope {
                            //println!("is scoped.");
                            let block: Vec<Token>;
                            let block_opt =
                                collect_block(&file.tokens[(file.working_index + args_tokcount)..]);
                            if block_opt.is_none() {
                                error_skid(
                                    context,
                                    file.tokens[file.working_index].template_origin,
                                    file.tokens[file.working_index].line_number,
                                    &"Malformed Block".into(),
                                );
                            }

                            (block, block_tokcount) = block_opt.unwrap();

                            if ephemeral {
                                expansion = Vec::new();
                            } else {
                                expansion = m.expand(
                                    //file,
                                    file.tokens[file.working_index].origin_file,
                                    file.tokens[file.working_index].line_number,
                                    context,
                                    &args,
                                    &block,
                                );
                            }
                        } else {
                            block_tokcount = 0;

                            if ephemeral {
                                expansion = Vec::new();
                            } else {
                                expansion = m.expand(
                                    //file,
                                    file.tokens[file.working_index].origin_file,
                                    file.tokens[file.working_index].line_number,
                                    context,
                                    &args,
                                    &Vec::new()[..],
                                );
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
            }
            if !matched_macro {
                warn_skid(
                    context,
                    file.tokens[file.working_index].origin_file,
                    file.tokens[file.working_index].line_number,
                    &format!(
                        "Token written as a function but no such function exists \"{}\"",
                        file.tokens[file.working_index].contents.trim()
                    ),
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

    let mut folder = file.file_skidout.clone();
    folder.pop();
    if fs::create_dir_all(&folder).is_err() {
        error_generic(&format!("Could not make the folder {:?}", &folder));
    }

    if convert_html {
        fs::write(&file.file_skidout, &skid_output).expect("Couldn't write skid to file");

        //let html_output = markdown::to_html(&skid_output);
        let html_output = markdown::to_html_with_options(
            &skid_output,
            &Options {
                compile: CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,
                    gfm_tagfilter: false,
                    // gfm_footnote_clobber_prefix:
                    gfm_task_list_item_checkable: true,
                    allow_any_img_src: true,
                    ..CompileOptions::gfm()
                },
                parse: ParseOptions {
                    constructs: Constructs {
                        code_indented: false,

                        //html_flow: false,
                        ..Constructs::gfm()
                    },
                    ..ParseOptions::default()
                },
            },
        )
        .unwrap();
        fs::write(&file.file_out, &html_output).expect("Couldn't write output to file");
    } else {
        fs::write(&file.file_out, &skid_output).expect("Couldn't write output to file");
    }
    ok_generic(&format!(
        "{} written \n\n",
        file.file_out
            .to_str()
            .unwrap_or("Couldnt Unwrap file_out name")
    ));
}
