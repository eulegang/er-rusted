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

/*
#[derive(Debug)]
pub enum Command {
    /// Add input after a line
    Append(String),

    /// Add input before a line
    Insert(String),

    /// Delete and replace with string
    Change(String),

    /// Delete a range of text
    Delete,

    /// Join lines
    Join,

    /// Mark a line
    Mark(char),

    /// Move lines
    Move(Offset),

    /// Read file into buffer
    Read(String),

    /// Subst line
    Substitute(Re, String),

    /// Copy region to line
    Copy(Offset),

    /// Write text region to file
    Write(String),
}

impl Command {
    pub fn default_address(&self) -> Address {
        match self {
            Command::Append(_) | Command::Insert(_) => Address::Line(Default::default()),

            Command::Delete
            | Command::Change(_)
            | Command::Move(_)
            | Command::Substitute(_, _)
            | Command::Copy(_) => Address::Range {
                start: Default::default(),
                end: Default::default(),
            },

            Command::Join => Address::Range {
                start: Offset::Nil(Point::Current),
                end: Offset::Relf(Point::Current, 1),
            },

            Command::Mark(_) => Address::Line(Default::default()),
            Command::Read(_) => Address::Line(Offset::Nil(Point::Last)),
            Command::Write(_) => Address::Range {
                start: Offset::Nil(Point::Abs(1)),
                end: Offset::Nil(Point::Last),
            },
        }
    }
}*/
