use super::*;

pub struct RotateWindowLock;

impl Action for RotateWindowLock {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        tui.window_lock = tui.window_lock.next();
        tui.draw_buffer()?;

        Ok(())
    }
}
