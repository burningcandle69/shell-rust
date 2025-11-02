use regex::{Captures, Regex};
use std::fmt::Display;

/// This module will contain the Command struct
/// and the parsing logic along with it

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub stdout: Option<(String, bool)>,
    pub stderr: Option<(String, bool)>,
}

#[derive(Default)]
pub struct ExecResult {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl ExecResult {
    pub fn with_status(mut self, status: i32) -> Self {
        self.status = status;
        self
    }

    pub fn with_stdout<T: Display>(mut self, stdout: T) -> Self {
        self.stdout = stdout.to_string();
        self
    }

    pub fn with_stderr<T: Display>(mut self, stderr: T) -> Self {
        self.stderr = stderr.to_string();
        self
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
        let mut stdout = None;
        let mut stderr = None;

        if let Some(idx) = args
            .iter()
            .position(|x| [">", ">>", "1>", "1>>"].contains(&x.as_str()))
        {
            upto = upto.min(idx);
            stdout = Some((
                args[idx + 1].clone(),
                args[idx] == "1>>" || args[idx] == ">>",
            ));
        }

        if let Some(idx) = args
            .iter()
            .position(|x| ["2>", "2>>"].contains(&x.as_str()))
        {
            upto = upto.min(idx);
            stderr = Some((args[idx + 1].clone(), args[idx] == "2>>"));
        }

        Command {
            name: args[0].clone(),
            args: args[..upto].to_vec(),
            stdout,
            stderr,
        }
    }
}
