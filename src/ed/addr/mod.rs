use crate::re::Re;

mod parser;

#[cfg(test)]
mod test;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Point {
    Current,
    Last,
    Abs(usize),
    Ref(Re),
    Reb(Re),
    Mark(char),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Offset {
    Nil(Point),
    Relf(Point, usize),
    Relb(Point, usize),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Address {
    Line(Offset),
    Range { start: Offset, end: Offset },
}

impl Default for Point {
    fn default() -> Point {
        Point::Current
    }
}

impl Default for Offset {
    fn default() -> Offset {
        Offset::Nil(Default::default())
    }
}
