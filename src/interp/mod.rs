use crate::{
    ed::{Command, CommandResult, SubstFlags},
    re::{Pat, Re},
    Buffer,
};

use std::collections::HashMap;
use std::fs::File;
use std::io;

pub struct Interp {
    pub(crate) buffer: Buffer,
    pub(crate) marks: HashMap<char, usize>,
    pub(crate) cut: Vec<String>,

    pub(crate) last_re: Option<Re>,
    pub(crate) last_pat: Option<Pat>,
    pub(crate) last_flags: Option<SubstFlags>,
}

impl Interp {
    pub fn new(files: Vec<String>) -> io::Result<Interp> {
        let marks = HashMap::new();
        let cut = Vec::new();

        let buffer = if let Some(file) = files.get(0) {
            Buffer::read(File::open(file)?)?
        } else {
            Buffer::read("".as_bytes())?
        };

        let last_re = None;
        let last_pat = None;
        let last_flags = None;

        Ok(Interp {
            buffer,
            marks,
            cut,
            last_re,
            last_pat,
            last_flags,
        })
    }

    pub fn exec(&mut self, cmd: Command) -> CommandResult {
        cmd.invoke(self)
    }

    pub fn exec_with_text(&mut self, cmd: Command, text: Vec<String>) -> CommandResult {
        cmd.invoke_with_text(self, text)
    }
}
