use crate::builtins::Builtin;
use crate::command::Command;

#[derive(Default)]
pub struct Shell {
    status_code: usize,
    path: Vec<String>
}

impl Shell {
    pub fn execute(&mut self, input: String) -> Result<usize, String> {
        let cmd = Command::from(input);
        
        if cmd.is_builtin() {
            let status = Builtin::from(cmd).execute()?;
            self.status_code = status;
            return Ok(status);
        }

        Err(format!("{}: command not found", cmd.name))
    }
}