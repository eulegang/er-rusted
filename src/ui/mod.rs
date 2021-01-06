//! Gives an interface for users to use er-rusted

mod repl;
mod script;
mod tui;

pub use repl::Repl;
pub use script::Script;
pub use tui::Tui;

/// A trait to interact with a user
pub trait UI: Sized {
    /// Interact with the user
    fn run(&mut self) -> eyre::Result<()>;
}
