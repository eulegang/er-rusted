use super::*;

pub struct CmdDrawCmd<'a>(pub &'a str);

impl<'a> DrawCmd for CmdDrawCmd<'a> {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        tui.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(": "))?
            .queue(Print(self.0))?
            .queue(cursor::RestorePosition)?;

        Ok(())
    }
}
