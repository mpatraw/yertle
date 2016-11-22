extern crate yertle;

use yertle::*;

fn main() {
    let mut t = Turtle::new("yertle: empty", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('F', "F[++F][+F][-F]F[+F][+F]F");
    let mut m = Machine::new();
    m.bind('F', Forward(5.0));
    m.bind('-', Left(22.0));
    m.bind('+', Right(22.0));
    m.bind('[', PushState);
    m.bind(']', PopState);
    let src = l.grow_n("F", 5);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(400.0, 600.0);
    t.face(-90.0);
    while t.update(0.1) {
        prog.step(&mut t, 600);
    }
}

