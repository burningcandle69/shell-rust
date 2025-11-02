mod autocomplete;
mod builtins;
mod command;
mod shell;
mod trie;

use crate::autocomplete::ShellAutocomplete;
use crate::shell::Shell;
use rustyline::completion::{Completer, Pair};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut shell = Shell::new();
    let autocomplete = ShellAutocomplete::new(&shell.path);

    let mut rl = rustyline::Editor::new().unwrap();
    rl.set_completion_type(rustyline::CompletionType::List);
    rl.set_auto_add_history(true);
    rl.set_helper(Some(autocomplete));
    shell.read_history("history.txt");
    let _ = rl.load_history("history.txt");

    loop {
        let input = rl.readline("$ ");
        match input {
            Ok(line) => {
                shell.execute(line).err().and_then(|e| {
                    println!("{e}");
                    Option::<String>::None
                });
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
