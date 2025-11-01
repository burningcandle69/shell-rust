/// This module contains the builtin commands supported
/// by our shell and the execution logic
use crate::command::Command;
use crate::shell::Shell;
use is_executable::is_executable;
use std::path::PathBuf;

impl Command {
    pub fn execute(&self, shell: &mut Shell) -> Result<i32, String> {
        match self.name.as_str() {
            "exit" => self.exit(),
            "echo" => self.echo(),
            "type" => self.cmd_type(&shell.path),
            "pwd" => self.pwd(&shell.pwd),
            "cd" => self.cd(shell),
            _ => {
                if let Some(exe) = self.find_executable(&self.name, &shell.path) {
                    let mut cmd = std::process::Command::new(exe.file_name().unwrap());
                    if self.args.len() > 1 {
                        cmd.args(&self.args[1..]);
                    }
                    let mut child = cmd.spawn().unwrap();
                    let status = child.wait().unwrap_or_default().code().unwrap_or_default();
                    Ok(status)
                } else {
                    Err(format!("{}: command not found", self.name))
                }
            }
        }
    }

    fn cmd_type(&self, path: &Vec<PathBuf>) -> Result<i32, String> {
        if self.args.len() < 2 {
            return Err("Usage: type <command>".into());
        }

        let cmd = &self.args[1];

        match cmd.as_str() {
            "exit" | "echo" | "type" | "pwd" => println!("{} is a shell builtin", cmd),
            _ => {
                if let Some(path_str) = self.find_executable(cmd, path) {
                    println!("{} is {}", cmd, path_str.to_str().unwrap_or(""))
                } else {
                    println!("{}: not found", cmd);
                }
            }
        }

        Ok(0)
    }

    fn cd(&self, shell: &mut Shell) -> Result<i32, String> {
        let home = std::env::var("HOME").unwrap_or("~".into());

        if self.args.len() < 2 {
            return Err("cd: No such file or directory".into());
        }

        let p = if self.args[1] == "~" {
            PathBuf::from(&home)
        } else {
            PathBuf::from(&self.args[1])
        };

        if !p.exists() {
            return Err(format!("cd: {}: No such file or directory", self.args[1]));
        }

        shell.pwd = p.canonicalize().unwrap();
        std::env::set_current_dir(p).unwrap();

        Ok(0)
    }

    fn pwd(&self, pwd: &PathBuf) -> Result<i32, String> {
        println!("{}", pwd.to_str().unwrap());
        Ok(0)
    }

    fn exit(&self) -> Result<i32, String> {
        if self.args.len() < 2 {
            return Err("Usage: exit <exit_code>".into());
        }
        let exit_code = self.args[1].parse().map_err(|_| "invalid error code")?;
        std::process::exit(exit_code)
    }

    fn echo(&self) -> Result<i32, String> {
        println!("{}", self.args[1..].join(" "));
        Ok(0)
    }

    fn find_executable(&self, cmd: &String, path: &Vec<PathBuf>) -> Option<PathBuf> {
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
                    return Some(entry.path());
                }
            }
        }
        None
    }
}
