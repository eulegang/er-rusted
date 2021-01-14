use super::Tui;
use crossterm::event::{Event, KeyCode, KeyModifiers};
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
    pub(crate) fn process(&self, event: Event, tui: &mut Tui) -> eyre::Result<bool> {
        match self {
            Mode::Cmd => cmd::process_cmd(event, tui),
            Mode::LineEdit => line_edit::process_line_edit(event, tui),
            Mode::LineInsert => line_insert::process_line_insert(event, tui),
        }
    }

    pub fn show_cursor(&self) -> bool {
        !matches!(self, Mode::Cmd)
    }
}
