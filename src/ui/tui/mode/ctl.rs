use super::*;
use crate::ui::tui::action::*;

pub(crate) fn process_ctl(key: KeyEvent, tui: &mut Tui) -> eyre::Result<()> {
    match key.code {
        KeyCode::Char('c') => Reset.invoke(tui)?,
        KeyCode::Char('d') => Scroll::HalfDown.invoke(tui)?,
        KeyCode::Char('u') => Scroll::HalfUp.invoke(tui)?,
        KeyCode::Char('f') => Scroll::FullDown.invoke(tui)?,
        KeyCode::Char('b') => Scroll::FullUp.invoke(tui)?,
        KeyCode::Char('l') => RotateWindowLock::Down.invoke(tui)?,
        KeyCode::Char('o') => RotateWindowLock::Up.invoke(tui)?,
        _ => (),
    };

    Ok(())
}
