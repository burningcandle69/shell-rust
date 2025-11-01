/// This module contains the builtin commands supported
/// by our shell and the execution logic
use crate::command::{Command, CommandType};

impl Command {
    pub fn execute(&self) -> Result<usize, String> {
        match self.name.as_str() {
            "exit" => self.exit(),
            "echo" => self.echo(),
            "type" => self.cmd_type(),
            _ => Err(format!("{}: command not found", self.name)),
        }
    }

    fn cmd_type(&self) -> Result<usize, String> {
        if self.args.len() < 2 {
            return Err("Usage: type <command>".into());
        }

        let cmd = &self.args[1];

        let tp = CommandType::from_name(cmd);

        match tp {
            CommandType::Builtin => println!("{} is a shell builtin", cmd),
            CommandType::Binary => println!("{} is a binary or shell script", cmd),
            CommandType::Invalid => println!("{}: not found", cmd),
        }

        Ok(0)
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
