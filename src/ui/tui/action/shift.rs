use super::Action;
use crate::ui::tui::{motion::Motion, Tui};

pub struct Shift<M: Motion> {
    pub mag: usize,
    pub motion: M,
}

impl<M> Action for Shift<M>
where
    M: Motion,
{
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        let Shift { mag, motion } = self;

        for _ in 0..*mag {
            if let Some(cursor) = motion.move_cursor(&tui.cmd, tui.cursor) {
                tui.cursor = cursor;
            }
        }

        tui.draw_cursor()?;

        Ok(())
    }
}
