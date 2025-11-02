use crate::command::Command;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Shell {
    status_code: i32,
    pub path: Vec<PathBuf>,
    pub pwd: PathBuf,
    pub hist_file: String,
    pub history: Vec<String>,
    pub appended: usize,
}

impl Shell {
    pub fn new() -> Self {
        if let Ok(path_os_string) = std::env::var("PATH") {
            let paths: Vec<PathBuf> = std::env::split_paths(&path_os_string).collect();
            let hist_file = std::env::var("HISTFILE").unwrap_or_default();

            Shell {
                status_code: 0,
                path: paths.into_iter().filter(|p| p.is_dir()).collect(),
                pwd: std::env::current_dir().unwrap(),
                hist_file,
                history: vec![],
                appended: 0
            }
        } else {
            Shell::default()
        }
    }

    pub fn execute(&mut self, input: String) -> Result<i32, String> {
        self.history.push(input.clone());

        let cmd = Command::from(input);
        let result = cmd.execute(self);
        self.status_code = result.status;

        if let Some((stderr, append)) = cmd.stderr {
            let mut content = String::new();
            if append {
                content +=
                    &String::from_utf8_lossy(&fs::read(&stderr).unwrap_or_default()).to_string();
            }
            content += &result.stderr;
            fs::write(&stderr, content).map_err(|_| "cannot write to file")?;
        } else {
            eprint!("{}", result.stderr)
        }

        if let Some((stdout, append)) = cmd.stdout {
            let mut content = String::new();
            if append {
                content +=
                    &String::from_utf8_lossy(&fs::read(&stdout).unwrap_or_default()).to_string();
            }
            content += &result.stdout;
            fs::write(&stdout, content).map_err(|_| "cannot write to file")?;
        } else {
            print!("{}", result.stdout)
        }

        Ok(result.status)
    }

    pub fn read_history<P: AsRef<Path>>(&mut self, path_buf: P) -> std::io::Result<()> {
        let f = fs::read(path_buf)?;
        let h = String::from_utf8_lossy(&f);
        self.history.append(
            &mut h
                .trim()
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        );
        Ok(())
    }

    pub fn write_history<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        fs::write(path, self.history.join("\n") + "\n")?;
        Ok(())
    }
    
    pub fn append_history<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let f = fs::read(&path).unwrap_or_default();
        let h = String::from_utf8_lossy(&f).to_string();
        let hi = self.history[self.appended..].join("\n") + "\n";
        self.appended = self.history.len();
        fs::write(path, h + &hi)?;
        Ok(())
    }
}
