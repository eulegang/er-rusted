mod addr;
mod cmd;

pub use addr::{Address, Offset, Point};
pub use cmd::Command;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Op {
    range: Address,
    cmd: Command,
}
