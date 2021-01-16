mod stdout;
mod store;

pub use stdout::StdoutScratchPad;
pub use store::StoreScratchPad;

pub trait ScratchPad: Default {
    fn print(&mut self, line: &str);
}
