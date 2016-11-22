
use std;

use std::io::Write;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2;
use sdl2::Sdl;
use sdl2::render::{Renderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::pixels::{Color, PixelFormatEnum};

use command::*;

fn log_error<D: Display>(d: D) {
    writeln!(&mut std::io::stderr(), "{}", d).ok();
}

fn clamp<T>(v: T, lower: T, upper: T) -> T where T: std::cmp::Ord {
    std::cmp::min(upper, std::cmp::max(lower, v))
}

struct Context {
    sdl: Sdl,
    renderer: Renderer<'static>
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct State {
    x: f64,
    y: f64,
    pen: bool,
    color: Color,
    angle: f64
}

#[derive(Clone)]
pub struct Turtle {
    ctx: Rc<RefCell<Context>>,
    state: State,
    saved: Vec<State>
}

impl Turtle {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Turtle, Box<Error>> {
        let sdl_ctx = sdl2::init()?;
        let vsub = sdl_ctx.video()?;

        let window = vsub.window(title, width, height)
            .position_centered()
            .opengl()
            .build()?;

        let mut renderer = window.renderer().target_texture().build()?;

        let _ = renderer.render_target().unwrap()
            .create_and_set(PixelFormatEnum::RGBA8888, width, height);
        renderer.set_draw_color(Color::RGBA(0, 0, 0, 0));
        renderer.clear();
        Ok(Turtle {
            ctx: Rc::new(RefCell::new(Context {
                sdl: sdl_ctx,
                renderer: renderer
            })),
            state: State {
                x: 0.0,
                y: 0.0,
                pen: true,
                color: Color::RGB(255, 255, 255),
                angle: 0.0,
            },
            saved: vec![]
        })
    }

    pub fn x(&self) -> f64 {
        self.state.x
    }

    pub fn y(&self) -> f64 {
        self.state.y
    }

    pub fn angle(&self) -> f64 {
        f64::to_degrees(self.state.angle)
    }

    pub fn r(&self) -> u8 {
        match self.state.color {
            Color::RGB(r, _, _) | Color::RGBA(r, _, _, _) => r
        }
    }

    pub fn g(&self) -> u8 {
        match self.state.color {
            Color::RGB(_, g, _) | Color::RGBA(_, g, _, _) => g
        }
    }

    pub fn b(&self) -> u8 {
        match self.state.color {
            Color::RGB(_, _, b) | Color::RGBA(_, _, b, _) => b
        }
    }

    pub fn mouse_x(&self) -> f64 {
        let ctx = &mut *self.ctx.borrow_mut();
        let (_, x, _) = ctx.sdl.mouse().mouse_state();
        x as f64
    }

    pub fn mouse_y(&self) -> f64 {
        let ctx = &mut *self.ctx.borrow_mut();
        let (_, _, y) = ctx.sdl.mouse().mouse_state();
        y as f64
    }

    pub fn mouse_left(&self) -> bool {
        let ctx = &mut *self.ctx.borrow_mut();
        let (st, _, _) = ctx.sdl.mouse().relative_mouse_state();
        st.left()
    }

    pub fn mouse_right(&self) -> bool {
        let ctx = &mut *self.ctx.borrow_mut();
        let (st, _, _) = ctx.sdl.mouse().relative_mouse_state();
        st.right()
    }

    pub fn update(&mut self, delay: f64) -> bool {
        let ctx = &mut *self.ctx.borrow_mut();
        let target = ctx.renderer.render_target().unwrap().reset().unwrap().unwrap();

        ctx.renderer.set_draw_color(Color::RGB(0, 0, 0));
        ctx.renderer.clear();
        ctx.renderer.copy(&target, None, None).
            unwrap_or_else(log_error);
        ctx.renderer.present();

        let mut event_pump = ctx.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                },
                _ => {}
            }
        }
        if delay > 0.0 {
            let msdelay = (delay * 1000.0) as u32;
            ctx.sdl.timer().unwrap().delay(msdelay);
        }

        let _ = ctx.renderer.render_target().unwrap().set(target);
        true
    }

    pub fn running(&mut self) -> bool {
        self.update(0.0)
    }

    pub fn run(&mut self) {
        while self.running() {}
    }

    pub fn clear(&mut self) {
        let ctx = &mut *self.ctx.borrow_mut();
        ctx.renderer.set_draw_color(Color::RGB(0, 0, 0));
        ctx.renderer.clear();
    }

    pub fn reset(&mut self) {
        let s = &mut self.state;
        s.x = 0.0;
        s.y = 0.0;
        s.pen = true;
        s.angle = 0.0;
    }

    pub fn forward(&mut self, by: f64) {
        let ox = f64::cos(self.state.angle) * by;
        let oy = f64::sin(self.state.angle) * by;
        let (nx, ny) = (self.state.x + ox, self.state.y + oy);
        let ref mut rend = self.ctx.borrow_mut().renderer;

        if self.state.pen {
            rend.set_draw_color(self.state.color);
            rend.draw_line(
                Point::new(self.state.x as i32, self.state.y as i32),
                Point::new(nx as i32, ny as i32))
                    .unwrap_or_else(log_error);
        }
        self.state.x = nx;
        self.state.y = ny;
    }

    pub fn backward(&mut self, by: f64) {
        let ox = f64::cos(self.state.angle) * by;
        let oy = f64::sin(self.state.angle) * by;
        let (nx, ny) = (self.state.x - ox, self.state.y - oy);
        let ref mut rend = self.ctx.borrow_mut().renderer;

        if self.state.pen {
            rend.set_draw_color(self.state.color);
            rend.draw_line(
                Point::new(self.state.x as i32, self.state.y as i32),
                Point::new(nx as i32, ny as i32))
                    .unwrap_or_else(log_error);
        }
        self.state.x = nx;
        self.state.y = ny;
    }

    pub fn left(&mut self, by: f64) {
        self.rotate(-by);
    }

    pub fn right(&mut self, by: f64) {
        self.rotate(by);
    }

    pub fn rotate(&mut self, by: f64) {
        self.state.angle += f64::to_radians(by);
        self.state.angle %= std::f64::consts::PI * 2.0;
    }

    pub fn face(&mut self, at: f64) {
        self.state.angle = f64::to_radians(at);
        self.state.angle %= std::f64::consts::PI * 2.0;
    }

    pub fn pen(&mut self, down: bool) {
        self.state.pen = down;
    }

    pub fn color(&mut self, r: u8, g: u8, b: u8) {
        self.state.color = Color::RGB(r, g, b)
    }

    pub fn blend_mul(&mut self, r: f64, g: f64, b: f64) {
        match self.state.color {
            Color::RGB(rr, gg, bb) | Color::RGBA(rr, gg, bb, _) => {
                self.state.color = Color::RGB(
                    f64::min((rr as f64) * r, 255.0) as u8,
                    f64::min((gg as f64) * g, 255.0) as u8,
                    f64::min((bb as f64) * b, 255.0) as u8,
                );
            }
        }
    }

    pub fn blend_div(&mut self, r: f64, g: f64, b: f64) {
        match self.state.color {
            Color::RGB(rr, gg, bb) | Color::RGBA(rr, gg, bb, _) => {
                self.state.color = Color::RGB(
                    f64::min((rr as f64) / r, 255.0) as u8,
                    f64::min((gg as f64) / g, 255.0) as u8,
                    f64::min((bb as f64) / b, 255.0) as u8,
                );
            }
        }
    }

    pub fn blend_add(&mut self, r: u8, g: u8, b: u8) {
        match self.state.color {
            Color::RGB(rr, gg, bb) | Color::RGBA(rr, gg, bb, _) => {
                self.state.color = Color::RGB(
                    clamp(rr as i32 + r as i32, 0, 255) as u8,
                    clamp(gg as i32 + g as i32, 0, 255) as u8,
                    clamp(bb as i32 + b as i32, 0, 255) as u8,
                );
            }
        }
    }

    pub fn blend_sub(&mut self, r: u8, g: u8, b: u8) {
        match self.state.color {
            Color::RGB(rr, gg, bb) | Color::RGBA(rr, gg, bb, _) => {
                self.state.color = Color::RGB(
                    clamp(rr as i32 - r as i32, 0, 255) as u8,
                    clamp(gg as i32 - g as i32, 0, 255) as u8,
                    clamp(bb as i32 - b as i32, 0, 255) as u8,
                );
            }
        }
    }

    pub fn goto(&mut self, x: f64, y: f64) {
        self.state.x = x;
        self.state.y = y;
    }

    pub fn push(&mut self) {
        self.saved.push(self.state);
    }

    pub fn pop(&mut self) {
        self.state = self.saved.pop().unwrap();
    }

    pub fn execute(&mut self, cmd: Command) {
        use Command::*;
        match cmd {
            Noop => {},
            Reset => self.reset(),
            Clear => self.clear(),
            Call(f) => f(self),
            Goto(x, y) => self.goto(x, y),
            Forward(by) => self.forward(by),
            Backward(by) => self.backward(by),
            Left(by) => self.left(by),
            Right(by) => self.right(by),
            Rotate(by) => self.rotate(by),
            Face(at) => self.face(at),
            Pen(down) => self.pen(down),
            Color(r, g, b) => self.color(r, g, b),
            BlendMul(r, g, b) => self.blend_mul(r, g, b),
            BlendDiv(r, g, b) => self.blend_div(r, g, b),
            BlendAdd(r, g, b) => self.blend_add(r, g, b),
            BlendSub(r, g, b) => self.blend_sub(r, g, b),
            PushState => self.push(),
            PopState => self.pop(),
            _ => {},
        }
    }
}
