
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LSystem {
    rules: HashMap<char, String>
}

impl LSystem {
    pub fn new() -> LSystem {
        LSystem {
            rules: HashMap::new()
        }
    }

    pub fn variable(&mut self, on: char, res: &str) {
        self.rules.insert(on, res.to_string());
    }

    pub fn constant(&mut self, on: char) {
        self.rules.insert(on, on.to_string());
    }

    pub fn grow_once(&self, axiom: &str) -> String {
        let mut res = String::with_capacity(axiom.len() * 2);
        for c in axiom.chars() {
            match self.rules.get(&c) {
                Some(change) => res.push_str(change),
                None => res.push_str(c.to_string().as_str())
            }
        }
        res
    }

    pub fn grow_n(&self, axiom: &str, times: u32) -> String {
        let mut res = axiom.to_string();
        for _ in 0..times {
            res = self.grow_once(res.as_str());
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_all() {
        let mut l = LSystem::new();
        l.variable('0', "1[0]0");
        l.variable('1', "11");
        l.constant('[');
        l.constant(']');
        println!("{}", l.run("0", 3));
    }
}
