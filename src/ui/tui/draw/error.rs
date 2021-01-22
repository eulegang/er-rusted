use super::*;

pub struct ErrorDrawCmd<'a>(pub &'a str);

impl<'a> DrawCmd for ErrorDrawCmd<'a> {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        tui.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(style(format!("* {}", self.0)).with(Color::Red)))?
            .queue(cursor::RestorePosition)?;

        Ok(())
    }
}
