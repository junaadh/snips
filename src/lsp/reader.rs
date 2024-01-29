use std::path::Path;

use crate::lsp::error::SnipError;

pub fn read(path: String) -> String {
    let path = Path::new(&path);

    match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            std::process::exit(SnipError::SnipUnableToReadFile as i32);
        }
    }
}
