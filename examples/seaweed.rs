extern crate yertle;

use yertle::*;
use yertle::Command::*;

fn main() {
    let mut t = Turtle::new("yertle: seaweed", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('F', "FF-[-F+F+F]+[+F-F-F]");
    l.constant('+');
    l.constant('-');
    l.constant('[');
    l.constant(']');
    let mut m = Machine::new();
    m.bind('F', Forward(10.0));
    m.bind('-', Left(22.0));
    m.bind('+', Right(22.0));
    m.binds('[', vec![PushState, BlendMul(1.0, 1.2, 0.95)]);
    m.bind(']', PopState);
    let src = l.grow_n("F", 4);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    while !t.mouse_left() {
        t.update(0.0);
    }

    t.goto(200.0, 600.0);
    t.face(-60.0);
    t.color(0, 64, 128);
    while t.update(0.05) {
        for cmd in prog.step_iter(50) {
            t.execute(*cmd);
        }
    }
}

