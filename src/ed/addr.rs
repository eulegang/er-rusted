use crate::re::Re;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Point {
    Current,
    Last,
    Absolute(usize),
    RegexForward(Re),
    RegexBackward(Re),
    Mark(char),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Offset<T> {
    Nil(T),
    Relf(T, usize),
    Relb(T, usize),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Address {
    Line(Offset<Point>),
    Range(Offset<Point>, Offset<Point>),
}

impl Default for Point {
    fn default() -> Point {
        Point::Current
    }
}

impl<T> Default for Offset<T>
where
    T: Default,
{
    fn default() -> Offset<T> {
        Offset::Nil(Default::default())
    }
}
