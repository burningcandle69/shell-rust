use crate::command::Command;

#[derive(Default)]
pub struct Shell {
    status_code: usize,
    path: Vec<String>,
}

impl Shell {
    pub fn execute(&mut self, input: String) -> Result<usize, String> {
        let cmd = Command::from(input);
        let status = cmd.execute()?;
        self.status_code = status;
        Ok(status)
    }
}
