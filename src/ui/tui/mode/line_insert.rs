use super::*;
use crate::ui::tui::action::*;

pub(crate) fn process_line_insert(event: Event, tui: &mut Tui) -> eyre::Result<bool> {
    match event {
        Event::Key(key) => match key.code {
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
        },

        _ => (),
    }

    tui.flush()?;
    Ok(true)
}
