use super::{Address, Offset, Point};
use crate::re::Re;

mod parser;

#[cfg(test)]
mod test;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Command {
    Print(Address),
    Delete(Address),
    Nop(Offset),
}
