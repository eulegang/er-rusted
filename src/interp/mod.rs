use crate::{
    cmd::{Command, SubstFlags},
    re::{Pat, Re},
    Buffer,
};

use std::fs::File;
use std::io::{self, ErrorKind};

/// Interprets commands on a buffer
pub struct Interpreter {
    pub(crate) filelist: Vec<String>,
    pub(crate) filepos: usize,
    pub(crate) buffer: Buffer,
    pub(crate) env: Env,
}

pub struct Env {
    pub(crate) cut: Vec<String>,
    pub(crate) filename: Option<String>,

    pub(crate) last_re: Option<Re>,
    pub(crate) last_pat: Option<Pat>,
    pub(crate) last_flags: Option<SubstFlags>,

    pub(crate) last_cmd: Option<String>,
    pub(crate) last_rcmd: Option<String>,
    pub(crate) last_wcmd: Option<String>,
}

impl Interpreter {
    /// Creates an interpreter with multiple files in it's arglisT
    pub fn new(files: Vec<String>) -> io::Result<Interpreter> {
        let (filename, buffer) = if let Some(file) = files.get(0) {
            let buffer = match File::open(file) {
                Ok(f) => Buffer::read(f)?,
                Err(e) if e.kind() == ErrorKind::NotFound => Buffer::default(),
                Err(e) => Err(e)?,
            };
            (Some(file.clone()), buffer)
        } else {
            (None, Buffer::default())
        };

        let mut env = Env::default();
        env.filename = filename;

        let filelist = files;
        let filepos = 0;

        Ok(Interpreter {
            filelist,
            filepos,
            buffer,
            env,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_reader<R: std::io::Read>(r: R) -> io::Result<Interpreter> {
        let buffer = Buffer::read(r)?;
        let env = Env::default();

        let filelist = vec![];
        let filepos = 0;

        Ok(Interpreter {
            filelist,
            filepos,
            buffer,
            env,
        })
    }

    /// Executes a command on the given buffer
    pub fn exec(&mut self, cmd: Command) -> Result<bool, ()> {
        let (res, _) = cmd.invoke(self)?;

        Ok(res)
    }
}

impl Default for Env {
    fn default() -> Env {
        let cut = Vec::new();
        let filename = None;

        let last_re = None;
        let last_pat = None;
        let last_flags = None;

        let last_cmd = None;
        let last_rcmd = None;
        let last_wcmd = None;

        Env {
            cut,
            filename,
            last_re,
            last_pat,
            last_flags,

            last_cmd,
            last_rcmd,
            last_wcmd,
        }
    }
}
