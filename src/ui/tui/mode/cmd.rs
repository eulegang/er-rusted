use super::*;
use crate::ed::cmd::Command;
use crate::interp::scratch::ScratchPad;
use crate::ui::tui::action::*;
use crate::ui::tui::draw::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Cmd {
    buffer: String,
}

impl TMode for Cmd {
    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        match key.code {
            KeyCode::Char(ch) => {
                self.buffer.push(ch);
            }

            KeyCode::Backspace => {
                self.buffer.pop();
            }

            KeyCode::Enter => return self.process_cmd(tui),

            KeyCode::Esc => {
                let edit: LineEdit = self.buffer.into();
                ShowCursorDrawCmd(true).draw(tui)?;
                edit.draw(tui)?;

                return Ok(edit.into());
            }

            KeyCode::Tab => {
                let next: Scratch = self.into();
                next.draw(tui)?;
                return Ok(next.into());
            }

            _ => (),
        };

        self.draw(tui)?;

        Ok(self.into())
    }

    fn process_ctl_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        match key.code {
            KeyCode::Char('c') => self.buffer.clear(),

            KeyCode::Char('d') => Scroll::HalfDown.invoke(tui)?,
            KeyCode::Char('u') => Scroll::HalfUp.invoke(tui)?,
            KeyCode::Char('f') => Scroll::FullDown.invoke(tui)?,
            KeyCode::Char('b') => Scroll::FullUp.invoke(tui)?,
            KeyCode::Char('l') => RotateWindowLock::Down.invoke(tui)?,
            KeyCode::Char('o') => RotateWindowLock::Up.invoke(tui)?,
            KeyCode::Char('t') => tui.interp.scratch.print(&format!("{:?}", self)),
            _ => (),
        }

        Ok(self.into())
    }

    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()> {
        CmdDrawCmd(&self.buffer).draw(tui)?;

        Ok(())
    }
}

impl Default for Cmd {
    fn default() -> Self {
        let buffer = String::default();

        Cmd { buffer }
    }
}

impl From<String> for Cmd {
    fn from(buffer: String) -> Cmd {
        Cmd { buffer }
    }
}

impl Cmd {
    fn process_cmd(mut self, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        tui.history.reset();

        if !self.buffer.trim().is_empty() {
            tui.history.append(self.buffer.to_string());
        }

        let cmd = match Command::from_str(&self.buffer) {
            Ok(cmd) => cmd,
            Err(_) => {
                ErrorDrawCmd("unable to parse command").draw(tui)?;
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

        if tui.interp.scratch.is_stale() {
            tui.interp.scratch.refresh();
            let next: Scratch = self.into();
            next.draw(tui)?;
            return Ok(next.into());
        }

        return Ok(self.into());
    }
}
