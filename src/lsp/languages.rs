const DEF_LANGUAGES: &[&str] = &["c", "bash", "react"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language<'a> {
    pub name: &'a str,
    pub json_path: String,
}

impl<'a> Language<'a> {
    fn expand_path() -> String {
        let home = "HOME";
        std::env::var(home)
            .map_err(|error| {
                println!("ERROR: unable to expand home {error}");
                std::process::exit(1);
            })
            .unwrap()
    }

    fn get_path() -> String {
        Self::expand_path() + "/.config/snips/"
    }

    fn new(name: &'a str) -> Self {
        Self {
            name,
            json_path: format!("{}{}{}", Self::get_path(), name, ".json"),
        }
    }

    pub fn get_language(name: &'a str) -> Result<Self, ()> {
        let name = name.trim();
        for language in DEF_LANGUAGES {
            if language.contains(name) {
                return Ok(Self::new(name));
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {

    use super::Language;

    #[test]
    fn test_basic() {
        let result = Language::get_language("c");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            Language {
                name: "c",
                json_path: format!("{}{}", Language::expand_path(), "/.config/snips/c.json")
            }
        );
    }

    #[test]
    fn test_undefined() {
        let result = Language::get_language("testing");
        assert!(result.is_err());
    }

    #[test]
    fn test_path_expansion() {
        let result = Language::expand_path();
        assert_eq!(result, std::env::var("HOME").unwrap());
    }

    #[test]
    fn test_get_path() {
        let test = Language::new("test");
        let home = Language::expand_path();
        let path = home + "/.config/snips/" + test.name + ".json";
        assert_eq!(test.json_path, path);
    }
}
