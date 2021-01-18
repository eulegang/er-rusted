use enum_dispatch::enum_dispatch;

mod abs;
mod cclass;
mod rel;
mod search;

pub use abs::Absolute;
pub use cclass::CClass;
pub use rel::Relative;
pub use search::Search;

#[enum_dispatch]
pub trait Motion {
    fn move_cursor(&self, buffer: &str, cursor: usize) -> Option<usize>;
}

#[enum_dispatch]
pub trait Idemp {
    fn is_idempotent(&self) -> bool;
}

#[enum_dispatch(Motion, Idemp)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SealedMotion {
    Absolute,
    CClass,
    Relative,
    Search,
}
