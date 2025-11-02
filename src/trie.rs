use std::collections::HashMap;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Trie {
    is_member: bool,
    members: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            is_member: false,
            members: HashMap::new(),
        }
    }

    pub fn add(&mut self, s: Chars) {
        let mut tr = self;
        for c in s {
            tr = tr.members.entry(c).or_insert(Trie::new());
        }
        tr.is_member = true;
    }

    pub fn fuzzy(&self, mut s: Chars) -> Vec<String> {
        let this = match s.nth(0) {
            Some(v) => v,
            None => {
                let mut r = self.list_all();
                if self.is_member {
                    r.push("".into()); 
                }
                return r
            },
        };
        if self.members.contains_key(&this) {
            let mut res = vec![];
            let r = this.to_string();
            let child = self.members.get(&this).unwrap();
            for more_r in child.fuzzy(s) {
                res.push(r.clone() + &more_r)
            }
            res
        } else {
            vec![]
        }
    }

    pub fn list_all(&self) -> Vec<String> {
        let mut this = vec![];
        for (m, t) in &self.members {
            let mut res = vec![];
            let r = m.to_string();
            for more_r in t.list_all() {
                res.push(r.clone() + &more_r);
            }
            if t.is_member {
                this.push(r);
            }
            this.append(&mut res)
        }
        this
    }
}
