use super::*;
use crate::ui::tui::action::*;

pub struct Cmd {
    buffer: String,
}

impl TMode for Cmd {
    fn shows_cursor(&self) -> bool {
        false
    }

    fn process_key(mut self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        match key.code {
            KeyCode::Char(ch) => {
                self.buffer.push(ch);
            }

            KeyCode::Backspace => {
                self.buffer.pop();
            }

            KeyCode::Enter => {
                RunCmd(&self.buffer).invoke(tui)?;
                self.buffer.clear();
            }

            KeyCode::Esc => {
                let edit: LineEdit = self.buffer.into();
                tui.show_cursor()?;
                edit.draw(tui)?;

                return Ok(edit.into());
            }

            _ => (),
        };

        self.draw(tui)?;

        Ok(self.into())
    }

    fn process_ctl_key(mut self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<SealedTMode> {
        match key.code {
            KeyCode::Char('c') => self.buffer.clear(),

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

    fn draw(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.draw_cmdline(&self.buffer)?;

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
