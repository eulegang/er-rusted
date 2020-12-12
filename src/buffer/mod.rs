use std::io::{self, BufRead, BufReader, Read, Write};
use std::vec::Drain;

#[cfg(test)]
mod test;

/// A buffer representing a file being edited
pub struct Buffer {
    /// 1-based indexing in lines
    pub(crate) cur: usize,

    /// Line content
    pub(crate) lines: Vec<String>,
}

impl Buffer {
    /// Create a Buffer from a read
    pub fn read(r: impl Read) -> io::Result<Buffer> {
        let mut buf = BufReader::new(r);
        let mut lines = Vec::new();
        let cur = 1;

        loop {
            let mut line = String::new();
            let bytes = buf.read_line(&mut line)?;

            if bytes == 0 {
                break Ok(Buffer { cur, lines });
            } else {
                chomp(&mut line);
                lines.push(line);
            }
        }
    }

    /// Write a buffer out to a Write
    pub fn write(&self, w: &mut impl Write) -> io::Result<usize> {
        let mut written = 0;
        for line in &self.lines {
            let bytes = line.as_bytes();
            w.write_all(bytes)?;
            w.write_all(&[10])?;

            written += bytes.len() + 1;
        }

        w.flush()?;

        Ok(written)
    }

    /// gives the current lines
    pub fn cursor(&self) -> usize {
        self.cur
    }

    /// gives the number of lines
    pub fn lines(&self) -> usize {
        self.lines.len()
    }

    /// index lines 1-based
    pub fn line(&self, line: usize) -> Option<&str> {
        if let Some(s) = self.lines.get(line - 1) {
            Some(s.as_str())
        } else {
            None
        }
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Option<Drain<String>> {
        if 1 <= start && end <= self.lines.len() {
            self.cur = start;
            Some(self.lines.drain((start - 1)..=(end - 1)))
        } else {
            None
        }
    }

    pub fn insert(&mut self, line: usize, lines: Vec<String>) -> bool {
        let realign = line.checked_sub(1).unwrap_or(0);

        if realign < self.lines.len() {
            self.cur = realign + lines.len();
            self.lines.splice(realign..realign, lines);
            true
        } else {
            false
        }
    }

    pub fn range(&self, start: usize, end: usize) -> Option<Vec<String>> {
        let begin = start.checked_sub(1).unwrap_or(0);
        let end = end.checked_sub(1).unwrap_or(0);

        if end >= self.lines.len() {
            return None;
        }

        let mut buf = Vec::with_capacity(end - begin + 1);

        for i in begin..=end {
            buf.push(self.lines[i].clone());
        }

        Some(buf)
    }

    pub fn append(&mut self, line: usize, lines: Vec<String>) -> bool {
        if line <= self.lines.len() {
            self.cur = line + lines.len();
            self.lines.splice(line..line, lines);
            true
        } else {
            false
        }
    }

    pub fn change(&mut self, start: usize, end: usize, lines: Vec<String>) {
        self.lines.splice(start - 1..end, lines);
    }
}

fn chomp(line: &mut String) {
    let bytes = line.as_bytes();
    let has_nl = bytes.len() > 0 && bytes[bytes.len() - 1] == 10;
    let has_cr = bytes.len() > 1 && bytes[bytes.len() - 2] == 13;

    if has_nl {
        line.pop();
        if has_cr {
            line.pop();
        }
    }
}
