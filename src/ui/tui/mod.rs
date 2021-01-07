use super::UI;
use crate::Interpreter;
use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use eyre::WrapErr;
use history::History;
use lock::WindowLock;
use mode::Mode;
use std::io::Stdout;

mod draw;
mod history;
mod lock;
mod mode;

/// Create a tui similar to vim
pub struct Tui {
    pub(crate) interp: Interpreter,
    pub(crate) stdout: Stdout,
    pub(crate) window_lock: WindowLock,
    pub(crate) history: History,
    pub(crate) mode: Mode,
    pub(crate) cmd: String,
}

impl Tui {
    /// Creates a new tui
    pub fn new(files: Vec<String>) -> eyre::Result<Self> {
        let interp = Interpreter::new(files).wrap_err("failed to build")?;
        let stdout = std::io::stdout();
        let cmd = String::new();
        let history = History::new();
        let window_lock = WindowLock::Top;
        let mode = Mode::Cmd;

        Ok(Tui {
            interp,
            stdout,
            window_lock,
            history,
            mode,
            cmd,
        })
    }

    fn input_loop(&mut self) -> eyre::Result<()> {
        loop {
            if !self.process(read()?)? {
                break Ok(());
            }
        }
    }

    fn process(&mut self, event: Event) -> eyre::Result<bool> {
        self.mode.clone().process(event, self)
    }
}

impl UI for Tui {
    fn run(&mut self) -> eyre::Result<()> {
        enable_raw_mode()?;

        self.queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?
            .queue(MoveTo(0, 0))?
            .queue(Print(": "))?
            .flush()?;

        self.draw_buffer()?;

        let res = self.input_loop();

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
