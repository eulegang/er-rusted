use super::Tui;
use crossterm::{
    cursor::{self, MoveTo, MoveToNextLine},
    style::{style, Color, Print},
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

mod buffer;
mod cmd;
mod cur;
mod error;
mod key_buffer;
mod scratch;

pub use buffer::BufferDrawCmd;
pub use cmd::CmdDrawCmd;
pub use cur::{CursorDrawCmd, ShowCursorDrawCmd};
pub use error::ErrorDrawCmd;
pub use key_buffer::KeyBufferDrawCmd;
pub use scratch::ScratchDrawCmd;

pub trait DrawCmd {
    fn draw<'a>(&self, tui: &'a mut Tui) -> crossterm::Result<()>;
}
