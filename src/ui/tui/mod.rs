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

mod action;
mod draw;
mod history;
mod lock;
mod mode;
mod motion;

/// Create a tui similar to vim
#[derive(Debug)]
pub struct Tui {
    pub(crate) interp: Interpreter,
    pub(crate) stdout: Stdout,
    pub(crate) window_lock: WindowLock,
    pub(crate) history: History,
    pub(crate) mode: Mode,
    pub(crate) cmd: String,
    pub(crate) key_buffer: String,
    pub(crate) cursor: usize,
    pub(crate) pending_quit: bool,
    pub(crate) search: Option<motion::Search>,
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
        let cursor = 0;
        let key_buffer = String::default();
        let search = None;
        let pending_quit = false;

        Ok(Tui {
            interp,
            stdout,
            window_lock,
            history,
            mode,
            cmd,
            key_buffer,
            cursor,
            search,
            pending_quit,
        })
    }

    fn input_loop(&mut self) -> eyre::Result<()> {
        loop {
            self.process(read()?)?;
            if self.pending_quit {
                break Ok(());
            }
        }
    }

    fn process(&mut self, event: Event) -> eyre::Result<()> {
        let mode = self.mode.clone();
        match event {
            Event::Key(key) => mode.process_key(key, self),
            _ => Ok(()),
        }
    }
}

impl UI for Tui {
    fn run(&mut self) -> eyre::Result<()> {
        enable_raw_mode()?;
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            hook(info)
        }));

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

        if cfg!(debug_assertions) {
            dbg!(self);
        }
    }
}
