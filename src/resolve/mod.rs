use crate::addr::{Address, Offset, Point};
use crate::buffer::Buffer;

pub trait RangeResolver {
    fn resolve_range(&self, buffer: &Buffer) -> Option<(usize, usize)>;
}

impl RangeResolver for Address {
    fn resolve_range(&self, buffer: &Buffer) -> Option<(usize, usize)> {
        match self {
            Address::Line(offset) => {
                let pos = offset.resolve_line(buffer);

                pos.zip(pos)
            }

            Address::Range { start, end } => {
                start.resolve_line(buffer).zip(end.resolve_line(buffer))
            }
        }
    }
}

pub trait LineResolver {
    fn resolve_line(&self, buffer: &Buffer) -> Option<usize>;
}

impl LineResolver for Offset {
    fn resolve_line(&self, buffer: &Buffer) -> Option<usize> {
        match self {
            Offset::Nil(point) => point.resolve_line(buffer),
            Offset::Relf(point, offset) => point.resolve_line(buffer).map(|i| i + offset),
            Offset::Relb(point, offset) => point.resolve_line(buffer).map(|i| i - offset),
        }
    }
}

impl LineResolver for Point {
    fn resolve_line(&self, buffer: &Buffer) -> Option<usize> {
        match self {
            Point::Current => Some(buffer.cursor()),
            Point::Abs(s) => Some(s.clone()),
            Point::Mark(ch) => buffer.mark(*ch),
            Point::Last => Some(buffer.len()),

            Point::Ref(re) => {
                let mut i = buffer.cursor() + 1;

                while let Some(line) = buffer.line(i) {
                    if re.is_match(line) {
                        return Some(i);
                    }

                    i += 1;
                }

                None
            }

            Point::Reb(re) => {
                let mut i = buffer.cursor() - 1;

                while let Some(line) = buffer.line(i) {
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
