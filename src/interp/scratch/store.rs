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
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn is_stale(&self) -> bool {
        self.stale
    }

    #[allow(dead_code)]
    pub fn refresh(&mut self) {
        self.stale = false;
        self.offset = 0;
    }
}
