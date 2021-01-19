use super::Tui;
use enum_dispatch::enum_dispatch;

mod run;
mod scroll;
mod window_lock;

pub use run::RunCmd;
pub use scroll::Scroll;
pub use window_lock::RotateWindowLock;

#[enum_dispatch]
pub trait Action {
    fn invoke(&self, tui: &mut Tui) -> crossterm::Result<()>;
}

#[enum_dispatch(Action)]
pub enum SealedAction {
    Scroll,
    RotateWindowLock,
}
