use std::io;
use std::io::Write;
use crate::trie::Trie;
use inquire::autocompletion::Replacement;
use inquire::{Autocomplete, CustomUserError};
use is_executable::is_executable;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ShellAutocomplete {
    pub suggestions: Trie,
    show_suggestions: bool
}

impl ShellAutocomplete {
    pub fn new(path: &Vec<PathBuf>) -> Self {
        let mut res = Trie::new();

        vec!["exit", "echo", "type", "pwd"]
            .into_iter()
            .for_each(|x| res.add(x.chars()));

        for p in path {
            let entries = match p.read_dir() {
                Ok(r) => r,
                Err(_) => continue,
            };

            for entry in entries.flatten() {
                if !is_executable(entry.path()) {
                    continue;
                }

                let name = entry.file_name();
                let name = match name.to_str() {
                    Some(n) => n,
                    None => continue,
                };

                res.add(name.chars());
            }
        }
        ShellAutocomplete { suggestions: res, show_suggestions: false }
    }
}

impl Autocomplete for ShellAutocomplete {
    fn get_suggestions(&mut self, _: &str) -> Result<Vec<String>, CustomUserError> {
        Ok(vec![])
    }
    
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        if highlighted_suggestion.is_some() {
            return Ok(highlighted_suggestion);
        }

        let fs = self.suggestions.fuzzy(input.chars());
        if fs.is_empty() {
            print!("\x07");
            let _ = io::stdout().flush();
            Ok(None)
        } else if fs.len() == 1 {
            Ok(Some(fs[0].clone() + " "))
        } else {
            if !self.show_suggestions {
                self.show_suggestions = true;
            } else {
                print!("\r\n{}\r\n", fs.join("  "));
                print!("$ {}", input);
                let _ = io::stdout().flush();
                self.show_suggestions = false;
            }
            Ok(None)
        }
    }
}
