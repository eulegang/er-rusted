use crate::{
    ed::{Command, CommandResult},
    Buffer,
};

use std::collections::HashMap;
use std::fs::File;
use std::io;

pub struct Interp {
    pub(crate) buffer: Buffer,
    pub(crate) marks: HashMap<char, usize>,
}

impl Interp {
    pub fn new(files: Vec<String>) -> io::Result<Interp> {
        let marks = HashMap::new();

        let buffer = if let Some(file) = files.get(0) {
            Buffer::read(File::open(file)?)?
        } else {
            Buffer::read("".as_bytes())?
        };

        Ok(Interp { buffer, marks })
    }

    pub fn exec(&mut self, cmd: Command) -> CommandResult {
        cmd.invoke(self)
    }

    pub fn exec_with_text(&mut self, cmd: Command, text: Vec<String>) -> CommandResult {
        cmd.invoke_with_text(self, text)
    }
}
