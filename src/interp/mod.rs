use crate::Buffer;
use std::collections::HashMap;
use std::fs::File;
use std::io;

pub struct Interp {
    files: Vec<String>,
    index: usize,
    buffer: Buffer,
    marks: HashMap<char, usize>,
}

impl Interp {
    pub fn new(files: Vec<String>) -> io::Result<Interp> {
        let index = 0;
        let marks = HashMap::new();

        let buffer = if let Some(file) = files.get(0) {
            Buffer::read(File::open(file)?)?
        } else {
            Buffer::read("".as_bytes())?
        };

        Ok(Interp {
            files,
            index,
            buffer,
            marks,
        })
    }
}
