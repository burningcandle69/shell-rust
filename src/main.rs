mod autocomplete;
mod builtins;
mod command;
mod shell;
mod shell_io;
mod trie;

use crate::autocomplete::ShellAutocomplete;
use crate::shell::Shell;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use std::io::{self};

fn main() -> io::Result<()> {
    let mut shell = Shell::new();
    let autocomplete = ShellAutocomplete::new(&shell.path);

    let mut rl = rustyline::Editor::new().unwrap();
    rl.set_completion_type(rustyline::CompletionType::List);
    rl.set_auto_add_history(true);
    rl.set_helper(Some(autocomplete));
    let _ = shell.read_history(shell.hist_file.clone());
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
