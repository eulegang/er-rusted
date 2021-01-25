use super::Action;
use crate::ed::cmd::Command;
use crate::ui::tui::draw::*;
use crate::ui::tui::Tui;
use std::str::FromStr;

pub struct RunCmd<'a>(pub &'a str);

impl Action for RunCmd<'_> {
    fn invoke(&self, tui: &mut Tui) -> crossterm::Result<()> {
        tui.history.reset();

        if !self.0.trim().is_empty() {
            tui.history.append(self.0.to_string());
        }

        if let Ok(cmd) = Command::from_str(self.0) {
            if cmd.needs_text() {
                ErrorDrawCmd("text needed (not supported yet)").draw(tui)?;
                return Ok(());
            }

            match tui.interp.exec(&cmd) {
                Ok(false) => {
                    tui.pending_quit = true;
                }

                Ok(true) => {
                    CmdDrawCmd("").draw(tui)?;
                    BufferDrawCmd.draw(tui)?;
                }

                Err(err) => {
                    ErrorDrawCmd(&format!("{}", err)).draw(tui)?;
                }
            }
        } else {
            ErrorDrawCmd("unable to parse command").draw(tui)?;
        }

        Ok(())
    }
}
