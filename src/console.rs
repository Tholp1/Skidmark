use std::process::exit;

use colored::Colorize;

use crate::project::{FileIndexing, ProjectContext};

pub fn error_generic(msg: &String) {
    println!("{} {}", "[ERROR]".red(), msg);
    exit(1);
}

pub fn error_skid(context: &ProjectContext, origin_index: usize, origin_line: usize, msg: &String) {
    println!(
        "{} {:?}:{}; {}",
        "[ERROR]".red(),
        context
            .file_for_index(origin_index)
            .expect("Panic in the error func.... (file_for_index() was None!)"),
        origin_line,
        msg
    );
    exit(1);
}

pub fn warn_generic(msg: &String) {
    println!("{} {}", "[WARN]".yellow(), msg);
}

pub fn warn_skid(context: &ProjectContext, origin_index: usize, origin_line: usize, msg: &String) {
    println!(
        "{} {:?}:{}; {}",
        "[WARN]".yellow(),
        context
            .file_for_index(origin_index)
            .expect("Panic in the warn func.... (file_for_index() was None!)"),
        origin_line,
        msg
    );
}

pub fn ok_generic(msg: &String) {
    println!("{} {}", "[OK]".green(), msg);
}

pub fn reminder_skid(
    context: &ProjectContext,
    origin_index: usize,
    origin_line: usize,
    msg: &String,
) {
    println!(
        "{} {:?}:{}; {}",
        "[REMINDER]".cyan(),
        context
            .file_for_index(origin_index)
            .expect("Panic in the warn func.... (file_for_index() was None!)"),
        origin_line,
        msg
    );
}
