use super::*;
use crate::ui::tui::draw::*;

pub enum RotateWindowLock {
    Down,
    Up,
}

impl Action for RotateWindowLock {
    fn invoke(&self, tui: &mut Tui) -> crossterm::Result<()> {
        match self {
            RotateWindowLock::Down => tui.window_lock = tui.window_lock.next(),
            RotateWindowLock::Up => tui.window_lock = tui.window_lock.prev(),
        }

        BufferDrawCmd.draw(tui)?;

        Ok(())
    }
}
