use crate::re::Re;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Point {
    Current,
    Last,
    Abs(usize),
    Ref(Re),
    Reb(Re),
    Mark(char),
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Offset {
    Nil(Point),
    Relf(Point, usize),
    Relb(Point, usize),
}

#[derive(Debug, Clone)]
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

impl Default for Address {
    fn default() -> Address {
        Address::Line(Default::default())
    }
}

impl Offset {
    pub const CURRENT: Offset = Offset::Nil(Point::Current);
}

impl Address {
    pub const CURRENT: Address = Address::Line(Offset::Nil(Point::Current));
    pub const FULL: Address = Address::Range {
        start: Offset::Nil(Point::Abs(1)),
        end: Offset::Nil(Point::Last),
    };

    pub fn to_line(self) -> Option<Offset> {
        match self {
            Address::Line(offset) => Some(offset),
            _ => None,
        }
    }
}
