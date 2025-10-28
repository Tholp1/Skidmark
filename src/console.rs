use std::process::exit;

use colored::Colorize;

use crate::project::{Indexing, Project};

pub fn error_generic(msg: &String) {
    println!("{} {}", "[ERROR]".red(), msg);
    exit(1);
}

pub fn error_skid(proj_context: &Project, origin_index: usize, origin_line: usize, msg: &String) {
    println!(
        "{} {}:{}; {}",
        "[ERROR]".red(),
        proj_context
            .file_for_index(origin_index)
            .expect("Panic in the error func.... (file_for_index() was None!)")
            .into_os_string()
            .into_string()
            .unwrap(),
        origin_line,
        msg
    );
    exit(1);
}

pub fn warn_generic(msg: &String) {
    println!("{} {}", "[WARN]".yellow(), msg);
}

pub fn warn_skid(proj_context: &Project, origin_index: usize, origin_line: usize, msg: &String) {
    println!(
        "{} {}:{}; {}",
        "[WARN]".yellow(),
        proj_context
            .file_for_index(origin_index)
            .expect("Panic in the warn func.... (file_for_index() was None!)")
            .into_os_string()
            .into_string()
            .unwrap(),
        origin_line,
        msg
    );
}

pub fn ok_generic(msg: &String) {
    println!("{} {}", "[OK]".green(), msg);
}

pub fn reminder_skid(
    proj_context: &Project,
    origin_index: usize,
    origin_line: usize,
    msg: &String,
) {
    println!(
        "{} {}:{}; {}",
        "[REMINDER]".cyan(),
        proj_context
            .file_for_index(origin_index)
            .expect("Panic in the warn func.... (file_for_index() was None!)")
            .into_os_string()
            .into_string()
            .unwrap(),
        origin_line,
        msg
    );
}

pub fn info_generic(msg: &String) {
    println!("{} {}", "[INFO]".purple(), msg);
}
