use super::*;
use crate::ui::tui::action::{Action, RotateWindowLock, RunCmd, Scroll};
use crate::ui::tui::mode::key_seq::*;
use crate::ui::tui::motion::*;
use crossterm::event::KeyEvent;
use std::convert::Infallible;
use std::mem::take;
use std::str::FromStr;

pub struct LineEdit {
    buffer: String,
    ctx: String,
    cursor: usize,
    search: Option<Search>,
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
        let search = None;

        LineEdit {
            buffer,
            ctx,
            cursor,
            search,
        }
    }
}

impl From<(String, usize)> for LineEdit {
    fn from((buffer, cursor): (String, usize)) -> LineEdit {
        let ctx = String::with_capacity(8);
        let search = None;

        LineEdit {
            buffer,
            ctx,
            cursor,
            search,
        }
    }
}

impl TMode for LineEdit {
    fn draw(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.draw_cmdline(&self.buffer)?
            .draw_key_buffer(&self.ctx)?
            .draw_cursor_at(self.cursor)?;
        Ok(())
    }

    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        match key.code {
            KeyCode::Char(ch) => {
                self.ctx.push(ch);
            }

            KeyCode::Enter => {
                let cmd = Cmd::default();
                RunCmd(&self.buffer).invoke(tui)?;
                cmd.draw(tui)?;

                return Ok(cmd.into());
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

impl LineEdit {
    pub fn take_action(mut self, key_seq: KeySeq, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        let num = key_seq.num;
        match key_seq.action {
            KSAction::Move(range) => {
                if let Some(cur) = range.find_next(&self.buffer, self.cursor, num, self.search) {
                    self.cursor = cur;
                }
            }

            KSAction::Delete(Range::Whole) => self.buffer.clear(),
            KSAction::Delete(range) => {
                if let Some(cur) = range.find_next(&self.buffer, self.cursor, num, self.search) {
                    let (min, max) = (cur.min(self.cursor), cur.max(self.cursor));

                    self.buffer.drain(min..max);

                    self.cursor = min;
                }
            }

            KSAction::Change(Range::Whole) => {
                self.buffer.clear();

                let next = LineInsert::from((self.buffer, self.cursor));
                next.draw(tui)?;

                return Ok(next.into());
            }

            KSAction::Change(range) => {
                if let Some(cur) = range.find_next(&self.buffer, self.cursor, num, self.search) {
                    let (min, max) = (cur.min(self.cursor), cur.max(self.cursor));

                    self.buffer.drain(min..max);

                    self.cursor = min;
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
                tui.hide_cursor()?;
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
                        }
                    }

                    History::Past => {
                        if !hist.active() {
                            hist.hold(take(&mut self.buffer))
                        }

                        if let Some(cmd) = hist.up(num) {
                            self.buffer = cmd.to_string()
                        }
                    }

                    History::Current => {
                        if hist.active() {
                            self.buffer = hist.take().unwrap_or_default()
                        }
                    }

                    History::Last => {
                        if !hist.active() {
                            hist.hold(take(&mut self.buffer))
                        }

                        if let Some(cmd) = hist.last() {
                            self.buffer = cmd.to_string()
                        }
                    }
                }
            }
        }

        Ok(self.into())
    }
}
