#[cfg(test)]
macro_rules! re {
    ($regex: expr) => {
        crate::re::Re::from_str($regex).unwrap()
    };
}

mod addr;
mod buffer;
mod cmd;
mod interp;
mod re;
mod ui;

pub use buffer::Buffer;
pub use interp::Interpreter;
pub use ui::{Repl, UI};

pub(crate) const VALID_MARKS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ<>_";

pub(crate) trait Parsable: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}
