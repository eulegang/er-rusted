use super::Tui;
use crate::Command;
use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    terminal::size,
};
use std::io::Write;
use std::mem::take;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Cmd,
    LineEdit,
}

impl Mode {
    pub(crate) fn process(&self, event: Event, tui: &mut Tui) -> eyre::Result<bool> {
        match self {
            Mode::Cmd => cmd(event, tui),
            Mode::LineEdit => line_edit(event, tui),
        }
    }
}

fn line_edit(event: Event, tui: &mut Tui) -> eyre::Result<bool> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Enter => {
                tui.mode = Mode::Cmd;
                tui.hide_cursor()?;
                tui.history.reset();

                if let Ok(cmd) = Command::from_str(&tui.cmd) {
                    let selected = take(&mut tui.cmd);

                    if cmd.needs_text() {
                        tui.draw_error()?.flush()?;
                        return Ok(true);
                    }

                    match tui.interp.exec(&cmd) {
                        Ok(false) => return Ok(false),
                        Ok(true) => {
                            tui.history.append(selected);
                            tui.draw_cmd()?.draw_buffer()?;
                        }
                        Err(()) => {
                            tui.draw_error()?;
                        }
                    }
                } else {
                    tui.cmd.clear();
                    tui.draw_error()?;
                }
            }

            KeyCode::Char('k') => {
                let hist = &mut tui.history;
                if !hist.active() {
                    hist.hold(take(&mut tui.cmd));
                }

                if let Some(cmd) = hist.up() {
                    tui.cmd = cmd.to_string();
                    tui.draw_cmd()?;
                }
            }

            KeyCode::Char('j') => {
                let hist = &mut tui.history;

                if hist.active() {
                    if let Some(cmd) = hist.down() {
                        tui.cmd = cmd.to_string();
                    } else {
                        tui.cmd = hist.take().unwrap_or_default();
                    }
                    tui.draw_cmd()?;
                }
            }

            KeyCode::Char('c') => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    tui.mode = Mode::Cmd;
                    tui.hide_cursor()?;
                    tui.history.reset();
                    tui.cmd.clear();

                    tui.draw_cmd()?;
                }
            }

            _ => (),
        },

        _ => (),
    }

    tui.stdout.flush()?;

    Ok(true)
}

fn cmd(event: Event, tui: &mut Tui) -> eyre::Result<bool> {
    match event {
        Event::Key(key) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') => {
                        tui.cmd.clear();
                        tui.draw_cmd()?;
                    }

                    KeyCode::Char('d') => {
                        let (_, w) = size()?;
                        let w: usize = (w / 2).into();

                        tui.interp.buffer.scroll_forward(w);
                        tui.draw_buffer()?;
                    }

                    KeyCode::Char('u') => {
                        let (_, w) = size()?;
                        let w: usize = (w / 2).into();

                        tui.interp.buffer.scroll_backward(w);
                        tui.draw_buffer()?;
                    }

                    KeyCode::Char('f') => {
                        let (_, w) = size()?;
                        let w: usize = w.into();

                        tui.interp.buffer.scroll_forward(w);
                        tui.draw_buffer()?;
                    }

                    KeyCode::Char('b') => {
                        let (_, w) = size()?;
                        let w: usize = w.into();

                        tui.interp.buffer.scroll_backward(w);
                        tui.draw_buffer()?;
                    }

                    KeyCode::Char('l') => {
                        tui.window_lock = tui.window_lock.next();
                        tui.draw_buffer()?;
                    }

                    _ => (),
                }
            } else {
                match key.code {
                    KeyCode::Char(ch) => {
                        tui.cmd.push(ch);
                        tui.draw_cmd()?;
                    }

                    KeyCode::Backspace => {
                        tui.cmd.pop();
                        tui.draw_cmd()?;
                    }

                    KeyCode::Esc => {
                        tui.mode = Mode::LineEdit;
                        tui.show_cursor()?;
                        tui.set_col(tui.cmd.len() as u16)?;
                    }

                    KeyCode::Enter => {
                        if let Ok(cmd) = Command::from_str(&tui.cmd) {
                            let entered = take(&mut tui.cmd);

                            if cmd.needs_text() {
                                tui.draw_error()?;
                                return Ok(true);
                            }

                            match tui.interp.exec(&cmd) {
                                Ok(false) => return Ok(false),
                                Ok(true) => {
                                    if !entered.trim().is_empty() {
                                        tui.history.append(entered);
                                    }

                                    tui.draw_cmd()?.draw_buffer()?;
                                }
                                Err(()) => {
                                    tui.draw_error()?;
                                }
                            }
                        } else {
                            tui.cmd.clear();
                            tui.draw_error()?;
                        }
                    }

                    _ => (),
                };
            }
        }

        _ => (),
    }

    tui.flush()?;
    Ok(true)
}
