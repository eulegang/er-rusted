use super::UI;
use crate::{
    ed::cmd::Command,
    interp::{scratch::StdoutScratchPad, Interpreter},
};
use eyre::{bail, WrapErr};
use std::fs::{copy, read_to_string};

/// Run an er script
pub struct Script {
    files: Vec<String>,
    backup: Option<String>,
    commands: Vec<Command>,
}

impl Script {
    /// Create a script with a filename and run it against files
    pub fn from_file(
        script: &str,
        backup: Option<String>,
        files: Vec<String>,
    ) -> eyre::Result<Self> {
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

        Ok(Script {
            commands,
            backup,
            files,
        })
    }

    /// Create a script from an expression and run it against files
    pub fn from_expr(
        exprs: Vec<String>,
        backup: Option<String>,
        files: Vec<String>,
    ) -> eyre::Result<Self> {
        let mut commands = Vec::new();
        for expr in exprs {
            match Command::from_expr(&expr) {
                Ok(cmds) => commands.extend(cmds),
                Err(_) => bail!("{} is an invalid expression", expr),
            }
        }

        Ok(Script {
            commands,
            backup,
            files,
        })
    }
}

impl UI for Script {
    fn run(&mut self) -> eyre::Result<()> {
        'files: for file in &self.files {
            let mut interp = Interpreter::new::<StdoutScratchPad>(vec![file.clone()]).unwrap();

            if let Some(backup) = &self.backup {
                if let Err(e) = copy(&file, format!("{}.{}", file, backup)) {
                    eprintln!("failed to backup {}: {}", file, e);
                    continue;
                }
            }

            for cmd in &self.commands {
                match interp.exec(cmd) {
                    Err(err) => {
                        eprintln!("{} failed to exec on {}", err, file);
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
