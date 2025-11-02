use crate::trie::Trie;
use is_executable::is_executable;
use rustyline::completion::{Completer, Pair};
use rustyline::{Context, Helper};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

#[derive(Clone)]
pub struct ShellAutocomplete {
    pub suggestions: Trie,
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
        ShellAutocomplete {
            suggestions: res,
        }
    }
}

impl Completer for ShellAutocomplete {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _: usize,
        _: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut fs = self.suggestions.fuzzy(line.chars());
        fs.sort();

        if fs.is_empty() {
            print!("\x07");
            let _ = io::stdout().flush();
            Ok((0, vec![]))
        } else if fs.len() == 1 {
            let r = fs[0].clone() + " ";
            Ok((
                0,
                vec![Pair {
                    display: r.clone(),
                    replacement: r.clone(),
                }],
            ))
        } else {
            Ok((
                0,
                fs.into_iter()
                    .map(|f| Pair {
                        display: f.clone(),
                        replacement: f.clone(),
                    })
                    .collect(),
            ))
        }
    }
}

impl Helper for ShellAutocomplete {}
impl Highlighter for ShellAutocomplete {}
impl Hinter for ShellAutocomplete {
    type Hint = String;

    fn hint(&self, _: &str, _: usize, _: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Validator for ShellAutocomplete {}

