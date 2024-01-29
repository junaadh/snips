use std::process;

use self::{
    error::SnipError,
    languages::Language,
    parser::{format, parse_json},
    reader::read,
};

pub mod error;
pub mod languages;
pub mod parser;
pub mod reader;

pub fn run(input: &[String]) {
    let language = &input[0].to_lowercase();
    let snippet_name = &input[1];
    let values = &input[2..];

    let language = Language::get_language(language)
        .map_err(|_| process::exit(SnipError::SnipTooFewArguments as i32))
        .unwrap();
    let contents = read(language.json_path);
    let json_content = parse_json(contents);

    for snippet in json_content {
        if snippet.name.eq_ignore_ascii_case(snippet_name) {
            print!("{}", format(snippet.body, values));
        }
    }
    process::exit(SnipError::SnipNotFound as i32);
}

pub fn help() {
    let description = "snips - mock lsp completion to stdout\n\n";
    let command = "usage:\n\tsnips {language} {snippet_name} [values...]\n\t*snips can accept anynumber of values any values without placeholder in json bidy would be ignored\n";
    let error = "error:\n\terror codes correspond to errors.rs. See for more information\n";
    println!("{}{}{}", description, command, error);
}
