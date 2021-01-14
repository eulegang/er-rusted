use super::*;
use crate::ui::tui::action::*;

pub(crate) fn process_cmd(event: Event, tui: &mut Tui) -> eyre::Result<bool> {
    match event {
        Event::Key(key) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') => {
                        tui.cmd.clear();
                        tui.draw_cmd()?;
                    }

                    KeyCode::Char('d') => {
                        Scroll::HalfDown.invoke(tui)?;
                    }

                    KeyCode::Char('u') => {
                        Scroll::HalfUp.invoke(tui)?;
                    }

                    KeyCode::Char('f') => {
                        Scroll::FullDown.invoke(tui)?;
                    }

                    KeyCode::Char('b') => {
                        Scroll::FullUp.invoke(tui)?;
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
                        Edit::Insert(ch).invoke(tui)?;
                    }

                    KeyCode::Backspace => {
                        Edit::Backspace.invoke(tui)?;
                    }

                    KeyCode::Esc => {
                        SetMode(Mode::LineEdit).invoke(tui)?;
                    }

                    KeyCode::Enter => {
                        Run.invoke(tui)?;
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
