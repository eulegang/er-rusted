use crate::re::Re;

#[cfg(test)]
mod test;

/// A reference for a line in a buffer
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Point {
    /// The current point in the buffer (the cursor).
    Current,

    /// The last line in the buffer.
    Last,

    /// The specific line in the buffer (1-indexed, but zero is still allowed)
    Abs(usize),

    /// Regex Forward: search forward for a line matching a regex.
    Ref(Re),

    /// Regex Backward: search backward for a line matching a regex.
    Reb(Re),

    /// Lookup a mark in the buffer
    Mark(char),
}

/// A relative offset from a point
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Offset {
    /// Nil offset
    Nil(Point),

    /// Relative forward (down the buffer)
    Relf(Point, usize),

    /// Relative backward (up the buffer)
    Relb(Point, usize),
}

/// An address describles a set of continuous lines
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Address {
    /// Only one line
    Line(Offset),
    /// A range of lines
    Range {
        /// The start of the range
        start: Offset,

        /// The end of the range (inclusive)
        end: Offset,
    },
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
    /// A constent for the current point with a nil offset
    pub const CURRENT: Offset = Offset::Nil(Point::Current);
}

impl Address {
    /// A constent for the current point as an address
    pub const CURRENT: Address = Address::Line(Offset::Nil(Point::Current));

    /// A constent for the whole buffer as an address
    pub const FULL: Address = Address::Range {
        start: Offset::Nil(Point::Abs(1)),
        end: Offset::Nil(Point::Last),
    };

    pub(crate) fn to_line(self) -> Option<Offset> {
        match self {
            Address::Line(offset) => Some(offset),
            _ => None,
        }
    }
}
