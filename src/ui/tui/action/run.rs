use super::Action;
use crate::ui::tui::{mode::Mode, Tui};
use crate::Command;
use std::mem::take;
use std::str::FromStr;

pub struct Run;

impl Action for Run {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.mode = Mode::Cmd;
        tui.hide_cursor()?;
        tui.history.reset();
        tui.key_buffer.clear();
        tui.cursor = 0;

        if let Ok(cmd) = Command::from_str(&tui.cmd) {
            let selected = take(&mut tui.cmd);

            if cmd.needs_text() {
                tui.draw_error()?.flush()?;
                return Ok(());
            }

            match tui.interp.exec(&cmd) {
                Ok(false) => {
                    tui.pending_quit = true;
                }

                Ok(true) => {
                    tui.history.append(selected);
                    tui.draw_cmd()?.draw_buffer()?;
                }

                Err(()) => {
                    tui.draw_error()?;
                }
            }
        } else {
            tui.cmd.clear();
            tui.draw_error()?;
        }

        Ok(())
    }
}
