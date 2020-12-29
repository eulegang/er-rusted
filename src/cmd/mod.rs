use crate::{
    addr::{Address, Offset},
    re::{Pat, Re},
    syspoint::{Cmd, Sourcer, Syncer, SysPoint},
    Buffer,
};

mod action;

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
    Move(Address, Offset),
    Transfer(Address, Offset),

    Subst(Address, Option<Re>, Option<Pat>, Option<SubstFlags>),

    Yank(Address),
    Paste(Offset),

    Write(Address, SysPoint, bool),
    Read(Offset, SysPoint),
    Run(Cmd),

    Global(Address, Option<Re>, Vec<Command>),
    Void(Address, Option<Re>, Vec<Command>),

    NextBuffer,
    PrevBuffer,

    Quit,

    Append(Offset, Option<Vec<String>>),
    Insert(Offset, Option<Vec<String>>),
    Change(Address, Option<Vec<String>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SubstFlags {
    pub print: bool,
    pub occurances: usize,
}

impl Command {
    pub fn needs_text(&self) -> bool {
        use Command::*;

        matches!(self, Append(_, None) | Insert(_, None) | Change(_, None))
    }

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
