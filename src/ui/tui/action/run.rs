use super::Action;
use crate::ui::tui::Tui;
use crate::Command;
use std::str::FromStr;

pub struct RunCmd<'a>(pub &'a str);

impl Action for RunCmd<'_> {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.history.reset();

        if !self.0.trim().is_empty() {
            tui.history.append(self.0.to_string());
        }

        if let Ok(cmd) = Command::from_str(self.0) {
            if cmd.needs_text() {
                tui.draw_error()?.flush()?;
                return Ok(());
            }

            match tui.interp.exec(&cmd) {
                Ok(false) => {
                    tui.pending_quit = true;
                }

                Ok(true) => {
                    tui.draw_cmdline("")?.draw_buffer()?;
                }

                Err(()) => {
                    tui.draw_error()?;
                }
            }
        } else {
            tui.draw_error()?;
        }

        Ok(())
    }
}
