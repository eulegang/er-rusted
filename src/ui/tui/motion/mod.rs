mod abs;
mod cclass;
mod rel;
mod search;

pub use abs::Absolute;
pub use cclass::CClass;
pub use rel::Relative;
pub use search::Search;

pub trait Motion {
    fn move_cursor(&self, buffer: &str, cursor: usize) -> Option<usize>;
}
