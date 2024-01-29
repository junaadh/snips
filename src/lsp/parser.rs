use std::process;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::error::SnipError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Snippet {
    pub name: String,
    pub body: String,
}

pub fn parse_json(contents: String) -> Vec<Snippet> {
    match serde_json::from_str::<Map<String, Value>>(&contents) {
        Ok(json_map) => match json_map.get("snippets") {
            Some(snippets) => match serde_json::from_value::<Vec<Snippet>>(snippets.clone()) {
                Ok(snippet) => snippet,
                Err(_) => {
                    process::exit(SnipError::SnipFailParseSnippet as i32);
                }
            },
            None => {
                process::exit(SnipError::SnipJsonKeyNotFound as i32);
            }
        },
        Err(_) => {
            process::exit(SnipError::SnipFailParseJson as i32);
        }
    }
}

fn to_upper_first(word: &str) -> String {
    let word = word.to_lowercase();
    let mut chars: Vec<char> = word.chars().collect();
    if let Some(first) = chars.first_mut() {
        *first = first.to_ascii_uppercase();
    }
    chars.into_iter().collect()
}

fn to_upper_offset(word: &str, offset: usize) -> String {
    let word = word.to_lowercase();
    let mut chars: Vec<char> = word.chars().collect();
    if let Some(character) = chars.get_mut(offset) {
        *character = character.to_ascii_uppercase();
    }
    chars.into_iter().collect()
}

fn substitute(source: String, term: &str, value: &str) -> String {
    if !source.contains(term) {
        return source;
    }

    source.replace(term, value)
}

pub fn format(format_str: String, inputs: &[String]) -> String {
    let mut result = format_str.to_string();
    for (i, input) in inputs.iter().enumerate() {
        let search_buf = format!("#term{}", i + 1);
        let in1 = input.to_lowercase();
        result = substitute(result, &search_buf, &in1);
        let search_buf = to_upper_offset(&search_buf, 1);
        let in2 = to_upper_first(input);
        result = substitute(result, &search_buf, &in2);
        let search_buf = search_buf.to_uppercase();
        let in3 = input.to_uppercase();
        result = substitute(result, &search_buf, &in3);
    }
    result
        .replace("term", "")
        .replace("Term", "")
        .replace("TERM", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_first() {
        let term = "rust";
        assert_eq!(to_upper_first(term), "Rust");
    }

    #[test]
    fn test_upper_offset() {
        let term = "rust";
        assert_eq!(to_upper_offset(term, 1), "rUst");
    }

    #[test]
    fn test_substitute() {
        let string = "how is you?".to_string();
        let result = substitute(string, "is", "are");
        assert_eq!(result, "how are you?");
    }

    #[test]
    fn test_lower_case_substitute() {
        let string = "#term1, #term1, #term1".to_string();
        let result = substitute(string, "#term1", "HELLO".to_lowercase().as_str());
        assert_eq!(result, "hello, hello, hello".to_string());
    }
    #[test]
    fn test_first_upper_case_substitute() {
        let string = "#Term1, #Term1, #Term1".to_string();
        let result = substitute(string, "#Term1", &to_upper_first("HELLO"));
        assert_eq!(result, "Hello, Hello, Hello".to_string());
    }
    #[test]
    fn test_upper_case_substitute() {
        let string = "#TERM1, #TERM1, #TERM1".to_string();
        let result = substitute(string, "#TERM1", "HELLO".to_uppercase().as_str());
        assert_eq!(result, "HELLO, HELLO, HELLO".to_string());
    }

    #[test]
    fn test_format() {
        let inputs = &["rust".to_string(), "language".to_string()];
        let string = "#Term1, is a systems #term2. #TERM1, #Term2".to_string();
        let result = format(string, inputs);
        assert_eq!(
            result,
            "Rust, is a systems language. RUST, Language".to_string()
        );
    }
}
