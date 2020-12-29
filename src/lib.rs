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
mod syspoint;
pub mod ui;

pub use addr::{Address, Offset, Point};
pub use buffer::Buffer;
pub use cmd::{Command, SubstFlags};
pub use interp::Interpreter;
pub use re::{Expansion, Pat, Re};
pub use syspoint::SysPoint;

pub(crate) use parse::Parsable;

pub(crate) const VALID_MARKS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ<>_";
