use regex::{Captures, Regex};

/// This module will contain the Command struct
/// and the parsing logic along with it

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let value = value.replace("''", "");
        let value = value.replace(r#""""#, "");
        let re =
            Regex::new(r#"(?:[^'"\s\\]|\\.)+|'(?:[^'\\]|\\.)*'|"(?:[^"\\]|\\.)*""#).unwrap();
        let back_quotes = Regex::new(r#"\\(')"#).unwrap();
        let back_double_quotes = Regex::new(r#"\\(")"#).unwrap(); 
        let back = Regex::new(r#"\\(.)"#).unwrap();

        let mut args = vec![];
        for val in re.captures_iter(&value) {
            let v = val.get_match().as_str().trim();
            if v.is_empty() {
                continue;
            }
            let f = v.chars().nth(0).unwrap();
            let l = v.chars().last().unwrap();
            if f == '\'' &&  l == '\''  {
                let v = back_quotes.replace_all(v, |caps: &Captures| format!("{}", &caps[1]));
                let v = v[1..v.len() - 1].to_string();
               args.push(v); 
            } else if f == '"' && l == '"' {
                let v = back_double_quotes.replace_all(v, |caps: &Captures| format!("{}", &caps[1]));
                let v = v[1..v.len() - 1].to_string();
                args.push(v); 
            } else {
                let v = back.replace_all(v, |caps: &Captures| format!("{}", &caps[1]));
                args.push(v.to_string());
            }
            // println!("{v:?}");
        }

        // let args = value
        //     .trim()
        //     .split(" ")
        //     .map(|x| x.to_string())
        //     .collect::<Vec<_>>();

        Command {
            name: args[0].clone(),
            args,
        }
    }
}
