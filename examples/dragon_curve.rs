extern crate yertle;

use yertle::*;
use yertle::Command::*;

fn main() {
    let mut t = Turtle::new("yertle: dragon curve", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('X', "X+YF+");
    l.variable('Y', "-FX-Y");
    let mut m = Machine::new();
    m.bind('F', Forward(3.0));
    m.bind('1', Forward(3.0));
    m.bind('-', Left(90.0));
    m.bind('+', Right(90.0));
    m.bind('X', Noop);
    m.bind('Y', Noop);
    let src = l.grow_n("FX", 17);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(400.0, 300.0);
    t.face(-90.0);
    let mut cnt = 0i32;
    while t.update(0.00) {
        t.color((cnt / 10) as u8, 0, 0);
        for cmd in prog.step_iter(20) {
            t.execute(*cmd);
            cnt += 1;
        }
    }
}

