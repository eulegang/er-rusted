use crate::Buffer;
use std::fs::{copy, File, OpenOptions};
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

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
        match self {
            WriteHook::Id => {
                let mut file = match OpenOptions::new()
                    .truncate(true)
                    .write(true)
                    .create(true)
                    .open(name)
                {
                    Ok(f) => f,
                    Err(_) => return false,
                };

                for line in lines {
                    if let Err(_) = writeln!(file, "{}", line) {
                        return false;
                    }
                }

                true
            }

            WriteHook::Proc(cmd) => {
                let temp = match NamedTempFile::new() {
                    Ok(f) => f,
                    Err(_) => return false,
                };

                let (tfile, tpath) = temp.into_parts();

                let mut child = match Command::new(cmd)
                    .stdin(Stdio::piped())
                    .stdout(tfile)
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

                let status = match child.wait() {
                    Ok(s) => s,
                    Err(_) => return false,
                };

                if !status.success() {
                    return false;
                }

                if let Err(_) = copy(tpath, name) {
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
