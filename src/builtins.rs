/// This module contains the builtin commands supported
/// by our shell and the execution logic
use crate::command::Command;
use crate::shell::Shell;
use is_executable::is_executable;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Stdio};

pub enum ChildOrStatus {
    Child(Child),
    Status(i32),
}

const OK: ChildOrStatus = ChildOrStatus::Status(0);
const NOT_OK: ChildOrStatus = ChildOrStatus::Status(1);

impl ChildOrStatus {
    pub fn wait(self) -> i32 {
        match self {
            ChildOrStatus::Child(mut c) => {
                c.wait().unwrap_or_default().code().unwrap_or_default()
            },
            ChildOrStatus::Status(s) => s,
        }
    }
}

impl Command {
    pub fn execute(&mut self, shell: &mut Shell) -> std::io::Result<ChildOrStatus> {
        match self.name.as_str() {
            "exit" => {
                let _ = shell.write_history(&shell.hist_file);
                self.exit()
            }
            "echo" => self.echo(),
            "type" => self.cmd_type(&shell.path),
            "pwd" => self.pwd(&shell.pwd),
            "cd" => self.cd(shell),
            "history" => self.history(shell),
            _ => {
                if let Some(exe) = self.find_executable(&self.name, &shell.path) {
                    let mut cmd = std::process::Command::new(exe.file_name().unwrap());
                    cmd.stdin(Stdio::from(self.stdin.take()));
                    cmd.stdout(Stdio::from(self.stdout.take()));
                    cmd.stderr(Stdio::from(self.stderr.take()));

                    if self.args.len() > 1 {
                        cmd.args(&self.args[1..]);
                    }
                    let child = cmd.spawn()?;

                    Ok(ChildOrStatus::Child(child))
                } else {
                    let (_, _, mut stderr) = self.take_io();
                    let _ = write!(stderr, "{}: command not found\n", self.name);
                    Ok(NOT_OK)
                }
            }
        }
    }

    fn cmd_type(&mut self, path: &Vec<PathBuf>) -> std::io::Result<ChildOrStatus> {
        let (_, mut stdout, mut stderr) = self.take_io();

        if self.args.len() < 2 {
            write!(stderr, "Usage: type <command>\n")?;
            return Ok(NOT_OK);
        }

        let cmd = &self.args[1];

        match cmd.as_str() {
            "exit" | "echo" | "type" | "pwd" | "history" => {
                write!(stdout, "{} is a shell builtin\n", cmd)
            }
            _ => {
                if let Some(path_str) = self.find_executable(cmd, path) {
                    write!(stdout, "{} is {}\n", cmd, path_str.to_str().unwrap_or(""))
                } else {
                    write!(stderr, "{}: not found\n", cmd)
                }
            }
        }?;

        Ok(OK)
    }

    fn cd(&mut self, shell: &mut Shell) -> std::io::Result<ChildOrStatus> {
        let (_, _, mut stderr) = self.take_io();
        let home = std::env::var("HOME").unwrap_or("~".into());

        if self.args.len() < 2 {
            write!(stderr, "cd: No such file or directory\n")?;
            return Ok(NOT_OK);
        }

        let p = if self.args[1] == "~" {
            PathBuf::from(&home)
        } else {
            PathBuf::from(&self.args[1])
        };

        if !p.exists() {
            write!(stderr, "cd: {}: No such file or directory\n", self.args[1])?;
            return Ok(NOT_OK);
        }

        shell.pwd = p.canonicalize()?;
        std::env::set_current_dir(p)?;

        Ok(OK)
    }

    fn history(&mut self, shell: &mut Shell) -> std::io::Result<ChildOrStatus> {
        let (_, mut stdout, mut stderr) = self.take_io();
        if self.args.len() == 3 {
            if self.args[1] == "-r" {
                let _ = shell.read_history(self.args[2].clone());
            } else if self.args[1] == "-w" {
                let _ = shell.write_history(self.args[2].clone());
            } else if self.args[1] == "-a" {
                let _ = shell.append_history(self.args[2].clone());
            }
            return Ok(OK);
        }

        let history = &shell.history;
        let mut lim = history.len();
        if self.args.len() > 1 {
            match self.args[1].parse() {
                Ok(l) => lim = lim.min(l),
                Err(e) => write!(stderr, "{e}")?,
            }
        }

        for i in history.len() - lim..history.len() {
            write!(stdout, "    {}  {}\n", i + 1, history[i])?;
        }

        Ok(OK)
    }

    fn pwd(&mut self, pwd: &PathBuf) -> std::io::Result<ChildOrStatus> {
        let (_, mut stdout, _) = self.take_io();
        write!(stdout, "{}\n", pwd.to_str().unwrap())?;
        Ok(OK)
    }

    fn exit(&mut self) -> std::io::Result<ChildOrStatus> {
        let (_, _, mut stderr) = self.take_io();

        if self.args.len() < 2 {
            write!(stderr, "Usage: exit <exit_code>")?;
            return Ok(NOT_OK);
        }
        let exit_code = self.args[1].parse::<i32>();
        match exit_code {
            Ok(code) => std::process::exit(code),
            Err(_) => {
                write!(stderr, "invalid error code")?;
                Ok(NOT_OK)
            }
        }
    }

    fn echo(&mut self) -> std::io::Result<ChildOrStatus> {
        let (_, mut stdout, _) = self.take_io();
        write!(stdout, "{}\n", self.args[1..].join(" ").trim())?;
        Ok(OK)
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
