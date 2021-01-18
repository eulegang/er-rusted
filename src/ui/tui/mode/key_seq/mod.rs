use crate::ui::tui::motion::{Idemp, Motion, SealedMotion, Search};

mod parse;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeySeq {
    pub num: usize,
    pub action: KSAction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KeySeqErr {
    Failed,
    Insufficient,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KSAction {
    Move(Range),
    Delete(Range),
    Change(Range),
    Replace(char),

    Transition(Transition),
    History(History),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Transition {
    Insert,
    Append,
    HardInsert,
    HardAppend,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum History {
    Current,
    Past,
    Recent,
    Last,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Range {
    Motion(SealedMotion),
    RepeatSearch,
    RepeatRevSearch,
    Whole,
}

impl Range {
    pub fn find_next(
        &self,
        buf: &str,
        cursor: usize,
        num: usize,
        search: Option<Search>,
    ) -> Option<usize> {
        fn submove(
            motion: SealedMotion,
            buf: &str,
            mut cursor: usize,
            num: usize,
        ) -> Option<usize> {
            let runs = if motion.is_idempotent() { 1 } else { num };

            for _ in 0..runs {
                cursor = motion.move_cursor(buf, cursor)?;
            }

            Some(cursor)
        }
        match self {
            Range::Whole => None,
            Range::Motion(motion) => submove(*motion, buf, cursor, num),

            Range::RepeatSearch => submove(search?.into(), buf, cursor, num),
            Range::RepeatRevSearch => submove(search?.reverse().into(), buf, cursor, num),
        }
    }
}

impl Transition {
    pub fn update_cursor(&self, buf: &str, cursor: usize) -> usize {
        match self {
            Transition::Insert => cursor,
            Transition::Append => buf.len().min(cursor + 1),
            Transition::HardInsert => 0,
            Transition::HardAppend => buf.len(),
        }
    }
}
