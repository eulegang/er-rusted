use super::Action;
use crate::ui::tui::{mode::Mode, Tui};

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
            Transition::Append => tui.cursor += 1,
            Transition::HardInsert => tui.cursor = 0,
            Transition::HardAppend => tui.cursor = tui.cmd.len() - 1,
        }

        tui.mode = Mode::LineInsert;

        tui.draw_cursor()?;

        Ok(())
    }
}
