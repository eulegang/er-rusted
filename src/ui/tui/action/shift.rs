use super::Action;
use crate::ui::tui::{motion::Motion, Tui};
use std::cmp::{max, min};

pub struct Shift<M: Motion> {
    pub mag: usize,
    pub motion: M,
}

pub struct CutShift<M: Motion> {
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

impl<M> Action for CutShift<M>
where
    M: Motion,
{
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        let CutShift { mag, motion } = self;

        let mut pos = tui.cursor;
        for _ in 0..*mag {
            if let Some(cursor) = motion.move_cursor(&tui.cmd, pos) {
                pos = cursor;
            }
        }

        if pos != tui.cursor {
            let (low, high) = (min(pos, tui.cursor), max(pos, tui.cursor));

            tui.cmd.drain(low..=high);
            tui.cursor = low;
        }

        tui.draw_cmd()?.draw_cursor()?;

        Ok(())
    }
}

impl<M> Shift<M>
where
    M: Motion,
{
    pub fn to_cut(self) -> CutShift<M> {
        let Shift { mag, motion } = self;
        CutShift { mag, motion }
    }
}
