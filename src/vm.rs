
use std;
use std::collections::HashMap;
use std::vec::Vec;

use command::*;
use turtle::*;

#[derive(Clone)]
pub struct Program {
    code: Vec<Command>,
    ip: usize
}

impl Program {
    pub fn finished(&self) -> bool {
        self.ip >= self.code.len()
    }

    pub fn execute(&self, turtle: &mut Turtle) {
        for cmd in self.code.iter() {
            let a = *cmd;
            turtle.execute(a);
        }
    }

    pub fn execute_iter(&self) -> std::slice::Iter<Command> {
        self.code.iter()
    }

    pub fn step(&mut self, turtle: &mut Turtle, n: usize) {
        let end = std::cmp::min(self.ip + n, self.code.len());
        while self.ip < end {
            turtle.execute(self.code[self.ip]);
            self.ip += 1;
        }
    }

    pub fn step_iter(&mut self, n: usize) -> std::slice::Iter<Command> {
        let end = std::cmp::min(self.ip + n, self.code.len());
        let old = self.ip;
        self.ip = end;
        self.code[old..end].iter()
    }

    pub fn finish(&mut self, turtle: &mut Turtle) {
        while self.ip < self.code.len() {
            turtle.execute(self.code[self.ip]);
            self.ip += 1;
        }
    }

    pub fn finish_iter(&mut self) -> std::slice::Iter<Command> {
        let old = self.ip;
        self.ip = self.code.len();
        self.code[old..].iter()
    }

    pub fn reset(&mut self) {
        self.ip = 0;
    }
}

pub struct Machine {
    commands: HashMap<char, Vec<Command>>
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            commands: HashMap::new()
        }
    }

    pub fn bind(&mut self, on: char, cmd: Command) {
        self.binds(on, vec![cmd]);
    }

    pub fn binds(&mut self, on: char, cmds: Vec<Command>) {
        self.commands.insert(on, cmds);
    }

    pub fn compile(&self, src: &str) -> Program {
        let mut v = Vec::with_capacity(src.len());
        for c in src.chars() {
            for cmd in self.commands.get(&c).unwrap().iter() {
                v.push(*cmd);
            }
        }
        Program {
            code: v,
            ip: 0,
        }
    }
}

