use crate::{
    cmd::{Command, InvocationError, SubstFlags},
    interp::write_hook::WriteHook,
    re::{Pat, Re},
    Buffer,
};

use scratch::{ScratchPad, StdoutScratchPad};
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind};

pub(crate) mod scratch;
pub(crate) mod write_hook;

/// Interprets commands on a buffer
#[derive(Debug)]
pub struct Interpreter<S: ScratchPad = StdoutScratchPad> {
    pub(crate) filelist: Vec<String>,
    pub(crate) filepos: usize,
    pub(crate) buffer: Buffer,
    pub(crate) env: Env,
    pub(crate) scratch: S,
}

#[derive(Debug)]
pub struct Env {
    pub(crate) cut: Vec<String>,
    pub(crate) filename: Option<String>,
    pub(crate) scroll: Option<usize>,

    pub(crate) last_re: Option<Re>,
    pub(crate) last_pat: Option<Pat>,
    pub(crate) last_flags: Option<SubstFlags>,

    pub(crate) last_cmd: Option<String>,
    pub(crate) last_rcmd: Option<String>,
    pub(crate) last_wcmd: Option<String>,
    pub(crate) write_hook: WriteHook,
}

impl Interpreter {
    /// Creates an interpreter with multiple files in it's arglist
    pub fn new<S>(files: Vec<String>) -> io::Result<Interpreter<S>>
    where
        S: ScratchPad,
    {
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
        let scratch = S::default();

        let filelist = files;
        let filepos = 0;

        Ok(Interpreter {
            filelist,
            filepos,
            buffer,
            env,
            scratch,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_reader<S: ScratchPad, R: std::io::Read>(r: R) -> io::Result<Interpreter<S>> {
        let buffer = Buffer::read(r)?;
        let env = Env::default();

        let filelist = vec![];
        let filepos = 0;
        let scratch = S::default();

        Ok(Interpreter {
            filelist,
            filepos,
            buffer,
            env,
            scratch,
        })
    }
}

impl<S> Interpreter<S>
where
    S: ScratchPad,
{
    /// Executes a command on the given buffer
    pub fn exec(&mut self, cmd: &Command) -> Result<bool, InvocationError> {
        let (res, _) = cmd.invoke(self)?;

        Ok(res)
    }

    /// Writes to filename if buffer is dirty
    pub fn ensure_clean(&mut self) -> io::Result<()> {
        if self.buffer.is_dirty() {
            if let Some(path) = &self.env.filename {
                let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

                self.buffer.write(&mut file)?;
            }
        }

        Ok(())
    }
}

impl Default for Env {
    fn default() -> Env {
        let cut = Vec::new();
        let scroll = None;
        let filename = None;

        let last_re = None;
        let last_pat = None;
        let last_flags = None;

        let last_cmd = None;
        let last_rcmd = None;
        let last_wcmd = None;

        let write_hook = WriteHook::default();

        Env {
            cut,
            filename,
            scroll,

            last_re,
            last_pat,
            last_flags,

            last_cmd,
            last_rcmd,
            last_wcmd,

            write_hook,
        }
    }
}
