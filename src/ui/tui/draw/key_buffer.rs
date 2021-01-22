use super::*;

pub struct KeyBufferDrawCmd<'a>(pub &'a str);

impl<'a> DrawCmd for KeyBufferDrawCmd<'a> {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        let (width, _) = size()?;

        tui.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(width - self.0.len() as u16 - 2, 0))?
            .queue(Print(self.0))?
            .queue(cursor::RestorePosition)?;

        Ok(())
    }
}
