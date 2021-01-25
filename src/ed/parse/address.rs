use super::Parsable;
use crate::ed::addr::{Address, Offset, Point};
use nom::{character::complete::one_of, combinator::opt, IResult};

impl Parsable for Address {
    fn parse(input: &str) -> IResult<&str, Address> {
        let (input, range) = opt(one_of("%"))(input)?;
        if range.is_some() {
            return Ok((
                input,
                Address::Range {
                    start: Offset::Nil(Point::Abs(1)),
                    end: Offset::Nil(Point::Last),
                },
            ));
        }

        let (input, start) = opt(Offset::parse)(input)?;
        let (input, sep) = opt(one_of(",;"))(input)?;

        if start.is_none() && sep.is_none() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fix,
            )));
        }

        match sep {
            Some(',') => {
                let (input, end) = opt(Offset::parse)(input)?;

                Ok((
                    input,
                    Address::Range {
                        start: start.unwrap_or(Offset::Nil(Point::Abs(1))),
                        end: end.unwrap_or(Offset::Nil(Point::Last)),
                    },
                ))
            }
            Some(';') => {
                let (input, end) = opt(Offset::parse)(input)?;

                Ok((
                    input,
                    Address::Range {
                        start: start.unwrap_or(Offset::Nil(Point::Current)),
                        end: end.unwrap_or(Offset::Nil(Point::Last)),
                    },
                ))
            }

            None => Ok((input, Address::Line(start.unwrap()))),

            _ => unreachable!(),
        }
    }
}
