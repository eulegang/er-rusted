use super::mode::SetMode;
use super::Action;
use crate::ui::tui::{mode::Mode, Tui};
use std::cmp::min;

pub enum Transition {
    Insert,
    Append,
    HardInsert,
    HardAppend,
}

impl Action for Transition {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        match self {
            Transition::Insert => {}
            Transition::Append => tui.cursor = min(tui.cmd.len(), tui.cursor + 1),
            Transition::HardInsert => tui.cursor = 0,
            Transition::HardAppend => tui.cursor = tui.cmd.len(),
        }

        SetMode(Mode::LineInsert).invoke(tui)?;

        tui.draw_cursor()?;

        Ok(())
    }
}
