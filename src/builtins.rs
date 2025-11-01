/// This module contains the builtin commands supported
/// by our shell and the execution logic
use crate::command::Command;
use std::path::PathBuf;
use is_executable::is_executable;

impl Command {
    pub fn execute(&self, path: &Vec<PathBuf>) -> Result<usize, String> {
        match self.name.as_str() {
            "exit" => self.exit(),
            "echo" => self.echo(),
            "type" => self.cmd_type(path),
            _ => Err(format!("{}: command not found", self.name)),
        }
    }

    fn cmd_type(&self, path: &Vec<PathBuf>) -> Result<usize, String> {
        if self.args.len() < 2 {
            return Err("Usage: type <command>".into());
        }

        let cmd = &self.args[1];

        match cmd.as_str() {
            "exit" | "echo" | "type" => println!("{} is a shell builtin", cmd),
            _ => {
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

                        if name == cmd {
                            if let Some(path_str) = entry.path().to_str() {
                                println!("{} is {}", cmd, path_str)
                            }
                            return Ok(0);
                        }
                    }
                }
                println!("{}: not found", cmd);
            }
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
