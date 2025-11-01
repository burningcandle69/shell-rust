use crate::command::CommandType::{Builtin, Invalid};
use std::fmt::{Display, Formatter};

/// This module will contain the Command struct
/// and the parsing logic along with it

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let args = value
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        Command {
            name: args[0].clone(),
            args,
        }
    }
}

pub enum CommandType {
    Builtin,
    Binary,
    Invalid,
}

impl CommandType {
    pub fn from_name(name: &String) -> Self {
        match name.as_str() {
            "exit" | "echo" | "type" => Builtin,
            _ => Invalid,
        }
    }
}
