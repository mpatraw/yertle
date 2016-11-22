extern crate yertle;

use yertle::*;

fn generate_tree() -> Program {
    let mut m = Machine::new();
    m.bind('F', Forward(3.0));
    m.bind('-', Left(22.0));
    m.bind('+', Right(22.0));
    m.bind('[', PushState);
    m.bind(']', PopState);
    m.bind('B', BlendSub(0, 40, 0));
    m.bind('C', Color(0, 255, 0));

    let mut l = LSystem::new();
    l.variable('F', "FF[BB++F][B+FFF]F[B-FFF][BB--F]");
    let mut src = "C".to_string();
    src.push_str(l.grow_n("F", 4).as_str());
    println!("tree #cmds: {}", src.len());

    m.compile(src.as_str())
}

fn generate_star() -> Program {
    let mut m = Machine::new();
    m.bind('F', Forward(50.0));
    m.bind('+', Left(144.0));
    m.bind('Y', Color(255, 255, 0));
    m.bind('[', PushState);
    m.bind(']', PopState);

    let l = LSystem::new();
    let src = l.grow_n("[YF+F+F+F+F+]", 0);
    println!("star #cmds: {}", src.len());

    m.compile(src.as_str())
}

fn run_program(t: &mut Turtle, mut prog: Program) {
    while !prog.finished() {
        t.update(0.05);
        prog.step(t, 5000);
    }
}

fn main() {
    let mut t = Turtle::new("yertle: christmas", 800, 600).unwrap();

    t.goto(400.0, 100.0);
    t.face(90.0);
    run_program(&mut t, generate_tree());
    t.reset();
    t.goto(375.0, 75.0);
    t.face(36.0);
    run_program(&mut t, generate_star());

    t.run();
}

