use super::Tui;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io::Write;

mod cmd;
mod line_edit;
mod line_insert;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Cmd,
    LineEdit,
    LineInsert,
}

impl Mode {
    pub(crate) fn process_key(&self, key: KeyEvent, tui: &mut Tui) -> eyre::Result<bool> {
        match self {
            Mode::Cmd => cmd::process_cmd(key, tui),
            Mode::LineEdit => line_edit::process_line_edit(key, tui),
            Mode::LineInsert => line_insert::process_line_insert(key, tui),
        }
    }

    pub fn show_cursor(&self) -> bool {
        !matches!(self, Mode::Cmd)
    }
}
