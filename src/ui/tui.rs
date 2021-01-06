use super::UI;
use crate::{Command, Interpreter};
use crossterm::{
    cursor::{self, MoveTo, MoveToNextLine},
    event::{read, Event, KeyCode, KeyModifiers},
    style::{style, Color, Print},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    QueueableCommand,
};
use eyre::WrapErr;
use std::io::{self, Stdout, Write};
use std::str::FromStr;

/// Create a tui similar to vim
pub struct Tui {
    interp: Interpreter,
    stdout: Stdout,
    window_lock: Option<WindowLock>,
    cmd: String,
}

impl Tui {
    /// Creates a new tui
    pub fn new(files: Vec<String>) -> eyre::Result<Self> {
        let interp = Interpreter::new(files).wrap_err("failed to build")?;
        let stdout = std::io::stdout();
        let cmd = String::new();
        let window_lock = Some(WindowLock::Top);

        Ok(Tui {
            interp,
            stdout,
            window_lock,
            cmd,
        })
    }

    fn draw_buffer(&mut self) -> eyre::Result<&mut Self> {
        let (_, height) = size()?;
        let mut height: usize = height.into();
        let mut offset = 0;
        self.queue(MoveTo(1, 0))?
            .queue(Clear(ClearType::FromCursorDown))?;

        let pad = digits(self.interp.buffer.lines());

        if let Some(lock) = &self.window_lock {
            match lock.find_pos(height, self.interp.buffer.cursor()) {
                Ok(s) => offset = s,

                Err(s) => {
                    for _ in 1..s {
                        self.queue(MoveToNextLine(1))?
                            .queue(Print(style("~").with(Color::Blue)))?;
                    }

                    height -= s - 1;
                }
            };
        }

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

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    fn init(&mut self) -> eyre::Result<()> {
        self.queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?
            .queue(MoveTo(0, 0))?
            .queue(Print(": "))?
            .flush()?;

        self.draw_buffer()?;

        Ok(())
    }

    fn do_run(&mut self) -> eyre::Result<()> {
        self.init()?;

        loop {
            if !self.process(read()?)? {
                break Ok(());
            }
        }
    }

    fn print_cmd(&mut self, clear: bool) -> eyre::Result<&mut Self> {
        let cmd = self.cmd.as_str();
        self.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?;

        if clear {
            self.stdout.queue(Clear(ClearType::CurrentLine))?;
        }

        self.stdout
            .queue(Print(": "))?
            .queue(Print(cmd))?
            .queue(cursor::RestorePosition)?;

        Ok(self)
    }

    fn display_error(&mut self) -> eyre::Result<&mut Self> {
        self.stdout
            .queue(cursor::SavePosition)?
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(style("* ").with(Color::Red)))?;

        Ok(self)
    }

    fn process(&mut self, event: Event) -> eyre::Result<bool> {
        match event {
            Event::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('c') => {
                            self.cmd.clear();
                            self.print_cmd(true)?.flush()?;
                        }

                        KeyCode::Char('d') => {
                            let (_, w) = size()?;
                            let w: usize = (w / 2).into();

                            self.interp.buffer.scroll_forward(w);
                            self.draw_buffer()?.flush()?;
                        }

                        KeyCode::Char('u') => {
                            let (_, w) = size()?;
                            let w: usize = (w / 2).into();

                            self.interp.buffer.scroll_backward(w);
                            self.draw_buffer()?.flush()?;
                        }

                        KeyCode::Char('f') => {
                            let (_, w) = size()?;
                            let w: usize = w.into();

                            self.interp.buffer.scroll_forward(w);
                            self.draw_buffer()?.flush()?;
                        }

                        KeyCode::Char('b') => {
                            let (_, w) = size()?;
                            let w: usize = w.into();

                            self.interp.buffer.scroll_backward(w);
                            self.draw_buffer()?.flush()?;
                        }

                        KeyCode::Char('l') => {
                            self.window_lock = self.window_lock.as_ref().map(|l| l.next());
                            self.draw_buffer()?.flush()?;
                        }

                        _ => (),
                    }
                } else {
                    match key.code {
                        KeyCode::Char(ch) => {
                            self.cmd.push(ch);
                            self.print_cmd(false)?.flush()?;
                        }

                        KeyCode::Backspace => {
                            self.cmd.pop();
                            self.print_cmd(true)?.flush()?;
                        }

                        KeyCode::Enter => {
                            if let Ok(cmd) = Command::from_str(&self.cmd) {
                                self.cmd.clear();

                                if cmd.needs_text() {
                                    self.display_error()?.flush()?;
                                    return Ok(true);
                                }

                                match self.interp.exec(&cmd) {
                                    Ok(false) => return Ok(false),
                                    Ok(true) => {
                                        self.print_cmd(true)?.draw_buffer()?.flush()?;
                                    }
                                    Err(()) => {
                                        self.display_error()?.flush()?;
                                    }
                                }
                            } else {
                                self.cmd.clear();
                                self.display_error()?.flush()?;
                            }
                        }

                        _ => (),
                    };
                }

                Ok(true)
            }

            _ => Ok(true),
        }
    }
}

impl UI for Tui {
    fn run(&mut self) -> eyre::Result<()> {
        enable_raw_mode()?;
        let res = self.do_run();

        if res.is_ok() {
            self.queue(Clear(ClearType::All))?
                .queue(cursor::Show)?
                .queue(cursor::MoveTo(0, 0))?
                .flush()?;
        }

        res
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
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

#[derive(Debug, PartialEq)]
enum WindowLock {
    Top,
    Perc20,
    Middle,
    Perc80,
    Bottom,
}

impl WindowLock {
    ///  finds the position to start rendering or err if negative space should be used
    fn find_pos(&self, height: usize, cur: usize) -> Result<usize, usize> {
        let diff = match self {
            WindowLock::Top => 1,
            WindowLock::Perc20 => height / 5,
            WindowLock::Middle => height / 2,
            WindowLock::Perc80 => height - (height / 5),
            WindowLock::Bottom => height,
        };

        if let Some(s) = cur.checked_sub(diff) {
            Ok(s)
        } else {
            Err(diff - cur)
        }
    }

    fn next(&self) -> WindowLock {
        match self {
            WindowLock::Top => WindowLock::Perc20,
            WindowLock::Perc20 => WindowLock::Middle,
            WindowLock::Middle => WindowLock::Perc80,
            WindowLock::Perc80 => WindowLock::Bottom,
            WindowLock::Bottom => WindowLock::Top,
        }
    }
}
