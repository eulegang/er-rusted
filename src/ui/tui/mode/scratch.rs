use super::{Cmd, LineEdit, LineInsert, SealedTMode, TMode, Tui};
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::size,
};

pub struct Scratch {
    prev: CoreMode,
    key_buffer: String,
}

impl TMode for Scratch {
    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        let scratch = &mut tui.interp.scratch;

        match key.code {
            KeyCode::Tab => {
                let next: SealedTMode = self.prev.into();
                next.draw(tui)?;
                if !matches!(next, SealedTMode::Cmd(_)) {
                    tui.show_cursor()?;
                }
                tui.draw_buffer()?;
                return Ok(next);
            }

            KeyCode::Char(digit) if digit.is_digit(10) => {
                self.key_buffer.push(digit);
            }

            KeyCode::Char('j') => {
                let delta: usize = self.key_buffer.parse().unwrap_or(1);
                self.key_buffer.clear();

                scratch.down(delta);
                tui.draw_scratch()?;
            }

            KeyCode::Char('k') => {
                let (_, w) = size()?;
                let w: usize = w.into();
                let delta: usize = self.key_buffer.parse().unwrap_or(1);
                self.key_buffer.clear();
                scratch.up(delta, w);

                tui.draw_scratch()?;
            }

            KeyCode::Char('d') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.down(w / 2);

                tui.draw_scratch()?;
            }

            KeyCode::Char('u') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.up(w / 2, w);

                tui.draw_scratch()?;
            }

            KeyCode::Char('f') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.down(w);

                tui.draw_scratch()?;
            }
            KeyCode::Char('b') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.up(w, w);

                tui.draw_scratch()?;
            }

            KeyCode::Char('g') => {
                self.key_buffer.clear();
                let (_, w) = size()?;
                let w: usize = w.into();
                scratch.last(w);

                tui.draw_scratch()?;
            }

            KeyCode::Char('G') => {
                self.key_buffer.clear();
                scratch.refresh();
                tui.draw_scratch()?;
            }

            KeyCode::Char('C') => {
                self.key_buffer.clear();
                scratch.clear();
                tui.draw_scratch()?;
            }

            _ => (),
        };

        Ok(self.into())
    }

    fn process_ctl_key(self, _: KeyEvent, _: &mut Tui) -> eyre::Result<SealedTMode> {
        Ok(self.into())
    }

    fn draw(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.draw_scratch()?;
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
