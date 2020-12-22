use crate::{
    cmd::{Command, SubstFlags},
    re::{Pat, Re},
    Buffer,
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind};

pub struct Interpreter {
    pub(crate) buffer: Buffer,
    pub(crate) env: Env,
}

pub struct Env {
    pub(crate) marks: HashMap<char, usize>,
    pub(crate) cut: Vec<String>,
    pub(crate) filename: Option<String>,

    pub(crate) last_re: Option<Re>,
    pub(crate) last_pat: Option<Pat>,
    pub(crate) last_flags: Option<SubstFlags>,

    pub(crate) last_cmd: Option<String>,
    pub(crate) last_rcmd: Option<String>,
    pub(crate) last_wcmd: Option<String>,
}

pub enum Action {
    Nop,
    Quit,
}

impl Interpreter {
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

        Ok(Interpreter { buffer, env })
    }

    pub fn exec(&mut self, cmd: Command) -> Result<Action, ()> {
        cmd.invoke(self)
    }
}

impl Default for Env {
    fn default() -> Env {
        let marks = HashMap::new();
        let cut = Vec::new();
        let filename = None;

        let last_re = None;
        let last_pat = None;
        let last_flags = None;

        let last_cmd = None;
        let last_rcmd = None;
        let last_wcmd = None;

        Env {
            marks,
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
