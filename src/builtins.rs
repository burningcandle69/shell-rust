/// This module contains the builtin commands supported
/// by our shell and the execution logic

use crate::command::Command;

impl Command {
    pub fn execute(&self) -> Result<usize, String> {
        match self.name.as_str() {
            "exit" => self.exit(),
            "echo" => self.echo(),
            _ => Err(format!("{}: command not found", self.name))
        }
    }
    
    fn exit(&self) -> Result<usize, String> {
        if self.args.len() < 2 {
            return Err("Usage: exit <exit_code>".into());
        }
        let exit_code = self.args[1].parse().map_err(|_| "invalid error code")?;
        std::process::exit(exit_code)
    }
    
    fn echo(&self) -> Result<usize, String> {
        println!("{}", self.args[1..].join(" "));
        Ok(0)
    }
}