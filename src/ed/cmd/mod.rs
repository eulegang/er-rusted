use super::{Address, Offset, Point};
use crate::re::{Pat, Re};

mod action;
mod parser;

#[cfg(test)]
mod test;

pub enum CommandResult {
    Success,
    Failed,
    Quit,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Command {
    Print(Address),
    Delete(Address),
    Nop(Offset),
    Mark(Offset, char),
    Join(Address),
    Move(Address, Offset),
    Transfer(Address, Offset),

    Subst(Address, Option<Re>, Option<Pat>), // TODO: add flags

    Yank(Address),
    Paste(Offset),

    Quit,

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
