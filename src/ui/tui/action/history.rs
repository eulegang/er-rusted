use super::Action;
use crate::ui::tui::Tui;
use std::mem::take;

pub enum History {
    Past,
    Recent,
}

impl Action for History {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        let hist = &mut tui.history;
        match self {
            History::Past => {
                if !hist.active() {
                    hist.hold(take(&mut tui.cmd));
                }

                if let Some(cmd) = hist.up() {
                    tui.cmd = cmd.to_string();
                    tui.draw_cmd()?;
                }
            }

            History::Recent => {
                if hist.active() {
                    if let Some(cmd) = hist.down() {
                        tui.cmd = cmd.to_string();
                    } else {
                        tui.cmd = hist.take().unwrap_or_default();
                    }
                    tui.draw_cmd()?;
                }
            }
        }

        Ok(())
    }
}
