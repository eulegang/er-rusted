use super::UI;
use crate::{interp::scratch::StoreScratchPad, interp::write_hook::WriteHook, Interpreter};
use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use eyre::WrapErr;
use history::History;
use lock::WindowLock;
use mode::{SealedTMode, TMode};
use motion::Search;
use std::env::var;
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
    pub(crate) interp: Interpreter<StoreScratchPad>,
    pub(crate) stdout: Stdout,
    pub(crate) window_lock: WindowLock,
    pub(crate) history: History,
    pub(crate) pending_quit: bool,
    pub(crate) search: Option<Search>,
}

impl Tui {
    /// Creates a new tui
    pub fn new(files: Vec<String>) -> eyre::Result<Self> {
        let interp = Interpreter::new(files).wrap_err("failed to build")?;
        let stdout = std::io::stdout();
        let history = History::new();
        let window_lock = WindowLock::Top;
        let pending_quit = false;
        let search = None;

        Ok(Tui {
            interp,
            stdout,
            window_lock,
            history,
            pending_quit,
            search,
        })
    }

    fn input_loop(&mut self) -> crossterm::Result<()> {
        let mut tmode = SealedTMode::default();
        loop {
            tmode = self.process(tmode, read()?)?;
            if self.pending_quit {
                break Ok(());
            }
        }
    }

    fn process(&mut self, tmode: SealedTMode, event: Event) -> crossterm::Result<SealedTMode> {
        let next = match event {
            Event::Key(key) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    tmode.process_key(key, self)
                } else {
                    tmode.process_ctl_key(key, self)
                }
            }
            _ => Ok(tmode),
        };

        self.flush()?;

        next
    }
}

impl UI for Tui {
    fn run(&mut self) -> eyre::Result<()> {
        if let Ok(hook) = var("ER_WRITE_HOOK") {
            self.interp.env.write_hook = WriteHook::Proc(hook);
        }

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

        let res = self.input_loop().wrap_err("Failed to write to tui");

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
