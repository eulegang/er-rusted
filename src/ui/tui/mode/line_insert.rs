use super::*;
use crate::ui::tui::action::*;

pub struct LineInsert {
    buffer: String,
    cursor: usize,
}

impl TMode for LineInsert {
    fn draw(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.draw_cmdline(&self.buffer)?
            .draw_cursor_at(self.cursor)?;

        Ok(())
    }

    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        match key.code {
            KeyCode::Char(ch) => {
                self.buffer.insert(self.cursor, ch);
                self.cursor += 1;
            }

            KeyCode::Backspace => {
                if let Some(cur) = self.cursor.checked_sub(1) {
                    self.buffer.remove(cur);
                    self.cursor = cur;
                }
            }

            KeyCode::Enter => {
                RunCmd(&self.buffer).invoke(tui)?;
                self.buffer.clear();
            }

            KeyCode::Esc => {
                let edit: LineEdit = (self.buffer, self.cursor).into();

                return Ok(edit.into());
            }

            _ => (),
        };

        self.draw(tui)?;

        Ok(self.into())
    }

    fn process_ctl_key(self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        match key.code {
            KeyCode::Char('c') => {
                let next = Cmd::default();
                tui.hide_cursor()?;
                next.draw(tui)?;

                return Ok(next.into());
            }

            KeyCode::Char('d') => Scroll::HalfDown.invoke(tui)?,
            KeyCode::Char('u') => Scroll::HalfUp.invoke(tui)?,
            KeyCode::Char('f') => Scroll::FullDown.invoke(tui)?,
            KeyCode::Char('b') => Scroll::FullUp.invoke(tui)?,
            KeyCode::Char('l') => RotateWindowLock::Down.invoke(tui)?,
            KeyCode::Char('o') => RotateWindowLock::Up.invoke(tui)?,
            _ => (),
        }

        Ok(self.into())
    }
}

impl Default for LineInsert {
    fn default() -> Self {
        let buffer = String::default();
        let cursor = buffer.len().checked_sub(1).unwrap_or(0);

        LineInsert { buffer, cursor }
    }
}

impl From<(String, usize)> for LineInsert {
    fn from((buffer, cursor): (String, usize)) -> LineInsert {
        LineInsert { buffer, cursor }
    }
}
