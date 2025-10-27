mod args;
mod closures;
mod console;
mod macros;
mod project;
mod reservednames;
mod stringtools;
mod types;

use crate::{
    reservednames::RESERVED_NAMES_MISC,
    stringtools::TokenTools,
    types::{IsScoped, MacroExpand, SkidContext},
};

use console::*;
use macros::MACRO_LIST;
use markdown::{CompileOptions, Constructs, Options, ParseOptions};
use project::{parse_project, Indexing, Project};
use reservednames::RESERVED_NAMES_HTML;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};
use stringtools::{collect_arguments, collect_block, split_to_tokens};
use types::{InputFile, Token};

// really need to change this whole thing to work with characters rather than
// strings split on kind of abitrary chars..
static DELIMITERS: &'static [char] = &[
    ' ', '\n', '\t', '(', ')', '{', '}', '[', ']', '<', '>', '\\', '\'', '\"', ';', '?', '^', '-',
    '`',
];

#[derive(PartialEq)]
enum EphemeralType {
    Normal,
    Ephemeral,
    InverseEphemeral,
}

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

    for i in 0..project.filegroups.len() {
        if !project.filegroups[i].process {
            continue;
        }
        let convert_html = project.filegroups[i].convert_html;
        for k in 0..project.filegroups[i].files.len() {
            let file_input = project.filegroups[i].files[k].file_input.clone();
            let contents = fs::read_to_string(&project.filegroups[i].files[k].file_input)
                .expect("File unreadable or missing");
            let tokens = split_to_tokens(contents, project.index_of_file(&file_input));

            let mut skid_context = SkidContext::new(project.index_of_file(&file_input));
            write_file(
                &project.filegroups[i].files[k].file_skidout.clone(),
                &project.filegroups[i].files[k].file_out.clone(),
                convert_html,
                &process_skid(&tokens, &mut project, &mut skid_context),
            );
        }
    }
}

fn find_and_run_macro(
    tokens_in: &[Token],
    proj_context: &mut Project,
    skid_context: &mut SkidContext,
) -> Option<(Vec<Token>, usize)> {
    // (Output, to be consumed size)

    // At this point we think its a macro (starts with ! or &) so check which, we have the rest of the file
    let ephemeral_type: EphemeralType;
    if tokens_in.len() < 2 {
        return None;
    }

    if tokens_in[0] == '!' && tokens_in[1] == '&' {
        ephemeral_type = EphemeralType::InverseEphemeral;
    } else if tokens_in[0] == '!' {
        ephemeral_type = EphemeralType::Normal;
    } else if tokens_in[0] == '&' {
        ephemeral_type = EphemeralType::Ephemeral;
    } else {
        return None;
    }

    let mut chars_consumed = if ephemeral_type == EphemeralType::InverseEphemeral {
        2
    } else {
        1
    };
    // Look for name
    let mut symbol: String = "".into();
    for tok in &tokens_in[chars_consumed..] {
        if tok.contents.is_whitespace() || DELIMITERS.contains(&tok.contents) {
            break;
        }
        symbol.push(tok.contents);
        chars_consumed += 1;
    }

    if symbol.is_empty() {
        return None;
    }

    let args;
    let block;

    {
        let mut expander: &dyn IsScoped = &MACRO_LIST[0]; // assinging because it complains about possibly being empty later even if not the case
        let mut found = false;
        // Check if its a macro
        for m in MACRO_LIST {
            if m.symbol == symbol {
                found = true;
                expander = m;
                break;
            }
        }

        // Not a macro check templates
        if !found {
            for t in &skid_context.templates {
                if t.symbol == symbol {
                    found = true;
                    expander = t;
                    break;
                }
            }
        }

        // Not a template either, see if its reserved or not to see if we should say something
        if !found {
            let name = symbol.to_lowercase();
            let mut dont_error = false;

            for reserved in RESERVED_NAMES_HTML {
                if name.starts_with(reserved) {
                    dont_error = true;
                    break;
                }
            }

            if !dont_error {
                for reserved in RESERVED_NAMES_MISC {
                    if name.starts_with(reserved) {
                        dont_error = true;
                        break;
                    }
                }
            }

            if !dont_error {
                warn_skid(
                    &proj_context,
                    tokens_in[0].origin_index,
                    tokens_in[0].origin_line,
                    &format!("No such macro or defined template \"{symbol}\""),
                );
            }
            return None;
        }

        let args_result = collect_arguments(&tokens_in[chars_consumed..]);
        if args_result.is_none() {
            error_skid(
                proj_context,
                tokens_in[0].origin_index,
                tokens_in[0].origin_line,
                &format!("Didnt find any arguments for macro \"{symbol}\"."),
            );
            return None;
        }

        let consumed_by_args;
        (args, consumed_by_args) = args_result.unwrap();
        chars_consumed += consumed_by_args;

        if expander.is_scoped() {
            let block_result = collect_block(&tokens_in[chars_consumed..]);
            if block_result.is_none() {
                error_skid(
                    proj_context,
                    tokens_in[0].origin_index,
                    tokens_in[0].origin_line,
                    &format!("Didnt find a block for macro \"{symbol}\"."),
                );
                return None;
            }
            let consumed_by_block;
            (block, consumed_by_block) = block_result.unwrap();
            chars_consumed += consumed_by_block;
        } else {
            block = Vec::new();
        }
    }

    let return_empty: bool;

    match ephemeral_type {
        EphemeralType::Normal => return_empty = false,
        EphemeralType::Ephemeral => {
            return_empty = skid_context.file_index != tokens_in[0].origin_index
        }
        EphemeralType::InverseEphemeral => {
            return_empty = skid_context.file_index == tokens_in[0].origin_index
        }
    }

    if return_empty {
        return Some((Vec::new(), chars_consumed));
    } else {
        // we have to find it again because of borrower
        for m in MACRO_LIST {
            if m.symbol == symbol {
                return Some((
                    m.expand(
                        tokens_in[0].origin_index,
                        tokens_in[0].origin_line,
                        proj_context,
                        skid_context,
                        &args,
                        &block,
                    )
                    .trim_whitespace()
                    .to_vec(),
                    chars_consumed,
                ));
            }
        }
        let mut i = 0;
        while i < skid_context.templates.len() {
            if skid_context.templates[i].symbol == symbol {
                return Some((
                    skid_context.templates[i]
                        .expand(
                            tokens_in[0].origin_index,
                            tokens_in[0].origin_line,
                            proj_context,
                            &args,
                            &block,
                        )
                        .trim_whitespace()
                        .to_vec(),
                    chars_consumed,
                ));
            }
            i += 1;
        }
    }
    None
}

fn process_skid(
    tokens_in: &[Token],
    proj_context: &mut Project,
    skid_context: &mut SkidContext,
) -> Vec<Token> {
    //}, context: &mut ProjectContext) {
    //println!("{}\n {}", f.filename_out, contents);

    //file.tokens = strings_to_tokens(split_keep_delimiters(contents), file.filename_input.clone());

    //let mut escaped = false;
    let mut tokens = tokens_in.to_vec();
    let starting_template_count = skid_context.templates.len();

    let mut escaped = false;
    let mut working_index = 0;

    while working_index < tokens.len() {
        if tokens[working_index] == '\\' && !escaped {
            tokens[working_index].contents = '\0'; // skip over this later when outputting to avoid shifting memory rn
            escaped = true;
            working_index += 1;

            // bit of a hack for reverse ephemeral escaping behavior to be the same as previously
            if tokens.len() > working_index + 1
                && tokens[working_index] == '!'
                && tokens[working_index + 1] == '&'
            {
                working_index += 1;
            }
            continue;
        }

        if (tokens[working_index] == '!' || tokens[working_index] == '&') && !escaped {
            let expansion =
                find_and_run_macro(&tokens[working_index..], proj_context, skid_context);
            if expansion.is_some() {
                tokens.splice(
                    working_index..working_index + expansion.as_ref().unwrap().1,
                    expansion.unwrap().0,
                );
                continue;
            }
        }

        // Not a macro or template, look through our closures
        // for c in CLOSURE_LIST
        // {
        //     if tokens[working_index].contents.starts_with(c.opener)
        //     {

        //     }
        // }

        working_index += 1;

        escaped = false;
    }
    skid_context.templates.truncate(starting_template_count);

    tokens.retain(|t| t.contents != '\0');

    return tokens;
}

fn write_file(file_skidout: &PathBuf, file_out: &PathBuf, convert_html: bool, tokens: &[Token]) {
    //println!("{:?}", tokens);
    let mut skid_output: String = if convert_html {
        "<!-- Generated by Skidmark, Do Not Edit! -->\n\n".into()
    } else {
        "".into()
    };
    for t in tokens {
        skid_output.push(t.contents);
    }

    let mut folder = file_skidout.clone();
    folder.pop();
    if fs::create_dir_all(&folder).is_err() {
        error_generic(&format!("Could not make the folder {:?}", &folder));
    }

    if convert_html {
        fs::write(&file_skidout, &skid_output).expect("Couldn't write skid to file");

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
        fs::write(&file_out, &html_output).expect("Couldn't write output to file");
    } else {
        fs::write(&file_out, &skid_output).expect("Couldn't write output to file");
    }
    ok_generic(&format!(
        "{} written",
        file_out.to_str().unwrap_or("Couldnt Unwrap file_out name")
    ));
}
