use super::Tui;
use crossterm::event::{KeyCode, KeyEvent};
use enum_dispatch::enum_dispatch;

mod cmd;
mod key_seq;
mod line_edit;
mod line_insert;
mod scratch;

pub use cmd::Cmd;
pub use line_edit::LineEdit;
pub use line_insert::LineInsert;
pub use scratch::Scratch;

pub use key_seq::{KeySeq, KeySeqErr};

#[enum_dispatch]
pub trait TMode {
    fn process_key(self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode>;
    fn process_ctl_key(self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode>;

    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()>;
}

#[enum_dispatch(TMode)]
pub enum SealedTMode {
    Cmd,
    LineEdit,
    LineInsert,
    Scratch,
}

impl Default for SealedTMode {
    fn default() -> Self {
        Cmd::default().into()
    }
}
