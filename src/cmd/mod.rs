use crate::{
    addr::{Address, Offset},
    re::{Pat, Re},
    syspoint::{Cmd, Sourcer, Syncer, SysPoint},
    Buffer,
};

mod action;

#[cfg(test)]
mod test;

/// A command to run on a buffer
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Command {
    /// Print lines within an address
    Print(Address),

    /// Delete the lines within an address
    Delete(Address),

    /// Move the cursor to the new point
    Nop(Offset),
    /// Mark a given offset with a given character
    Mark(Offset, char),
    /// Join the lines over an address into one line
    Join(Address),
    /// Move a set of lines to a point in the buffer
    Move(Address, Offset),
    /// Copy a set of lines to a point in the bufffer
    Transfer(Address, Offset),

    /// Substitue a regex for a pattern in a set of lines
    Subst(Address, Option<Re>, Option<Pat>, Option<SubstFlags>),

    /// Yank a set of lines into the cut buffer
    Yank(Address),
    /// Paste the cut buffer into a point in the buffer
    Paste(Offset),

    /// Write a set of lines to a syspoint and optionally quit
    Write(Address, SysPoint, bool),
    /// Read the contents of a syspoint and put them at a point in the buffer
    Read(Offset, SysPoint),
    /// Run a system command
    Run(Cmd),

    /// Search for a set (non contigous) of lines that match a regex and run commands with them
    Global(Address, Option<Re>, Vec<Command>),
    /// Search for a set (non contigous) of lines that do not match a regex and run commands with them
    Void(Address, Option<Re>, Vec<Command>),

    /// Load the next file and set that as the buffer
    NextBuffer,
    /// Load the previous file and set that as the buffer
    PrevBuffer,

    /// Quits the interpreter
    Quit,

    /// Append a set of lines after a point in the buffer
    Append(Offset, Option<Vec<String>>),
    /// Insert a set of lines before a point in the buffer
    Insert(Offset, Option<Vec<String>>),
    /// Change s set of lines to a different set of lines
    Change(Address, Option<Vec<String>>),
}

/// Additional flags to the subst command
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SubstFlags {
    /// Whether or not to print the line after substitutions
    pub print: bool,
    /// The number of substitutions to make.  (0 is inifite)
    pub occurances: usize,
}

impl Command {
    /// Append, Insert, and Change need text to operate but may be input without the text to
    /// operate with.  This method tells if the command still needs text to operate.
    pub fn needs_text(&self) -> bool {
        use Command::*;

        matches!(self, Append(_, None) | Insert(_, None) | Change(_, None))
    }

    /// Injects text into a command
    ///
    /// Only works if `cmd.needs_text()` returns true otherwise it panics.
    pub fn inject(&mut self, lines: Vec<String>) {
        use std::mem::take;

        match self {
            Command::Append(line, None) => *self = Command::Append(take(line), Some(lines)),
            Command::Insert(line, None) => *self = Command::Insert(take(line), Some(lines)),
            Command::Change(addr, None) => *self = Command::Change(take(addr), Some(lines)),

            _ => panic!("can not inject text into non aci commands"),
        };
    }
}

impl Default for SubstFlags {
    fn default() -> SubstFlags {
        SubstFlags {
            print: false,
            occurances: 1,
        }
    }
}
