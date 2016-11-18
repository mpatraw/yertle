extern crate yertle;

use yertle::*;
use yertle::TurtleCommand::*;

fn main() {
    let mut t = Turtle::new("yertle: sierpinski", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('F', "F-G+F+G-F");
    l.variable('G', "GG");
    l.constant('+');
    l.constant('-');
    let mut m = Machine::new();
    m.bind('F', Forward(20.0));
    m.bind('G', Forward(20.0));
    m.bind('+', Left(120.0));
    m.bind('-', Right(120.0));
    let src = l.grow_n("F-G-G", 5);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(0.0, 0.0);
    while t.update(0.05) {
        prog.step(&mut t, 5);
    }
}

