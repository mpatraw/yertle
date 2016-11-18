#![crate_name = "yertle"]
#![crate_type = "lib"]

extern crate sdl2;

pub mod command;
pub mod lsystem;
pub mod turtle;
pub mod vm;
pub use command::*;
pub use lsystem::*;
pub use turtle::*;
pub use vm::*;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        println!("hello world");
    }
}
