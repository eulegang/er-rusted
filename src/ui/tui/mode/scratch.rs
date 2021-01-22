use super::{Cmd, LineEdit, LineInsert, SealedTMode, TMode, Tui};
use crate::ui::tui::draw::*;
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::size,
};

pub struct Scratch {
    prev: CoreMode,
    key_buffer: String,
}

impl TMode for Scratch {
    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        let scratch = &mut tui.interp.scratch;

        match key.code {
            KeyCode::Tab => {
                let next: SealedTMode = self.prev.into();
                if !matches!(next, SealedTMode::Cmd(_)) {
                    ShowCursorDrawCmd(true).draw(tui)?;
                }
                BufferDrawCmd.draw(tui)?;
                next.draw(tui)?;
                return Ok(next);
            }

            KeyCode::Char(digit) if digit.is_digit(10) => {
                self.key_buffer.push(digit);
                return Ok(self.into());
            }

            KeyCode::Char('j') => {
                let delta: usize = self.key_buffer.parse().unwrap_or(1);
                self.key_buffer.clear();

                scratch.down(delta);
            }

            KeyCode::Char('k') => {
                let (_, w) = size()?;
                let w: usize = w.into();
                let delta: usize = self.key_buffer.parse().unwrap_or(1);
                self.key_buffer.clear();
                scratch.up(delta, w);
            }

            KeyCode::Char('d') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.down(w / 2);
            }

            KeyCode::Char('u') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.up(w / 2, w);
            }

            KeyCode::Char('f') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.down(w);
            }

            KeyCode::Char('b') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.up(w, w);
            }

            KeyCode::Char('g') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.last(w);
            }

            KeyCode::Char('G') => {
                self.key_buffer.clear();
                scratch.refresh();
            }

            KeyCode::Char('C') => {
                self.key_buffer.clear();
                scratch.clear();
            }

            _ => (),
        };

        ScratchDrawCmd.draw(tui)?;

        Ok(self.into())
    }

    fn process_ctl_key(self, _: KeyEvent, _: &mut Tui) -> crossterm::Result<SealedTMode> {
        Ok(self.into())
    }

    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()> {
        ScratchDrawCmd.draw(tui)?;
        Ok(())
    }
}

impl<M> From<M> for Scratch
where
    M: Into<CoreMode>,
{
    fn from(m: M) -> Scratch {
        let prev = m.into();
        let key_buffer = String::new();

        Scratch { prev, key_buffer }
    }
}

pub enum CoreMode {
    Cmd(Cmd),
    LineEdit(LineEdit),
    LineInsert(LineInsert),
}

impl Into<SealedTMode> for CoreMode {
    fn into(self) -> SealedTMode {
        match self {
            CoreMode::Cmd(cmd) => cmd.into(),
            CoreMode::LineEdit(line_edit) => line_edit.into(),
            CoreMode::LineInsert(line_insert) => line_insert.into(),
        }
    }
}

impl From<Cmd> for CoreMode {
    fn from(cmd: Cmd) -> CoreMode {
        CoreMode::Cmd(cmd)
    }
}

impl From<LineEdit> for CoreMode {
    fn from(line_edit: LineEdit) -> CoreMode {
        CoreMode::LineEdit(line_edit)
    }
}

impl From<LineInsert> for CoreMode {
    fn from(line_insert: LineInsert) -> CoreMode {
        CoreMode::LineInsert(line_insert)
    }
}
