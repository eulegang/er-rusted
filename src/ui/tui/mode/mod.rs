use super::Tui;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

mod cmd;
mod ctl;
mod line_edit;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Cmd,
    LineInsert,
    LineEdit,
}

impl Mode {
    pub(crate) fn process_key(&self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<()> {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            ctl::process_ctl(key, tui)?
        } else {
            match self {
                Mode::Cmd => cmd::process_cmd(key, tui)?,
                Mode::LineInsert => cmd::process_cmd(key, tui)?,
                Mode::LineEdit => line_edit::process_line_edit(key, tui)?,
            }
        }

        tui.flush()?;

        Ok(())
    }

    pub fn show_cursor(&self) -> bool {
        !matches!(self, Mode::Cmd)
    }
}
