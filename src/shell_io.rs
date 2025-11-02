use std::fs::File;
use std::io::{PipeReader, PipeWriter, Read, Write};
use std::mem;
use std::process::Stdio;

pub enum Input {
    File(File),
    Pipe(PipeReader),
    Stdin,
    None,
}

impl Input {
    pub fn take(&mut self) -> Self {
        mem::replace(self, Input::None)
    }

    pub fn take_read(&mut self) -> Box<dyn Read> {
        let r = mem::replace(self, Input::None);

        match r {
            Input::File(f) => Box::new(f),
            Input::Pipe(p) => Box::new(p),
            Input::Stdin => Box::new(std::io::stdin()),
            Input::None => {
                panic!("error: tried to convert none input to read")
            }
        }
    }
}

impl From<Input> for Stdio {
    fn from(value: Input) -> Self {
        match value {
            Input::File(f) => f.into(),
            Input::Pipe(p) => p.into(),
            Input::Stdin => Stdio::inherit(),
            Input::None => {
                panic!("error: tried to convert none input to stdio")
            }
        }
    }
}

pub enum Output {
    File(File),
    Pipe(PipeWriter),
    Stdout,
    Stderr,
    None,
}

impl Output {
    pub fn take(&mut self) -> Self {
        mem::replace(self, Output::None)
    }

    pub fn take_write(&mut self) -> Box<dyn Write> {
        let r = mem::replace(self, Output::None);

        match r {
            Output::File(f) => Box::new(f),
            Output::Pipe(p) => Box::new(p),
            Output::Stdout => Box::new(std::io::stdout()),
            Output::Stderr => Box::new(std::io::stderr()),
            Output::None => {
                panic!("error: tried to convert none input to read")
            }
        }
    }
}

impl From<Output> for Stdio {
    fn from(value: Output) -> Stdio {
        match value {
            Output::File(f) => f.into(),
            Output::Pipe(p) => p.into(),
            Output::Stdout => Stdio::inherit(),
            Output::Stderr => Stdio::inherit(),
            Output::None => {
                panic!("error: tried to convert none input to stdio")
            }
        }
    }
}
