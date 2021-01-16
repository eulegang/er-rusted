use super::ScratchPad;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct StoreScratchPad {
    lines: VecDeque<String>,
    offset: usize,
    stale: bool,
}

impl Default for StoreScratchPad {
    fn default() -> Self {
        let lines = VecDeque::with_capacity(1024);
        let offset = 0;
        let stale = false;

        StoreScratchPad {
            lines,
            offset,
            stale,
        }
    }
}

impl ScratchPad for StoreScratchPad {
    fn print(&mut self, line: &str) {
        self.stale = true;
        if self.lines.len() == self.lines.capacity() {
            self.lines.pop_back();
        }

        self.lines.push_front(line.to_string());
    }
}
