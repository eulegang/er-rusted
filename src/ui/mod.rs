//! Gives an interface for users to use er-rusted

mod repl;
mod script;

pub use repl::Repl;
pub use script::Script;

/// A trait to interact with a user
pub trait UI: Sized {
    /// Interact with the user
    fn run(&mut self) -> eyre::Result<()>;
}
