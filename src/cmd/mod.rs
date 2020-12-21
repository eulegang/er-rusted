use crate::{
    addr::{Address, Offset},
    buffer::chomp,
    interp::Interpreter,
    re::{Pat, Re},
};

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command as SysCmd, Stdio};

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

    Write(Address, SysPoint, bool),
    Read(Offset, SysPoint),
    Run(Cmd),

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
pub enum SysPoint {
    Filename,
    File(String),
    Command(Cmd),
}

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Repeat,
    System(String),
}

pub trait Syncer {
    fn sync(&self, interp: &Interpreter, lines: &[String]) -> CommandResult;
}

pub trait Sourcer {
    fn source(&self, interp: &Interpreter) -> Result<Vec<String>, CommandResult>;
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

impl Syncer for SysPoint {
    fn sync(&self, interp: &Interpreter, lines: &[String]) -> CommandResult {
        fn sync_file(name: &str, lines: &[String]) -> CommandResult {
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

        match self {
            SysPoint::Filename => {
                if let Some(filename) = &interp.filename {
                    sync_file(filename, lines)
                } else {
                    CommandResult::Failed
                }
            }

            SysPoint::File(name) => sync_file(name, lines),
            SysPoint::Command(command) => command.sync(interp, lines),
        }
    }
}

impl Sourcer for SysPoint {
    fn source(&self, interp: &Interpreter) -> Result<Vec<String>, CommandResult> {
        fn src_file(filename: &str) -> Result<Vec<String>, CommandResult> {
            if let Ok(file) = OpenOptions::new().read(true).open(filename) {
                let mut reader = BufReader::new(file);
                let mut buffer = String::new();
                let mut lines = Vec::new();
                loop {
                    let read = reader.read_line(&mut buffer);

                    match read {
                        Ok(0) => break Ok(lines),
                        Err(_) => break Err(CommandResult::Failed),
                        _ => {
                            chomp(&mut buffer);
                            lines.push(buffer);
                            buffer = String::new();
                        }
                    }
                }
            } else {
                Err(CommandResult::Failed)
            }
        }

        match self {
            SysPoint::Filename => {
                if let Some(filename) = &interp.filename {
                    src_file(filename)
                } else {
                    Err(CommandResult::Failed)
                }
            }

            SysPoint::File(file) => src_file(file),
            SysPoint::Command(command) => command.source(interp),
        }
    }
}

impl Syncer for Cmd {
    fn sync(&self, interp: &Interpreter, lines: &[String]) -> CommandResult {
        let cmd = if let Some(cmd) =
            self.replace_filename(interp.filename.as_deref(), interp.last_wcmd.as_deref())
        {
            cmd
        } else {
            return CommandResult::Failed;
        };

        let rchild = SysCmd::new("sh")
            .arg("-c")
            .arg(cmd)
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

impl Sourcer for Cmd {
    fn source(&self, interp: &Interpreter) -> Result<Vec<String>, CommandResult> {
        let cmd = if let Some(cmd) =
            self.replace_filename(interp.filename.as_deref(), interp.last_rcmd.as_deref())
        {
            cmd
        } else {
            return Err(CommandResult::Failed);
        };

        let rchild = SysCmd::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .spawn();

        if let Ok(mut child) = rchild {
            let stdout = child.stdout.take().unwrap();
            let mut reader = BufReader::new(stdout);
            let mut buffer = String::new();
            let mut lines = Vec::new();

            let lines = loop {
                let read = reader.read_line(&mut buffer);

                match read {
                    Ok(0) => break Ok(dbg!(lines)),
                    Err(_) => break Err(CommandResult::Failed),
                    _ => {
                        chomp(&mut buffer);
                        lines.push(buffer);
                        buffer = String::new();
                    }
                }
            };

            if matches!(child.wait(), Err(_)) {
                return Err(CommandResult::Failed);
            } else {
                lines
            }
        } else {
            Err(CommandResult::Failed)
        }
    }
}

impl Cmd {
    fn replace_filename(
        &self,
        filename: Option<&str>,
        prev_command: Option<&str>,
    ) -> Option<String> {
        let expr = match (self, prev_command) {
            (Cmd::System(expr), _) => expr,
            (Cmd::Repeat, Some(prev)) => prev,
            _ => return None,
        };

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

        Some(buf)
    }

    fn run(&self, interp: &Interpreter) -> CommandResult {
        let cmd = if let Some(cmd) =
            self.replace_filename(interp.filename.as_deref(), interp.last_cmd.as_deref())
        {
            cmd
        } else {
            return CommandResult::Failed;
        };

        let status = SysCmd::new("sh").arg("-c").arg(cmd).status();

        if status.map_or(false, |s| s.success()) {
            CommandResult::Success
        } else {
            CommandResult::Failed
        }
    }
}
