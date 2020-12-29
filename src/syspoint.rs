use crate::{
    buffer::{chomp, Buffer},
    interp::Env,
};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command as SysCmd, Stdio};

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
    fn sync(&self, buffer: &mut Buffer, env: &Env, lines: &[String]) -> bool;
}

pub trait Sourcer {
    fn source(&self, buffer: &Buffer, env: &Env) -> Option<Vec<String>>;
}

impl Syncer for SysPoint {
    fn sync(&self, buffer: &mut Buffer, env: &Env, lines: &[String]) -> bool {
        fn sync_file(name: &str, lines: &[String]) -> bool {
            if let Ok(mut file) = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(name)
            {
                for line in lines {
                    if let Err(_) = writeln!(file, "{}", line) {
                        return false;
                    }
                }

                true
            } else {
                false
            }
        }

        match self {
            SysPoint::Filename => {
                if let Some(filename) = &env.filename {
                    sync_file(filename, lines)
                } else {
                    false
                }
            }

            SysPoint::File(name) => sync_file(name, lines),
            SysPoint::Command(command) => command.sync(buffer, env, lines),
        }
    }
}

impl Sourcer for SysPoint {
    fn source(&self, buffer: &Buffer, env: &Env) -> Option<Vec<String>> {
        fn src_file(filename: &str) -> Option<Vec<String>> {
            if let Ok(file) = OpenOptions::new().read(true).open(filename) {
                let mut reader = BufReader::new(file);
                let mut buffer = String::new();
                let mut lines = Vec::new();
                loop {
                    let read = reader.read_line(&mut buffer);

                    match read {
                        Ok(0) => break Some(lines),
                        Err(_) => break None,
                        _ => {
                            chomp(&mut buffer);
                            lines.push(buffer);
                            buffer = String::new();
                        }
                    }
                }
            } else {
                None
            }
        }

        match self {
            SysPoint::Filename => {
                if let Some(filename) = &env.filename {
                    src_file(filename)
                } else {
                    None
                }
            }

            SysPoint::File(file) => src_file(file),
            SysPoint::Command(command) => command.source(buffer, env),
        }
    }
}

impl Syncer for Cmd {
    fn sync(&self, _: &mut Buffer, env: &Env, lines: &[String]) -> bool {
        let cmd = if let Some(cmd) =
            self.replace_filename(env.filename.as_deref(), env.last_wcmd.as_deref())
        {
            cmd
        } else {
            return false;
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
                    return false;
                }
            }

            if matches!(child.wait(), Err(_)) {
                return false;
            }

            true
        } else {
            false
        }
    }
}

impl Sourcer for Cmd {
    fn source(&self, _: &Buffer, env: &Env) -> Option<Vec<String>> {
        let cmd = if let Some(cmd) =
            self.replace_filename(env.filename.as_deref(), env.last_rcmd.as_deref())
        {
            cmd
        } else {
            return None;
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
                    Ok(0) => break Some(lines),
                    Err(_) => break None,
                    _ => {
                        chomp(&mut buffer);
                        lines.push(buffer);
                        buffer = String::new();
                    }
                }
            };

            if matches!(child.wait(), Err(_)) {
                None
            } else {
                lines
            }
        } else {
            None
        }
    }
}

impl Cmd {
    pub(crate) fn replace_filename(
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

    pub(crate) fn run(&self, env: &Env) -> bool {
        let cmd = if let Some(cmd) =
            self.replace_filename(env.filename.as_deref(), env.last_cmd.as_deref())
        {
            cmd
        } else {
            return false;
        };

        let status = SysCmd::new("sh").arg("-c").arg(cmd).status();

        if status.map_or(false, |s| s.success()) {
            true
        } else {
            false
        }
    }
}