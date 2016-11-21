
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Command {
    Noop,
    Reset,
    Clear,
    Goto(f64, f64),
    Forward(f64),
    Backward(f64),
    Left(f64),
    Right(f64),
    Rotate(f64),
    Face(f64),
    Pen(bool),
    Color(u8, u8, u8),
    BlendMul(f64, f64, f64),
    BlendDiv(f64, f64, f64),
    BlendAdd(u8, u8, u8),
    BlendSub(u8, u8, u8),
    PushState,
    PopState,

    Marker(u32)
}
