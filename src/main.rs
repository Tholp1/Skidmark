mod args;
mod closures;
mod console;
mod macros;
mod project;
mod reservednames;
mod stringtools;
mod types;

use crate::{
    args::ProgramArgs,
    closures::CLOSURE_LIST,
    macros::template::SkidTemplate,
    project::FileGroup,
    reservednames::RESERVED_NAMES_MISC,
    types::{Expand, SkidContext},
};
use clap::Parser;
use console::*;
use macros::MACRO_LIST;
use markdown::{CompileOptions, Constructs, Options, ParseOptions};
use project::{parse_project, Indexing, ProjectContext};
use reservednames::RESERVED_NAMES_HTML;
use std::{
    env,
    fs::{self},
    path::PathBuf,
    task::Context,
};
use stringtools::{collect_arguments, collect_block, split_to_tokens, trim_whitespace_tokens};
use types::{InputFile, Token};

// really need to change this whole thing to work with characters rather than
// strings split on kind of abitrary chars..
static DELIMITERS: &'static [char] = &[
    ' ', '\n', '\t', '(', ')', '{', '}', '[', ']', '<', '>', '\\', '\'', '\"', ';', '?', '^', '-',
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
    println!("Operatting on {:?}", &project_path.as_os_str());
    assert!(env::set_current_dir(&project_folder).is_ok());

    let mut project = parse_project(&project_path);

    let mut num = 0;

    for group in &project.filegroups {
        num = num + group.files.len();
    }

    println!("Proccesing {} files.", num);
    // for group in &mut project.filegroups {
    //     for infile in &mut group.files {
    //         process_skid(infile, group.convert_html, &mut project.context);
    //     }
    // }

    for group in &mut project.filegroups {
        for infile in &mut group.files {
            let contents =
                fs::read_to_string(&infile.file_input).expect("File unreadable or missing");
            infile.tokens =
                split_to_tokens(contents, project.context.index_of_file(&infile.file_input));

            let mut skid_context = SkidContext::new();
            process_skid(
                &mut infile.tokens,
                project.context.index_of_file(&infile.file_input),
                &mut project.context,
                &mut skid_context,
            );
        }
    }
}

fn process_skid(
    tokens_in: &mut [Token],
    file_index: usize,
    context: &mut ProjectContext,
    skid_context: &mut SkidContext,
) -> Vec<Token> {
    //}, context: &mut ProjectContext) {
    //println!("{}\n {}", f.filename_out, contents);

    //file.tokens = strings_to_tokens(split_keep_delimiters(contents), file.filename_input.clone());

    //let mut escaped = false;
    let mut tokens = tokens_in.to_vec();
    let mut starting_template_count = skid_context.templates.len();

    let mut working_index = 0;

    while working_index < tokens.len() {
        //look for macros or blocks
        //println!(">\"{}\"<", tokens[working_index].contents);

        if tokens[working_index].contents.len() == 0 {
            working_index += 1;
            continue;
        }

        if tokens[working_index].contents == "\\" {
            tokens[working_index].contents = "".into();
            working_index += 2;
            //println!("Hit backslash");
            continue;
        }

        let mut matched_macro: bool = false;
        if tokens[working_index].contents.starts_with(['!', '&']) {
            let mut prefix_len = 1;
            let mut symbol = tokens[working_index].contents.clone();
            symbol = symbol.trim().to_string();

            if symbol.len() > 2 {
                let mut ephemeral = false;
                let same_file = tokens[working_index].origin_file != file_index;

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

                        let (args, args_tokcount) = collect_arguments(&tokens[working_index..]);
                        let expansion: Vec<Token>;
                        let block_tokcount: usize;
                        if m.has_scope {
                            //println!("is scoped.");

                            let block_opt =
                                collect_block(&tokens[(working_index + args_tokcount)..]);
                            if block_opt.is_none() {
                                error_skid(
                                    context,
                                    tokens[working_index].template_origin,
                                    tokens[working_index].line_number,
                                    &"Malformed Block".into(),
                                );
                            }
                            let block: Vec<Token>;
                            (block, block_tokcount) = block_opt.unwrap();

                            if ephemeral {
                                expansion = Vec::new();
                            } else {
                                expansion = m.expand(
                                    tokens[working_index].origin_file,
                                    tokens[working_index].line_number,
                                    context,
                                    skid_context,
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
                                    tokens[working_index].origin_file,
                                    tokens[working_index].line_number,
                                    context,
                                    skid_context,
                                    &args,
                                    &Vec::new()[..],
                                );
                            }
                        }

                        let trimmed = trim_whitespace_tokens(&expansion);

                        tokens.remove(working_index);
                        tokens.splice(
                            working_index..(working_index + args_tokcount + block_tokcount - 1),
                            trimmed.iter().cloned(),
                        );
                        if expansion.len() == 0 && working_index > 0 {
                            working_index -= 1;
                        }
                    }
                }

                // check for templates
                // todo maybe deduplicate this
                for t in &skid_context.templates {
                    if &symbol[prefix_len..] == t.symbol {
                        matched_macro = true;
                        //println!("Found a macro ({})", m.symbol);

                        let (args, args_tokcount) = collect_arguments(&tokens[working_index..]);
                        let expansion: Vec<Token>;
                        let block_tokcount: usize;

                        if t.has_scope {
                            //println!("is scoped.");
                            let block: Vec<Token>;
                            let block_opt =
                                collect_block(&tokens[(working_index + args_tokcount)..]);
                            if block_opt.is_none() {
                                error_skid(
                                    context,
                                    tokens[working_index].template_origin,
                                    tokens[working_index].line_number,
                                    &"Malformed Block".into(),
                                );
                            }

                            (block, block_tokcount) = block_opt.unwrap();

                            if ephemeral {
                                expansion = Vec::new();
                            } else {
                                expansion = t.expand(
                                    //file,
                                    tokens[working_index].origin_file,
                                    tokens[working_index].line_number,
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
                                expansion = t.expand(
                                    //file,
                                    tokens[working_index].origin_file,
                                    tokens[working_index].line_number,
                                    context,
                                    &args,
                                    &Vec::new()[..],
                                );
                            }
                        }

                        let trimmed = trim_whitespace_tokens(&expansion);

                        tokens.remove(working_index);
                        tokens.splice(
                            working_index..(working_index + args_tokcount + block_tokcount - 1),
                            trimmed.iter().cloned(),
                        );
                        if expansion.len() == 0 && working_index > 0 {
                            working_index -= 1;
                        }
                    }
                }
            }
            if !matched_macro {
                let name = tokens[working_index].contents.trim().to_lowercase();
                let mut dont_error = name.len() <= 1;
                {
                    if !dont_error {
                        for reserved in RESERVED_NAMES_HTML {
                            if name[1..].starts_with(reserved) {
                                dont_error = true;
                                break;
                            }
                        }
                    }

                    if !dont_error {
                        for reserved in RESERVED_NAMES_MISC {
                            if name[1..].starts_with(reserved) {
                                dont_error = true;
                                break;
                            }
                        }
                    }
                }
                if !dont_error {
                    warn_skid(
                        context,
                        tokens[working_index].origin_file,
                        tokens[working_index].line_number,
                        &format!(
                            "Token written as a function but no such function exists \"{}\"",
                            tokens[working_index].contents.trim()
                        ),
                    );
                }
            }
        }

        // Not a macro or template, look through our closures
        // for c in CLOSURE_LIST
        // {
        //     if tokens[working_index].contents.starts_with(c.opener)
        //     {

        //     }
        // }

        if !matched_macro {
            working_index += 1;
        }
    }
    skid_context.templates.truncate(starting_template_count);
    return tokens;
}

fn write_file(file: InputFile, convert_html: bool) {
    //println!("{:?}", tokens);
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
        "{} written",
        file.file_out
            .to_str()
            .unwrap_or("Couldnt Unwrap file_out name")
    ));
}
