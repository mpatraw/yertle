extern crate yertle;

use yertle::*;
use yertle::TurtleCommand::*;

fn main() {
    let mut t = Turtle::new("yertle: fractal plant", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('X', "F-[[X]+X]+F[+FX]-X");
    l.variable('F', "FF");
    l.constant('+');
    l.constant('-');
    l.constant('[');
    l.constant(']');
    let mut m = Machine::new();
    m.bind('F', Forward(3.0));
    m.bind('1', Forward(3.0));
    m.bind('-', Rotate(-25.0));
    m.bind('+', Rotate(25.0));
    m.bind('[', PushState);
    m.bind(']', PopState);
    m.bind('X', Noop);
    let src = l.grow_n("X", 6);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(200.0, 600.0);
    t.face(-60.0);
    let mut cnt = 0usize;
    while t.update(0.05) {
        for cmd in prog.step_iter(50) {
            t.color(0, (cnt * 255 / src.len()) as u8, 0);
            t.execute(*cmd);
            cnt += 1;
        }
    }
}

