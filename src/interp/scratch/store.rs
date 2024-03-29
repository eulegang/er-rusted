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

impl StoreScratchPad {
    pub fn buffer_lines(&mut self, cap: usize) -> Vec<&str> {
        let mut buf = Vec::with_capacity(cap);

        for i in self.offset..(self.offset + cap) {
            if let Some(elem) = self.lines.get(i) {
                buf.push(elem.as_str())
            } else {
                break;
            }
        }

        buf
    }

    pub fn is_stale(&self) -> bool {
        self.stale
    }

    pub fn refresh(&mut self) {
        self.stale = false;
        self.offset = 0;
    }

    pub fn up(&mut self, delta: usize, frame_size: usize) {
        self.offset += delta;
        self.offset = self
            .offset
            .min(self.lines.len().checked_sub(frame_size).unwrap_or(0))
    }

    pub fn down(&mut self, delta: usize) {
        self.offset = self.offset.checked_sub(delta).unwrap_or(0)
    }

    pub fn last(&mut self, frame_size: usize) {
        self.offset = self.lines.len().checked_sub(frame_size).unwrap_or(0)
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.offset = 0;
        self.stale = false;
    }
}
