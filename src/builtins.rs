/// This module contains the builtin commands supported
/// by our shell and the execution logic
use crate::command::{Command, ExecResult};
use crate::shell::Shell;
use is_executable::is_executable;
use std::path::PathBuf;
use std::process::Stdio;

impl Command {
    pub fn execute(&self, shell: &mut Shell) -> ExecResult {
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
                let res = ExecResult::default();

                if let Some(exe) = self.find_executable(&self.name, &shell.path) {
                    let mut cmd = std::process::Command::new(exe.file_name().unwrap());
                    cmd.stderr(Stdio::piped()).stdout(Stdio::piped());
                    if self.args.len() > 1 {
                        cmd.args(&self.args[1..]);
                    }
                    let child = cmd.spawn().unwrap();
                    let output = child.wait_with_output().unwrap();
                    res.with_status(output.status.code().unwrap())
                        .with_stdout(String::from_utf8_lossy(&output.stdout).to_string())
                        .with_stderr(String::from_utf8_lossy(&output.stderr).to_string())
                } else {
                    res.with_stderr(format!("{}: command not found\n", self.name))
                }
            }
        }
    }

    fn cmd_type(&self, path: &Vec<PathBuf>) -> ExecResult {
        let result = ExecResult::default();

        if self.args.len() < 2 {
            return result.with_stderr("Usage: type <command>\n");
        }

        let cmd = &self.args[1];

        let res = match cmd.as_str() {
            "exit" | "echo" | "type" | "pwd" | "history" => format!("{} is a shell builtin\n", cmd),
            _ => {
                if let Some(path_str) = self.find_executable(cmd, path) {
                    format!("{} is {}\n", cmd, path_str.to_str().unwrap_or(""))
                } else {
                    format!("{}: not found\n", cmd)
                }
            }
        };

        result.with_stdout(res)
    }

    fn cd(&self, shell: &mut Shell) -> ExecResult {
        let res = ExecResult::default();

        let home = std::env::var("HOME").unwrap_or("~".into());

        if self.args.len() < 2 {
            return res.with_stderr("cd: No such file or directory\n");
        }

        let p = if self.args[1] == "~" {
            PathBuf::from(&home)
        } else {
            PathBuf::from(&self.args[1])
        };

        if !p.exists() {
            return res.with_stderr(format!("cd: {}: No such file or directory\n", self.args[1]));
        }

        shell.pwd = p.canonicalize().unwrap();
        std::env::set_current_dir(p).unwrap();

        res
    }

    fn history(&self, shell: &mut Shell) -> ExecResult {
        if self.args.len() == 3 {
            if self.args[1] == "-r" {
                let _ = shell.read_history(self.args[2].clone());
            } else if self.args[1] == "-w" {
                let _ = shell.write_history(self.args[2].clone());
            }
            return ExecResult::default();
        }

        let history = &shell.history;
        let mut lim = history.len();
        if self.args.len() > 1 {
            lim = lim.min(self.args[1].parse().unwrap());
        }
        let mut res = String::new();
        for i in history.len() - lim..history.len() {
            res += &format!("    {}  {}\n", i + 1, history[i]);
        }
        ExecResult::default().with_stdout(res)
    }

    fn pwd(&self, pwd: &PathBuf) -> ExecResult {
        ExecResult::default().with_stdout(format!("{}\n", pwd.to_str().unwrap()))
    }

    fn exit(&self) -> ExecResult {
        if self.args.len() < 2 {
            return ExecResult::default()
                .with_status(1)
                .with_stderr("Usage: exit <exit_code>");
        }
        let exit_code = self.args[1].parse::<i32>();
        match exit_code {
            Ok(code) => std::process::exit(code),
            Err(_) => ExecResult::default()
                .with_stderr(1)
                .with_stderr("invalid error code"),
        }
    }

    fn echo(&self) -> ExecResult {
        ExecResult::default().with_stdout(format!("{}\n", self.args[1..].join(" ").trim()))
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
