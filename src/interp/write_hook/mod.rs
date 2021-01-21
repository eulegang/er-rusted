use crate::Buffer;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq)]
pub enum WriteHook {
    Id,
    Proc(String),
}

impl Default for WriteHook {
    fn default() -> WriteHook {
        WriteHook::Id
    }
}

impl WriteHook {
    pub fn sync(&self, name: &str, buffer: &mut Buffer, lines: &[String]) -> bool {
        let mut file = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(name)
        {
            Ok(f) => f,
            Err(_) => return false,
        };

        match self {
            WriteHook::Id => {
                for line in lines {
                    if let Err(_) = writeln!(file, "{}", line) {
                        return false;
                    }
                }

                true
            }

            WriteHook::Proc(cmd) => {
                let mut child = match Command::new(cmd)
                    .stdin(Stdio::piped())
                    .stdout(file)
                    .arg(name)
                    .spawn()
                {
                    Ok(child) => child,
                    Err(_) => return false,
                };

                let mut stdin = child.stdin.take().unwrap();

                for line in lines {
                    if let Err(_) = writeln!(stdin, "{}", line) {
                        return false;
                    }
                }

                drop(stdin);

                if !child.wait().is_ok() {
                    return false;
                }

                let file = match File::open(name) {
                    Ok(file) => file,
                    Err(_) => return false,
                };

                buffer.load(file).is_ok()
            }
        }
    }
}
