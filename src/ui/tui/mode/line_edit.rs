use super::*;
use crate::ed::cmd::Command;
use crate::ui::tui::action::{Action, RotateWindowLock, Scroll};
use crate::ui::tui::draw::*;
use crate::ui::tui::mode::key_seq::*;
use crate::ui::tui::motion::SealedMotion;
use crossterm::event::KeyEvent;
use std::convert::Infallible;
use std::mem::take;
use std::str::FromStr;

pub struct LineEdit {
    buffer: String,
    ctx: String,
    cursor: usize,
}

impl FromStr for LineEdit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<LineEdit, Infallible> {
        Ok(s.to_string().into())
    }
}

impl From<String> for LineEdit {
    fn from(s: String) -> LineEdit {
        let buffer = s;
        let ctx = String::with_capacity(8);
        let cursor = buffer.len().checked_sub(1).unwrap_or(0);

        LineEdit {
            buffer,
            ctx,
            cursor,
        }
    }
}

impl From<(String, usize)> for LineEdit {
    fn from((buffer, cursor): (String, usize)) -> LineEdit {
        let ctx = String::with_capacity(8);

        LineEdit {
            buffer,
            ctx,
            cursor,
        }
    }
}

impl TMode for LineEdit {
    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()> {
        CmdDrawCmd(&self.buffer).draw(tui)?;
        KeyBufferDrawCmd(&self.ctx).draw(tui)?;
        CursorDrawCmd(self.cursor).draw(tui)?;

        Ok(())
    }

    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        match key.code {
            KeyCode::Char(ch) => {
                self.ctx.push(ch);
            }

            KeyCode::Enter => return self.process_cmd(tui),

            KeyCode::Tab => {
                let next: Scratch = self.into();
                next.draw(tui)?;
                ShowCursorDrawCmd(false).draw(tui)?;
                return Ok(next.into());
            }

            _ => (),
        };

        match self.ctx.parse::<KeySeq>() {
            Ok(key_seq) => {
                self.ctx.clear();
                let next = self.take_action(key_seq, tui)?;
                next.draw(tui)?;
                return Ok(next);
            }

            Err(KeySeqErr::Failed) => {
                self.ctx.clear();
            }

            Err(KeySeqErr::Insufficient) => (),
        }

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

impl LineEdit {
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

    pub fn take_action(mut self, key_seq: KeySeq, tui: &mut Tui) -> crossterm::Result<SealedTMode> {
        let num = key_seq.num;
        match key_seq.action {
            KSAction::Move(range) => {
                if let Some(cur) = range.find_next(&self.buffer, self.cursor, num, tui.search) {
                    self.cursor = cur;
                }

                if let Range::Motion(SealedMotion::Search(search)) = range {
                    tui.search = Some(search);
                }
            }

            KSAction::Delete(Range::Whole) => self.buffer.clear(),
            KSAction::Delete(range) => {
                if let Some(cur) = range.find_next(&self.buffer, self.cursor, num, tui.search) {
                    let (min, max) = (cur.min(self.cursor), cur.max(self.cursor));

                    self.buffer.drain(min..max);

                    self.cursor = min;
                }

                if let Range::Motion(SealedMotion::Search(search)) = range {
                    tui.search = Some(search);
                }
            }

            KSAction::Change(Range::Whole) => {
                self.buffer.clear();

                let next = LineInsert::from((self.buffer, self.cursor));
                next.draw(tui)?;

                return Ok(next.into());
            }

            KSAction::Change(range) => {
                if let Some(cur) = range.find_next(&self.buffer, self.cursor, num, tui.search) {
                    let (min, max) = (cur.min(self.cursor), cur.max(self.cursor));

                    self.buffer.drain(min..max);

                    self.cursor = min;
                }

                if let Range::Motion(SealedMotion::Search(search)) = range {
                    tui.search = Some(search);
                }

                let next = LineInsert::from((self.buffer, self.cursor));
                next.draw(tui)?;

                return Ok(next.into());
            }

            KSAction::Replace(ch) => {
                self.buffer.remove(self.cursor);
                self.buffer.insert(self.cursor, ch);
            }

            KSAction::Transition(Transition::HardAppend) => {
                let next = Cmd::from(self.buffer);
                next.draw(tui)?;

                return Ok(next.into());
            }

            KSAction::Transition(transition) => {
                let cursor = transition.update_cursor(&self.buffer, self.cursor);
                let next = LineInsert::from((self.buffer, cursor));
                next.draw(tui)?;

                return Ok(next.into());
            }

            KSAction::History(history) => {
                let hist = &mut tui.history;
                match history {
                    History::Recent => {
                        if hist.active() {
                            self.buffer = if let Some(cmd) = hist.down(num) {
                                cmd.to_string()
                            } else {
                                hist.take().unwrap_or_default()
                            };
                            self.cursor = self.cursor.min(self.buffer.len());
                        }
                    }

                    History::Past => {
                        if !hist.active() {
                            hist.hold(take(&mut self.buffer))
                        }

                        if let Some(cmd) = hist.up(num) {
                            self.buffer = cmd.to_string();
                            self.cursor = self.cursor.min(self.buffer.len());
                        }
                    }

                    History::Current => {
                        if hist.active() {
                            self.buffer = hist.take().unwrap_or_default();
                            self.cursor = self.cursor.min(self.buffer.len());
                        }
                    }

                    History::Last => {
                        if !hist.active() {
                            hist.hold(take(&mut self.buffer))
                        }

                        if let Some(cmd) = hist.last() {
                            self.buffer = cmd.to_string();
                            self.cursor = self.cursor.min(self.buffer.len());
                        }
                    }
                }
            }
        }

        Ok(self.into())
    }
}
