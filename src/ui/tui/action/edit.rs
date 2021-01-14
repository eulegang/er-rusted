use super::Action;
use crate::ui::tui::Tui;

pub enum Edit {
    Insert(char),
    Backspace,
    CutRest,
    CutAll,
    CutTil(Option<usize>),
}

impl Action for Edit {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        match self {
            Edit::Insert(ch) => {
                tui.cmd.insert(tui.cursor, *ch);
                tui.cursor += 1
            }

            Edit::Backspace => {
                if tui.cursor == tui.cmd.len() {
                    tui.cmd.pop();
                } else {
                    tui.cmd.remove(tui.cursor);
                }
                tui.cursor = tui.cursor.checked_sub(1).unwrap_or(0)
            }

            Edit::CutRest => {
                let end = tui.cmd.len();
                drop(tui.cmd.drain(tui.cursor..end));
            }

            Edit::CutAll => {
                tui.cmd.clear();
                tui.cursor = 0;
            }

            Edit::CutTil(times) => {
                if let Some(set) = times {
                    let end = tui.cmd.len().min(tui.cursor + set);
                    drop(tui.cmd.drain(tui.cursor..end))
                } else {
                    tui.cmd.remove(tui.cursor);
                }
            }
        }

        tui.draw_cmd()?.draw_cursor()?;

        Ok(())
    }
}
