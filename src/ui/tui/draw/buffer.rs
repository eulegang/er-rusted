use super::*;

use crate::Buffer;

pub struct BufferDrawCmd;

impl DrawCmd for BufferDrawCmd {
    fn draw<'t>(&self, tui: &'t mut Tui) -> crossterm::Result<()> {
        let (_, height) = size()?;
        let mut height: usize = height.into();
        let mut offset = 0;

        let buffer = &tui.interp.buffer;
        let window_lock = &tui.window_lock;

        tui.stdout
            .queue(MoveTo(1, 0))?
            .queue(Clear(ClearType::FromCursorDown))?;

        let pad = digits(buffer.len());

        match window_lock.find_pos(height, buffer.cursor()) {
            Ok(s) => offset = s,

            Err(s) => {
                for _ in 1..s {
                    tui.stdout
                        .queue(MoveToNextLine(1))?
                        .queue(Print(style("~").with(Color::Blue)))?;
                }

                height -= s - 1;
            }
        };

        if offset == 0 {
            offset += 1;
        }

        let lines = buffer.window(offset, height);
        height -= lines.len();

        for line in lines {
            let pos = offset;
            offset += 1;
            tui.stdout
                .queue(MoveToNextLine(1))?
                .queue(Print(self.form_line(&buffer, pos, pad, line)))?;
        }

        for _ in 0..height {
            tui.stdout
                .queue(MoveToNextLine(1))?
                .queue(Print(style("~").with(Color::Blue)))?;
        }

        Ok(())
    }
}

impl BufferDrawCmd {
    fn form_line(&self, buffer: &Buffer, pos: usize, pad: usize, line: &str) -> String {
        if pos == buffer.cursor() {
            format!("{0:1$} {2}", style(pos).with(Color::Blue), pad, line)
        } else if let Some(ch) = buffer.has_mark(pos) {
            format!(
                "{0:1$} {2}",
                style(format!("'{}", ch)).with(Color::Magenta),
                pad,
                line
            )
        } else {
            format!("{0:1$} {2}", style(pos).with(Color::Yellow), pad, line)
        }
    }
}

fn digits(mut x: usize) -> usize {
    let mut result = 0;
    loop {
        if x == 0 {
            break result;
        }

        x /= 10;
        result += 1;
    }
}
