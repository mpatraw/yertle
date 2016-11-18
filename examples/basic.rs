extern crate yertle;

use yertle::*;

fn main() {
    let mut t = Turtle::new("yertle: basic", 800, 600).unwrap();

    let mut n = 5.0;
    t.color(0, 0, 0);
    t.goto(400.0, 300.0);
    while t.update(0.01) {
        t.color(n as u8, 0, 0);
        t.execute(Command::Rotate(45.0));
        t.execute(Command::Forward(n));
        n += 1.0;
    }
}

