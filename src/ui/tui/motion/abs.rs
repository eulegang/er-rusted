use super::{Idemp, Motion};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Absolute {
    First,
    Last,
}

impl Motion for Absolute {
    fn move_cursor(&self, buffer: &str, _: usize) -> Option<usize> {
        match self {
            Absolute::First => Some(0),
            Absolute::Last => Some(buffer.len() - 1),
        }
    }
}

impl Idemp for Absolute {
    fn is_idempotent(&self) -> bool {
        true
    }
}
