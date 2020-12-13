use super::*;

use crate::{
    ed::{Command, CommandResult},
    Interpreter,
};
use eyre::WrapErr;
use rustyline::{error::ReadlineError, Editor, Helper};
use std::str::FromStr;

pub struct Repl {
    interp: Interpreter,
}

impl UI for Repl {
    fn new(files: Vec<String>) -> eyre::Result<Self> {
        let interp = Interpreter::new(files).wrap_err("failed to build")?;

        Ok(Repl { interp })
    }

    fn run(&mut self) -> eyre::Result<()> {
        let mut rl = Editor::<()>::new();

        loop {
            let line = match self.read_line(&mut rl) {
                Ok(line) => line,
                Err(LineHandling::Next) => continue,
                Err(LineHandling::Quit) => break,
                Err(LineHandling::InvalidCommand) => continue,
                Err(LineHandling::InvalidInvocation) => continue,
            };

            match self.process_line(&line, &mut rl) {
                LineHandling::Next => continue,
                LineHandling::Quit => break,
                LineHandling::InvalidCommand => continue,
                LineHandling::InvalidInvocation => continue,
            }
        }

        Ok(())
    }
}

enum LineHandling {
    Next,
    InvalidCommand,
    InvalidInvocation,
    Quit,
}

impl Repl {
    fn read_line<T: Helper>(&self, rl: &mut Editor<T>) -> Result<String, LineHandling> {
        use LineHandling::*;

        match rl.readline(":") {
            Ok(line) => Ok(line),
            Err(ReadlineError::Interrupted) => Err(Next),
            Err(ReadlineError::Eof) => Err(Quit),
            Err(err) => {
                eprintln!("err: {:?}", err);
                Err(InvalidInvocation)
            }
        }
    }

    fn process_line<T: Helper>(&mut self, line: &str, rl: &mut Editor<T>) -> LineHandling {
        use LineHandling::*;

        let cmd = match Command::from_str(&line) {
            Ok(cmd) => cmd,
            Err(_) => return InvalidCommand,
        };

        let result = if cmd.needs_text() {
            let lines = match self.read_text_mode(rl) {
                Ok(lines) => lines,
                Err(ReadlineError::Interrupted) => return Next,
                Err(ReadlineError::Eof) => return InvalidCommand,
                Err(err) => {
                    eprintln!("< error: {:?}", err);
                    return InvalidCommand;
                }
            };

            self.interp.exec_with_text(cmd, lines)
        } else {
            self.interp.exec(cmd)
        };

        match result {
            CommandResult::Failed => InvalidInvocation,
            CommandResult::Success => Next,
            CommandResult::Quit => Quit,
        }
    }

    fn read_text_mode<T: Helper>(&self, rl: &mut Editor<T>) -> Result<Vec<String>, ReadlineError> {
        let mut buf = Vec::new();
        loop {
            let line = rl.readline("")?;

            if line.as_str() == "." {
                break;
            }

            buf.push(line);
        }

        Ok(buf)
    }
}
