extern crate yertle;

use yertle::*;
use yertle::Command::*;

fn main() {
    let mut t = Turtle::new("yertle: pythagoras", 800, 600).unwrap();

    let mut l = LSystem::new();
    l.variable('0', "1[0]0");
    l.variable('1', "11");
    let mut m = Machine::new();
    m.binds('0', vec![Color(0, 255, 0), Forward(2.0), Color(255, 255, 255)]);
    m.bind('1', Forward(1.0));
    m.binds('[', vec![PushState, Left(45.0)]);
    m.binds(']', vec![PopState, Right(45.0)]);
    let src = l.grow_n("0", 8);
    println!("#cmds: {}", src.len());
    let mut prog = m.compile(src.as_str());

    t.goto(400.0, 600.0);
    t.face(-90.0);
    while t.update(0.05) {
        prog.step(&mut t, 50);
    }
}

