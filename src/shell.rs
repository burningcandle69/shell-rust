use crate::builtins::ChildOrStatus;
use crate::command::Command;
use crate::shell_io::{Input, Output};
use std::fs;
use std::io::pipe;
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
                appended: 0,
            }
        } else {
            Shell::default()
        }
    }

    pub fn execute(&mut self, input: String) -> std::io::Result<i32> {
        self.history.push(input.clone());

        let mut cmd = Command::from(input);

        let mut cmds = cmd
            .args
            .split(|x| x == "|")
            .map(|args| Command::new(args[0].clone()).with_args(args.to_vec()))
            .collect::<Vec<_>>();

        let n = cmds.len();

        cmds[n - 1].stderr = cmd.stderr.take();
        cmds[n - 1].stdout = cmd.stdout.take();

        for i in 1..n {
            let (pi, po) = pipe()?;
            cmds[i - 1].stdout = Output::Pipe(po);
            cmds[i].stdin = Input::Pipe(pi);
        }

        let r = cmds
            .into_iter()
            .map(|mut cmd| cmd.execute(self))
            .collect::<std::io::Result<Vec<ChildOrStatus>>>()?;

        for x in r {
            self.status_code = x.wait();
        }

        Ok(0)
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
