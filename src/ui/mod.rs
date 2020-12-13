pub mod repl;
pub use repl::Repl;

pub trait UI: Sized {
    fn new(files: Vec<String>) -> eyre::Result<Self>;
    fn run(&mut self) -> eyre::Result<()>;
}
