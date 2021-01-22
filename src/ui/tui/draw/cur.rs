use super::*;

pub struct ShowCursorDrawCmd(pub bool);

impl DrawCmd for ShowCursorDrawCmd {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        if self.0 {
            tui.stdout.queue(cursor::Show)?;
        } else {
            tui.stdout.queue(cursor::Hide)?;
        }

        Ok(())
    }
}

pub struct CursorDrawCmd(pub usize);

impl DrawCmd for CursorDrawCmd {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        tui.stdout.queue(cursor::MoveTo(self.0 as u16 + 2, 0))?;

        Ok(())
    }
}
