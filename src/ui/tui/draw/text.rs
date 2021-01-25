use super::*;
use crate::Buffer;

pub struct InsertTextDrawCmd<'a>(pub usize, pub usize, pub &'a [String]);

impl DrawCmd for InsertTextDrawCmd<'_> {
    fn draw(&self, tui: &mut Tui) -> crossterm::Result<()> {
        let (_, height) = size()?;
        let mut height: usize = height.into();
        let mut offset: usize = 1;

        let buffer = &tui.interp.buffer;
        let window_lock = &tui.window_lock;
        let pad = digits(buffer.len());

        tui.stdout
            .queue(MoveTo(1, 0))?
            .queue(Clear(ClearType::FromCursorDown))?;

        match window_lock.find_pos(height, self.0) {
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

        let lines = buffer.window(offset, height).unwrap();
        height -= lines.len();

        let mid_point = self.0 - offset;

        for pos in 0..mid_point {
            let line = lines[pos];
            let repr = pos + offset;

            tui.stdout
                .queue(MoveToNextLine(1))?
                .queue(Print(self.form_buffer_line(&buffer, repr, pad, line)))?;
        }

        for line in self.2 {
            tui.stdout.queue(MoveToNextLine(1))?.queue(Print(format!(
                "{0:1$} {2}",
                style("+").with(Color::Green),
                pad,
                line,
            )))?;
        }

        for pos in mid_point..lines.len() {
            let line = lines[pos];
            let lineno = pos + offset;

            tui.stdout
                .queue(MoveToNextLine(1))?
                .queue(Print(self.form_buffer_line(&buffer, lineno, pad, line)))?;
        }

        for _ in self.2.len()..height {
            tui.stdout
                .queue(MoveToNextLine(1))?
                .queue(Print(style("~").with(Color::Blue)))?;
        }

        Ok(())
    }
}

impl InsertTextDrawCmd<'_> {
    fn form_buffer_line(&self, buffer: &Buffer, pos: usize, pad: usize, line: &str) -> String {
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
