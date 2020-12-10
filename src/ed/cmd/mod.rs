use super::{Address, Offset, Point};

mod action;
mod parser;

#[cfg(test)]
mod test;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Command {
    Print(Address),
    Delete(Address),
    Nop(Offset),
    Mark(Offset, char),
    Join(Address),

    Append(Offset),
    Insert(Offset),
    Change(Address),
}

impl Command {
    pub fn needs_text(&self) -> bool {
        use Command::*;

        matches!(self, Append(_) | Insert(_) | Change(_))
    }
}
