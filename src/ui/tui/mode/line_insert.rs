use super::*;
use crate::ui::tui::action::*;

pub(crate) fn process_line_insert(key: KeyEvent, tui: &mut Tui) -> eyre::Result<bool> {
    match key.code {
        KeyCode::Char(ch) => {
            Edit::Insert(ch).invoke(tui)?;
        }

        KeyCode::Backspace => {
            Edit::Backspace.invoke(tui)?;
        }

        KeyCode::Esc => {
            SetMode(Mode::LineEdit).invoke(tui)?;
        }

        KeyCode::Enter => {
            Run.invoke(tui)?;
        }

        _ => (),
    }

    tui.flush()?;
    Ok(true)
}
