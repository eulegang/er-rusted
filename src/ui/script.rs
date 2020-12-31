use super::UI;
use crate::{cmd::Command, interp::Interpreter};
use eyre::{bail, WrapErr};
use std::fs::read_to_string;

/// Run an er script
pub struct Script {
    files: Vec<String>,
    commands: Vec<Command>,
}

impl Script {
    /// Create a script with a filename and run it against files
    pub fn from_file(script: &str, files: Vec<String>) -> eyre::Result<Self> {
        let content = read_to_string(script).wrap_err("failed to read script")?;

        let commands = match Command::from_content(&content) {
            Ok(commands) => commands,
            Err((line, pos)) => bail!("line {} failed to parse command: {}", pos, line),
        };

        for cmd in &commands {
            if cmd.needs_text() {
                bail!("{:?} needs text that was not provided", cmd)
            }
        }

        Ok(Script { commands, files })
    }
}

impl UI for Script {
    fn run(&mut self) -> eyre::Result<()> {
        'files: for file in &self.files {
            let mut interp = Interpreter::new(vec![file.clone()]).unwrap();

            for cmd in &self.commands {
                match interp.exec(cmd) {
                    Err(()) => {
                        eprintln!("failed to exec on {}", file);
                        continue 'files;
                    }
                    Ok(true) => (),
                    Ok(false) => continue 'files,
                }
            }

            if let Err(_) = interp.ensure_clean() {
                eprintln!("failed to write back to {}", file);
            }
        }

        Ok(())
    }
}
