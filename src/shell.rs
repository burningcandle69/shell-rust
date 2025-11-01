use crate::command::Command;
use std::path::PathBuf;

#[derive(Default)]
pub struct Shell {
    status_code: i32,
    path: Vec<PathBuf>,
}

impl Shell {
    pub fn new() -> Self {
        if let Ok(path_os_string) = std::env::var("PATH") {
            let paths: Vec<PathBuf> = std::env::split_paths(&path_os_string).collect();

            Shell {
                status_code: 0,
                path: paths.into_iter().filter(|p| p.is_dir()).collect(),
            }
        } else {
            Shell::default()
        }
    }

    pub fn execute(&mut self, input: String) -> Result<i32, String> {
        let cmd = Command::from(input);
        let status = cmd.execute(&self.path)?;
        self.status_code = status;
        Ok(status)
    }
}
