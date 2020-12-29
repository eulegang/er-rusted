use super::Parsable;
use crate::addr::{Offset, Point};
use nom::{
    character::complete::{digit0, one_of},
    combinator::opt,
    sequence::pair,
    IResult,
};
impl Parsable for Offset {
    fn parse(input: &str) -> IResult<&str, Offset> {
        let (input, p) = opt(Point::parse)(input)?;

        let (input, parts) = opt(pair(one_of("+-"), digit0))(input)?;

        if p.is_none() && parts.is_none() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fix,
            )));
        }

        let point = p.unwrap_or(Point::Current);

        match parts {
            Some(('+', "")) => Ok((input, Offset::Relf(point, 1))),
            Some(('-', "")) => Ok((input, Offset::Relb(point, 1))),
            Some(('+', mag)) => Ok((input, Offset::Relf(point, mag.parse().unwrap()))),
            Some(('-', mag)) => Ok((input, Offset::Relb(point, mag.parse().unwrap()))),
            None => Ok((input, Offset::Nil(point))),
            _ => unreachable!(),
        }
    }
}
