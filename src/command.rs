pub struct Command {
    pub name: String,
    pub args: Vec<String>
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let args = value
            .trim()
            .split(" ").map(|x| x.to_string())
            .collect::<Vec<_>>();
        
        Command {
            name: args[0].clone(),
            args
        }
    }
}
