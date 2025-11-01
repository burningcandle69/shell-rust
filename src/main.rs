#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 
        input = input.trim().into();
        println!("{input}: command not found");
    }
}
