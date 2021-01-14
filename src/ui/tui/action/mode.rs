use super::Action;
use crate::ui::tui::{mode::Mode, Tui};

pub struct SetMode(pub Mode);

impl Action for SetMode {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        match (tui.mode.show_cursor(), self.0.show_cursor()) {
            (true, false) => tui.hide_cursor()?,
            (false, true) => tui.show_cursor()?,
            _ => tui,
        };

        tui.mode = self.0;

        tui.draw_cursor()?;

        Ok(())
    }
}
