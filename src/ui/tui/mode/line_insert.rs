use super::*;
use crate::ed::cmd::Command;
use crate::ui::tui::action::*;
use crate::ui::tui::draw::*;
use std::str::FromStr;

pub struct LineInsert {
    buffer: String,
    cursor: usize,
}

impl TMode for LineInsert {
    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()> {
        CmdDrawCmd(&self.buffer).draw(tui)?;
        CursorDrawCmd(self.cursor).draw(tui)?;

        Ok(())
    }

    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
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

            KeyCode::Enter => return self.process_cmd(tui),

            KeyCode::Esc => {
                let edit: LineEdit = (self.buffer, self.cursor).into();

                return Ok(edit.into());
            }

            KeyCode::Tab => {
                let next: Scratch = self.into();
                next.draw(tui)?;
                ShowCursorDrawCmd(false).draw(tui)?;
                return Ok(next.into());
            }

            _ => (),
        };

        self.draw(tui)?;

        Ok(self.into())
    }

    fn process_ctl_key(self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        match key.code {
            KeyCode::Char('c') => {
                let next = Cmd::default();
                ShowCursorDrawCmd(false).draw(tui)?;
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

impl LineInsert {
    fn process_cmd(mut self, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        tui.history.reset();

        if !self.buffer.trim().is_empty() {
            tui.history.append(self.buffer.to_string());
        }

        let cmd = match Command::from_str(&self.buffer) {
            Ok(cmd) => cmd,
            Err(_) => {
                ErrorDrawCmd("unable to parse command").draw(tui)?;
                self.buffer.clear();
                return Ok(self.into());
            }
        };

        self.buffer.clear();

        if cmd.needs_text() {
            if let Some((pos, hide)) = cmd.text_markers(&tui.interp.buffer) {
                let next: Text = (pos, hide, cmd).into();
                next.draw(tui)?;

                return Ok(next.into());
            } else {
                ErrorDrawCmd("text needed (not supported yet)").draw(tui)?;
                return Ok(self.into());
            }
        }

        match tui.interp.exec(&cmd) {
            Ok(false) => {
                tui.pending_quit = true;
            }

            Ok(true) => {
                CmdDrawCmd("").draw(tui)?;
                BufferDrawCmd.draw(tui)?;
            }

            Err(err) => {
                ErrorDrawCmd(&format!("{}", err)).draw(tui)?;
            }
        }

        let next = Cmd::default();
        if tui.interp.scratch.is_stale() {
            tui.interp.scratch.refresh();
            ShowCursorDrawCmd(false).draw(tui)?;

            let next: Scratch = next.into();
            next.draw(tui)?;
            return Ok(next.into());
        }

        return Ok(next.into());
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
