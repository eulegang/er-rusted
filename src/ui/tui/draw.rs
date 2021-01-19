use super::Tui;
use crossterm::{
    cursor::{self, MoveTo, MoveToNextLine},
    style::{style, Color, Print},
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

impl Tui {
    pub(crate) fn draw_buffer(&mut self) -> crossterm::Result<&mut Self> {
        let (_, height) = size()?;
        let mut height: usize = height.into();
        let mut offset = 0;
        self.queue(MoveTo(1, 0))?
            .queue(Clear(ClearType::FromCursorDown))?;

        let pad = digits(self.interp.buffer.lines());

        match self
            .window_lock
            .find_pos(height, self.interp.buffer.cursor())
        {
            Ok(s) => offset = s,

            Err(s) => {
                for _ in 1..s {
                    self.queue(MoveToNextLine(1))?
                        .queue(Print(style("~").with(Color::Blue)))?;
                }

                height -= s - 1;
            }
        };

        for pos in 1..height {
            let pos = pos + offset;
            if let Some(line) = self.interp.buffer.line(pos.into()) {
                self.stdout.queue(MoveToNextLine(1))?;
                self.stdout.queue(Print(self.form_line(pos, pad, line)))?;
            } else {
                self.queue(MoveToNextLine(1))?
                    .queue(Print(style("~").with(Color::Blue)))?;
            }
        }

        self.flush()?;

        Ok(self)
    }

    fn form_line(&self, pos: usize, pad: usize, line: &str) -> String {
        if pos == self.interp.buffer.cursor() {
            format!("{0:1$} {2}", style(pos).with(Color::Blue), pad, line)
        } else if let Some(ch) = self.interp.buffer.has_mark(pos) {
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

    pub(crate) fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    pub(crate) fn draw_cmdline(&mut self, line: &str) -> crossterm::Result<&mut Self> {
        self.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(": "))?
            .queue(Print(line))?
            .queue(cursor::RestorePosition)?;

        Ok(self)
    }

    pub(crate) fn draw_key_buffer(&mut self, keys: &str) -> crossterm::Result<&mut Self> {
        let (width, _) = size()?;

        self.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(width - keys.len() as u16 - 2, 0))?
            .queue(Print(keys))?
            .queue(cursor::RestorePosition)?;

        Ok(self)
    }

    pub(crate) fn draw_error(&mut self) -> crossterm::Result<&mut Self> {
        self.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(style("* ").with(Color::Red)))?
            .queue(cursor::RestorePosition)?;

        Ok(self)
    }

    pub(crate) fn draw_scratch(&mut self) -> crossterm::Result<&mut Self> {
        let (_, height) = size()?;
        let height = height as usize;
        let lines = self.interp.scratch.buffer_lines(height);

        let blank = height - lines.len();

        self.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?;

        for _ in 0..blank {
            self.stdout
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(style("~").with(Color::Blue)))?
                .queue(MoveToNextLine(1))?;
        }

        for line in lines.iter().rev() {
            self.stdout
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(line))?
                .queue(MoveToNextLine(1))?;
        }

        self.stdout.queue(cursor::RestorePosition)?;

        Ok(self)
    }

    pub(crate) fn show_cursor(&mut self) -> crossterm::Result<&mut Self> {
        self.stdout.queue(cursor::Show)?;

        Ok(self)
    }

    pub(crate) fn hide_cursor(&mut self) -> crossterm::Result<&mut Self> {
        self.stdout.queue(cursor::Hide)?;

        Ok(self)
    }

    pub(crate) fn draw_cursor_at(&mut self, cursor: usize) -> crossterm::Result<&mut Self> {
        self.stdout.queue(cursor::MoveTo(cursor as u16 + 2, 0))?;

        Ok(self)
    }
}

impl QueueableCommand for Tui {
    fn queue(&mut self, command: impl crossterm::Command) -> crossterm::Result<&mut Self> {
        self.stdout.queue(command)?;
        Ok(self)
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
