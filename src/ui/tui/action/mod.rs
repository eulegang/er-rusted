use super::Tui;

mod edit;
mod history;
mod mode;
mod run;
mod scroll;
mod shift;
mod transition;

pub use edit::Edit;
pub use history::History;
pub use mode::SetMode;
pub use run::{Reset, Run};
pub use scroll::Scroll;
pub use shift::Shift;
pub use transition::Transition;

pub trait Action {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()>;
}
