//! Gives an interface for users to use er-rusted

mod repl;
pub use repl::Repl;

/// A trait to interact with a user
pub trait UI: Sized {
    /// Creates the ui with the files to be edited
    fn new(files: Vec<String>) -> eyre::Result<Self>;
    /// Interact with the user
    fn run(&mut self) -> eyre::Result<()>;
}
