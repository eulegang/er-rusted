use crate::{
    addr::{Address, Offset},
    re::{Pat, Re},
};

use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command as Cmd, Stdio};

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

    Subst(Address, Option<Re>, Option<Pat>, Option<SubstFlags>),

    Yank(Address),
    Paste(Offset),

    Write(Address, Sink),

    Quit,

    Append(Offset),
    Insert(Offset),
    Change(Address),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SubstFlags {
    pub print: bool,
    pub occurances: usize,
}

#[derive(Debug, PartialEq)]
pub enum Sink {
    Filename,
    File(String),
    Command(String),
}

impl Command {
    pub fn needs_text(&self) -> bool {
        use Command::*;

        matches!(self, Append(_) | Insert(_) | Change(_))
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

impl Sink {
    pub fn sink_lines(&self, filename: Option<&str>, lines: &[String]) -> CommandResult {
        match self {
            Sink::Filename => {
                if let Some(filename) = filename {
                    if let Ok(mut file) = OpenOptions::new()
                        .truncate(true)
                        .write(true)
                        .create(true)
                        .open(filename)
                    {
                        for line in lines {
                            if let Err(_) = writeln!(file, "{}", line) {
                                return CommandResult::Failed;
                            }
                        }

                        CommandResult::Success
                    } else {
                        CommandResult::Failed
                    }
                } else {
                    CommandResult::Failed
                }
            }

            Sink::File(name) => {
                if let Ok(mut file) = OpenOptions::new()
                    .truncate(true)
                    .write(true)
                    .create(true)
                    .open(name)
                {
                    for line in lines {
                        if let Err(_) = writeln!(file, "{}", line) {
                            return CommandResult::Failed;
                        }
                    }

                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Sink::Command(command) => {
                let rchild = Cmd::new("sh")
                    .arg("-c")
                    .arg(replace_file(command, filename))
                    .stdin(Stdio::piped())
                    .spawn();

                if let Ok(mut child) = rchild {
                    let mut stdin = child.stdin.take().unwrap();
                    for line in lines {
                        if let Err(_) = writeln!(stdin, "{}", line) {
                            return CommandResult::Failed;
                        }
                    }

                    if matches!(child.wait(), Err(_)) {
                        return CommandResult::Failed;
                    };

                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }
        }
    }
}

pub fn replace_file(expr: &str, filename: Option<&str>) -> String {
    let mut buf = String::with_capacity(expr.len());
    let mut toggle = false;

    for ch in expr.chars() {
        match (toggle, ch) {
            (false, '\\') => toggle = true,
            (true, '\\') => {
                buf.push(ch);
                toggle = false;
            }

            (false, '%') => {
                buf.push_str(filename.unwrap_or(""));
            }

            (true, '%') => {
                buf.push('%');
            }

            (false, otherwise) => buf.push(otherwise),
            (true, otherwise) => {
                buf.push('\\');
                buf.push(otherwise);
            }
        }
    }

    buf
}
