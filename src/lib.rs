//!
//! Text editing like the olde times
//!

#![warn(missing_docs)]

#[cfg(test)]
macro_rules! re {
    ($regex: expr) => {
        crate::ed::re::Re::from_str($regex).unwrap()
    };
}

mod buffer;
pub mod ed;
mod interp;
pub mod ui;

pub use buffer::Buffer;
pub use interp::Interpreter;

pub(crate) const VALID_MARKS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ<>_";
