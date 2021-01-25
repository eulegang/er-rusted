use super::Tui;
use crossterm::{
    cursor::{self, MoveTo, MoveToNextLine},
    style::{style, Color, Print},
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};

mod buffer;
mod cmd;
mod cur;
mod error;
mod key_buffer;
mod scratch;
mod text;

pub use buffer::BufferDrawCmd;
pub use cmd::CmdDrawCmd;
pub use cur::{CursorDrawCmd, ShowCursorDrawCmd};
pub use error::ErrorDrawCmd;
pub use key_buffer::KeyBufferDrawCmd;
pub use scratch::ScratchDrawCmd;
pub use text::InsertTextDrawCmd;

pub trait DrawCmd {
    fn draw<'a>(&self, tui: &'a mut Tui) -> crossterm::Result<()>;
}
