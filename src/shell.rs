use std::fs;
use crate::command::Command;
use std::path::PathBuf;

#[derive(Default)]
pub struct Shell {
    status_code: i32,
    pub path: Vec<PathBuf>,
    pub pwd: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        if let Ok(path_os_string) = std::env::var("PATH") {
            let paths: Vec<PathBuf> = std::env::split_paths(&path_os_string).collect();

            Shell {
                status_code: 0,
                path: paths.into_iter().filter(|p| p.is_dir()).collect(),
                pwd: std::env::current_dir().unwrap(),
            }
        } else {
            Shell::default()
        }
    }

    pub fn execute(&mut self, input: String) -> Result<i32, String> {
        let cmd = Command::from(input);
        let result = cmd.execute(self);
        self.status_code = result.status;

        if let Some((stderr, append)) = cmd.stderr {
            let mut content = String::new();
            if append {
                content += &String::from_utf8_lossy(&fs::read(&stderr).unwrap_or_default()).to_string();
            }
            content += &result.stderr;
            fs::write(&stderr, content).map_err(|_| "cannot write to file")?;
        } else {
            eprint!("{}", result.stderr)
        }

        if let Some((stdout, append)) = cmd.stdout {
            let mut content = String::new();
            if append {
                content += &String::from_utf8_lossy(&fs::read(&stdout).unwrap_or_default()).to_string();
            }
            content += &result.stdout;
            fs::write(&stdout, content).map_err(|_| "cannot write to file")?;
        } else {
            print!("{}", result.stdout)
        }

        Ok(result.status)
    }
}
