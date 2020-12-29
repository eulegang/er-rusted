use crate::interp::Interpreter;
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

pub trait RangeResolver {
    fn resolve_range(&self, interp: &Interpreter) -> Option<(usize, usize)>;
}

impl RangeResolver for Address {
    fn resolve_range(&self, interp: &Interpreter) -> Option<(usize, usize)> {
        match self {
            Address::Line(offset) => {
                let pos = offset.resolve_line(interp);

                pos.zip(pos)
            }

            Address::Range { start, end } => {
                start.resolve_line(interp).zip(end.resolve_line(interp))
            }
        }
    }
}

pub trait LineResolver {
    fn resolve_line(&self, interp: &Interpreter) -> Option<usize>;
}

impl LineResolver for Offset {
    fn resolve_line(&self, interp: &Interpreter) -> Option<usize> {
        match self {
            Offset::Nil(point) => point.resolve_line(interp),
            Offset::Relf(point, offset) => point.resolve_line(interp).map(|i| i + offset),
            Offset::Relb(point, offset) => point.resolve_line(interp).map(|i| i - offset),
        }
    }
}

impl LineResolver for Point {
    fn resolve_line(&self, interp: &Interpreter) -> Option<usize> {
        match self {
            Point::Current => Some(interp.buffer.cur),
            Point::Abs(s) => Some(s.clone()),
            Point::Mark(ch) => interp.env.marks.get(ch).cloned(),
            Point::Last => Some(interp.buffer.lines()),

            Point::Ref(re) => {
                let mut i = interp.buffer.cur + 1;

                while let Some(line) = interp.buffer.line(i) {
                    if re.is_match(line) {
                        return Some(i);
                    }

                    i += 1;
                }

                None
            }

            Point::Reb(re) => {
                let mut i = interp.buffer.cur - 1;

                while let Some(line) = interp.buffer.line(i) {
                    if re.is_match(line) {
                        return Some(i);
                    }

                    i -= 1;
                }

                None
            }
        }
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
