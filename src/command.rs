use crate::shell_io::{Input, Output};
use regex::{Captures, Regex};
use std::fs::OpenOptions;
use std::io::{Read, Write};

/// This module will contain the Command struct
/// and the parsing logic along with it

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub stdin: Input,
    pub stdout: Output,
    pub stderr: Output,
}

impl Command {
    pub fn new(name: String) -> Command {
        Command {
            name,
            args: vec![],
            stdin: Input::Stdin,
            stdout: Output::Stdout,
            stderr: Output::Stderr,
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Command {
        self.args = args;
        self
    }

    pub fn with_stderr(mut self, stderr: Output) -> Command {
        self.stderr = stderr;
        self
    }

    pub fn with_stdout(mut self, stdout: Output) -> Command {
        self.stdout = stdout;
        self
    }

    pub fn take_io(&mut self) -> (Box<dyn Read>, Box<dyn Write>, Box<dyn Write>) {
        let stdin = self.stdin.take_read();
        let stdout = self.stdout.take_write();
        let stderr = self.stderr.take_write();
        (stdin, stdout, stderr)
    }
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let re =
            Regex::new(r#"(?:[^'"\s\\]|\\.)+|'(?:[^'\\]|\\.)*'|"(?:[^"\\]|\\.)*"| +"#).unwrap();
        let back = Regex::new(r#"\\(.)"#).unwrap();
        let back_quotes = Regex::new(r#"\\(')"#).unwrap();
        let back_double_quotes = Regex::new(r#"\\([`$"\\\n])"#).unwrap();

        let mut args = vec![];
        let mut buf = String::new();
        for val in re.captures_iter(&value) {
            // println!("v: {:?}", val.get_match().as_str());
            let v = val.get_match().as_str().trim();
            if v.is_empty() {
                args.push(buf);
                buf = String::new();
                continue;
            }
            let f = v.chars().nth(0).unwrap();
            let l = v.chars().last().unwrap();
            if f == '\'' && l == '\'' {
                let v = back_quotes.replace_all(v, |caps: &Captures| format!("{}", &caps[1]));
                let v = v[1..v.len() - 1].to_string();
                buf += &v;
            } else if f == '"' && l == '"' {
                let v =
                    back_double_quotes.replace_all(v, |caps: &Captures| format!("{}", &caps[1]));
                let v = v[1..v.len() - 1].to_string();
                buf += &v;
            } else {
                let v = back.replace_all(v, |caps: &Captures| format!("{}", &caps[1]));
                buf += &v;
            }
        }

        if !buf.trim().is_empty() {
            args.push(buf);
        }

        let mut upto = args.len();
        let mut cmd = Command::new(args[0].clone());

        if let Some(idx) = args
            .iter()
            .position(|x| [">", "1>", ">>", "1>>"].contains(&x.as_str()))
        {
            upto = upto.min(idx);
            let mut options = OpenOptions::new();
            options.create(true);

            if args[idx] == ">>" || args[idx] == "1>>" {
                options.append(true);
            } else {
                options.write(true).truncate(true);
            }

            let stdout = options.open(args[idx + 1].clone()).unwrap();
            cmd = cmd.with_stdout(Output::File(stdout));
        }

        if let Some(idx) = args
            .iter()
            .position(|x| ["2>", "2>>"].contains(&x.as_str()))
        {
            upto = upto.min(idx);
            let mut options = OpenOptions::new();
            options.create(true);

            if args[idx] == "2>>" {
                options.append(true);
            } else {
                options.write(true).truncate(true);
            }

            let stderr = options.open(args[idx + 1].clone()).unwrap();

            cmd = cmd.with_stderr(Output::File(stderr));
        }

        cmd.with_args(args[..upto].to_vec())
    }
}
