use std::collections::VecDeque;

#[derive(Debug)]
pub struct History {
    lines: VecDeque<String>,
    pos: Option<u8>,
    hold: Option<String>,
}

impl History {
    pub(crate) fn new() -> History {
        let lines = VecDeque::with_capacity(256);
        let pos = None;
        let hold = None;

        History { lines, pos, hold }
    }

    pub(crate) fn append(&mut self, line: String) {
        if self.lines.len() == self.lines.capacity() {
            self.lines.pop_back();
        }

        self.lines.push_front(line);
    }

    fn get(&self) -> Option<&str> {
        if let Some(pos) = self.pos {
            self.lines.get(pos as usize).map(String::as_str)
        } else {
            None
        }
    }

    fn cap_pos(&mut self) {
        if let Some(max_pos) = self.lines.len().checked_sub(1) {
            if let Some(pos) = self.pos {
                self.pos = Some((max_pos as u8).min(pos))
            }
        } else {
            self.pos = None
        }
    }

    pub(crate) fn up(&mut self) -> Option<&str> {
        self.pos = Some(self.pos.map_or(0, |p| p + 1));
        self.cap_pos();

        self.get()
    }

    pub(crate) fn down(&mut self) -> Option<&str> {
        self.pos = self.pos.map_or(None, |p| p.checked_sub(1));

        self.get()
    }

    pub(crate) fn hold(&mut self, buf: String) {
        self.hold = Some(buf)
    }

    pub(crate) fn take(&mut self) -> Option<String> {
        self.hold.take()
    }

    pub(crate) fn active(&mut self) -> bool {
        self.pos.is_some()
    }

    pub(crate) fn reset(&mut self) {
        self.pos = None;
        self.hold = None;
    }
}
