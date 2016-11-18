extern crate yertle;

use yertle::*;
use yertle::TurtleCommand::*;

fn main() {
    let mut t = Turtle::new("yertle: arrowhead", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('A', "+B-A-B+");
    l.variable('B', "-A+B+A-");
    l.constant('+');
    l.constant('-');
    let mut m = Machine::new();
    m.bind('A', Forward(20.0));
    m.bind('B', Forward(20.0));
    m.bind('+', Left(60.0));
    m.bind('-', Right(60.0));
    let src = l.grow_n("A", 5);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(0.0, 600.0);
    while t.update(0.05) {
        prog.step(&mut t, 5);
    }
}

