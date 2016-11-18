extern crate yertle;

use yertle::*;
use yertle::Command::*;

fn main() {
    let mut t = Turtle::new("yertle: koch", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('F', "F+F-F-F+F");
    l.constant('+');
    l.constant('-');
    let mut m = Machine::new();
    m.bind('F', Forward(2.0));
    m.bind('+', Left(90.0));
    m.bind('-', Right(90.0));
    let src = l.grow_n("F", 5);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(0.0, 600.0);
    while t.update(0.05) {
        prog.step(&mut t, 50);
    }
}

