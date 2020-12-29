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
mod parse;
mod re;
mod resolve;
mod ui;

pub use buffer::Buffer;
pub use interp::Interpreter;
pub(crate) use parse::Parsable;
pub use ui::{Repl, UI};

pub(crate) const VALID_MARKS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ<>_";
