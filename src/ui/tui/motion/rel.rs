use super::Motion;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Relative {
    Left,
    Right,
}

impl Motion for Relative {
    fn move_cursor(&self, buffer: &str, cursor: usize) -> Option<usize> {
        match self {
            Relative::Left => cursor.checked_sub(1).or(Some(0)),
            Relative::Right => Some((buffer.len() - 1).min(cursor + 1)),
        }
    }
}
