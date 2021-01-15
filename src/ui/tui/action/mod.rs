use super::Tui;
use crate::ui::tui::motion::Motion;
use enum_dispatch::enum_dispatch;

mod edit;
mod history;
mod key_buffer;
mod mode;
mod run;
mod scroll;
mod shift;
mod transition;
mod window_lock;

pub use edit::Edit;
pub use history::History;
pub use key_buffer::KeyBuffer;
pub use mode::SetMode;
pub use run::{Reset, Run};
pub use scroll::Scroll;
pub use shift::{CutShift, Shift};
pub use transition::Transition;
pub use window_lock::RotateWindowLock;

#[enum_dispatch]
pub trait Action {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()>;
}

#[enum_dispatch(Action)]
pub enum SealedAction<M: Motion> {
    Edit,
    History,
    KeyBuffer,
    SetMode,
    Reset,
    Run,
    Scroll,
    CutShift(CutShift<M>),
    Shift(Shift<M>),
    Transition,
    RotateWindowLock,
}
