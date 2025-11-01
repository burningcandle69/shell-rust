mod command;
mod builtins;
mod shell;

#[allow(unused_imports)]
use std::io::{self, Write};
use crate::shell::Shell;

fn main() -> io::Result<()> {
    let mut shell = Shell::default();
    
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
