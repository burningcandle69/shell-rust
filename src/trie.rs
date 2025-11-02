use std::collections::HashMap;
use std::str::Chars;

#[derive(Clone)]
pub struct Trie {
    members: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            members: HashMap::new(),
        }
    }

    pub fn add(&mut self, mut s: Chars) {
        let c = match s.nth(0) {
            Some(v) => v,
            None => return,
        };
        self.members.entry(c).or_insert(Trie::new()).add(s);
    }

    pub fn fuzzy(&self, mut s: Chars) -> Vec<String> {
        let this = match s.nth(0) {
            Some(v) => v,
            None => {
                let r = self.list_all();
                if r.is_empty() {
                    return vec!["".into()]
                }
                return r
            },
        };
        if self.members.contains_key(&this) {
            let mut res = vec![];
            let r = this.to_string();
            for more_r in self.members.get(&this).unwrap().fuzzy(s) {
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
            if res.is_empty() {
                this.push(r)
            } else {
                this.append(&mut res)
            }
        }
        this
    }
}
