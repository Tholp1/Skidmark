use crate::{console::*, project::Project, stringtools::*, types::*};

fn for_each_base(
    identifier: &String,
    replacements: &[String],
    proj_context: &mut Project,
    origin_index: usize,
    origin_line: usize,
    scope: &[Token],
) -> Vec<Token> {
    let mut output = Vec::new();
    let block: Vec<Token> = scope.into();

    let mut replacement_count: usize = 0;

    let mut replacement_pattern = find_pattern(scope, format!("[[{}..1]]", identifier));

    if replacement_pattern.is_none() {
        warn_skid(
            proj_context,
            origin_index,
            origin_line,
            &format!(
                "Macro `for_each_arg` given block with no \"[[{}..1]]\", intentional?",
                identifier
            ),
        );
    }

    while replacement_pattern.is_some() {
        replacement_count += 1;
        replacement_pattern = find_pattern(
            scope,
            format!("[[{}..{}]]", identifier, replacement_count + 1),
        );
    }

    if replacement_count == 0 {
        for _i in 0..replacements.iter().count() {
            output.append(&mut block.clone());
        }
        return output;
    }

    if replacements.len() % replacement_count != 0 {
        error_skid(proj_context, origin_index, origin_line,
            &format!("`for_each_var` was not given a number of arguments({}) that was a multiple of its replacement posistions({}) (got {:?})",
            replacements.len(),
            replacement_count,
            replacements));
    }

    let mut replacement_index: usize = 0;
    let mut arg_output: Vec<Token> = block.clone();
    for r in replacements {
        let mut found_pattern = find_pattern(
            &arg_output,
            format!("[[{}..{}]]", identifier, replacement_index + 1),
        );

        while found_pattern.is_some() {
            let (start, len) = found_pattern.unwrap();
            let replacement = split_to_tokens(r.clone(), origin_index);
            arg_output.splice(start..start + len, replacement);
            found_pattern = find_pattern(
                &arg_output,
                format!("[[{}..{}]]", identifier, replacement_index + 1),
            );
            //println!("{}", replacement_index + 1);
        }

        //println!("{} {}", replacement_index, replacement_count);
        replacement_index += 1;
        if replacement_index == replacement_count {
            replacement_index = 0;
            output.append(&mut arg_output.trim_whitespace().into());
            arg_output = block.clone();
            //println!("push");
        }
        //println!("test");
    }
    return output;
}

pub fn macro_for_each_arg(
    origin_index: usize,
    origin_line: usize,
    proj_context: &mut Project,
    _skid_context: &mut SkidContext,
    args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    return for_each_base(
        &args[0],
        &args[1..],
        proj_context,
        origin_index,
        origin_line,
        scope,
    );
}

pub fn macro_for_each_file_in_group(
    origin_index: usize,
    origin_line: usize,
    proj_context: &mut Project,
    _skid_context: &mut SkidContext,
    args: &Vec<String>,
    scope: &[Token],
) -> Vec<Token> {
    let mut files: Vec<String> = Vec::new();
    for g in &proj_context.filegroups {
        if g.name == args[1] {
            for f in &g.files {
                let path = f
                    .file_input
                    .strip_prefix(&proj_context.input_folder)
                    .unwrap();
                files.push(path.to_str().unwrap().into());
            }
        }
    }
    return for_each_base(
        &args[0],
        &files,
        proj_context,
        origin_index,
        origin_line,
        scope,
    );
}
