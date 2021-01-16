use super::ScratchPad;
use std::io::{self, Stdout, Write};

#[derive(Debug)]
pub struct StdoutScratchPad {
    out: Stdout,
}

impl Default for StdoutScratchPad {
    fn default() -> StdoutScratchPad {
        let out = io::stdout();
        StdoutScratchPad { out }
    }
}

impl ScratchPad for StdoutScratchPad {
    fn print(&mut self, line: &str) {
        self.out
            .write_all(line.as_bytes())
            .expect("Failed to write to stdout");
    }
}
