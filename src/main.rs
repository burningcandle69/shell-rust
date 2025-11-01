mod builtins;
mod command;
mod shell;

use crate::shell::Shell;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut shell = Shell::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        shell.execute(input).err().and_then(|e| {
            println!("{e}");
            Option::<String>::None
        });
    }
}
