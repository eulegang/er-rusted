use super::*;
use crate::interp::scratch::ScratchPad;
use crate::ui::tui::action::*;
use crate::ui::tui::draw::*;
use crate::Command;

#[derive(Debug)]
pub struct Text {
    pos: usize,
    hide: usize,
    cmd: Command,
    lines: Vec<String>,
}

impl TMode for Text {
    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        match key.code {
            KeyCode::Enter => {
                if self.lines.last().map(String::as_str) == Some(".") {
                    self.lines.pop();
                    let next = Cmd::default();
                    self.cmd.inject(self.lines);

                    match tui.interp.exec(&self.cmd) {
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

                    next.draw(tui)?;
                    return Ok(next.into());
                } else {
                    self.lines.push(String::new());
                }
            }

            KeyCode::Char(ch) => {
                let size = self.lines.len();

                if size > 0 {
                    self.lines[size - 1].push(ch)
                }
            }

            KeyCode::Backspace => {
                let size = self.lines.len();

                if size > 0 {
                    let s = &mut self.lines[size - 1];

                    if s.is_empty() {
                        self.lines.pop();
                    } else {
                        s.pop();
                    }
                }
            }

            _ => (),
        }

        self.draw(tui)?;
        Ok(self.into())
    }

    fn process_ctl_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        match key.code {
            KeyCode::Char('c') => {
                if self.lines.len() == 0 {
                    let next = Cmd::default();
                    next.draw(tui)?;

                    return Ok(next.into());
                } else {
                    self.lines.pop();
                }
            }

            KeyCode::Char('d') => Scroll::HalfDown.invoke(tui)?,
            KeyCode::Char('u') => Scroll::HalfUp.invoke(tui)?,
            KeyCode::Char('f') => Scroll::FullDown.invoke(tui)?,
            KeyCode::Char('b') => Scroll::FullUp.invoke(tui)?,
            KeyCode::Char('l') => RotateWindowLock::Down.invoke(tui)?,
            KeyCode::Char('o') => RotateWindowLock::Up.invoke(tui)?,

            KeyCode::Char('t') => tui.interp.scratch.print(&format!("{:?}", self)),
            _ => (),
        }

        self.draw(tui)?;

        Ok(self.into())
    }

    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()> {
        InsertTextDrawCmd(self.pos, self.hide, &self.lines).draw(tui)?;

        Ok(())
    }
}

impl From<(usize, usize, Command)> for Text {
    fn from((pos, hide, cmd): (usize, usize, Command)) -> Text {
        let mut lines = Vec::new();
        lines.push(String::new());

        Text {
            pos,
            hide,
            lines,
            cmd,
        }
    }
}
