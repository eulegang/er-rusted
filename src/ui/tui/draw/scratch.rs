use super::*;

pub struct ScratchDrawCmd;

impl DrawCmd for ScratchDrawCmd {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        let (_, height) = size()?;
        let height = height as usize;
        let lines = tui.interp.scratch.buffer_lines(height);

        let blank = height - lines.len();

        tui.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?;

        for _ in 0..blank {
            tui.stdout
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(style("~").with(Color::Blue)))?
                .queue(MoveToNextLine(1))?;
        }

        for line in lines.iter().rev() {
            tui.stdout
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(line))?
                .queue(MoveToNextLine(1))?;
        }

        tui.stdout.queue(cursor::RestorePosition)?;

        Ok(())
    }
}
