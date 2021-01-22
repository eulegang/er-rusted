use super::Action;
use crate::ui::tui::draw::*;
use crate::ui::tui::Tui;
use crossterm::terminal::size;

pub enum Scroll {
    FullUp,
    HalfUp,
    HalfDown,
    FullDown,
}

impl Action for Scroll {
    fn invoke(&self, tui: &mut Tui) -> crossterm::Result<()> {
        let (_, w) = size()?;
        let w: usize = w.into();
        match self {
            Scroll::FullUp => {
                tui.interp.buffer.scroll_backward(w);
            }
            Scroll::HalfUp => {
                let w: usize = w / 2;

                tui.interp.buffer.scroll_backward(w);
            }
            Scroll::HalfDown => {
                let w: usize = w / 2;

                tui.interp.buffer.scroll_forward(w);
            }
            Scroll::FullDown => {
                tui.interp.buffer.scroll_forward(w);
            }
        }

        BufferDrawCmd.draw(tui)?;

        Ok(())
    }
}
