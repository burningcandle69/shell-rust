use crate::command::Command;

impl Command {
    pub fn is_builtin(&self) -> bool {
        match self.name.as_str() {
            "exit" => true,
            _ => false
        }
    }
}

pub struct Builtin {
    cmd: Command
}

impl From<Command> for Builtin {
    fn from(cmd: Command) -> Self {
        Builtin {cmd}
    }
}

impl Builtin {
    pub fn execute(&self) -> Result<usize, String> {
        match self.cmd.name.as_str() {
           "exit" => self.exit(),
            _ => Err("not a builtin".into())
        }
    }

    fn exit(&self) -> Result<usize, String> {
        if self.cmd.args.len() < 2 {
            return Err("Usage: exit <exit_code>".into());
        }
        let exit_code = self.cmd.args[1].parse().map_err(|_| "invalid error code")?;
        std::process::exit(exit_code)
    }
}