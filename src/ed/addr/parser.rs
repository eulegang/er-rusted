use super::*;
use crate::Parsable;
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{digit0, digit1, one_of},
    combinator::opt,
    sequence::pair,
    IResult,
};
use std::str::FromStr;

const VALID_MARKS: &str = "abcdefghijklmnopqrstuvwxyz_<>";
const FRE_ESCAPES: &str = "\\.+*?()|[]{}^$?\"/dDwWsS";
const BRE_ESCAPES: &str = "\\.+*?()|[]{}^$?\"dDwWsS";

impl Parsable for Address {
    fn parse(input: &str) -> IResult<&str, Address> {
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

impl Parsable for Point {
    fn parse(input: &str) -> IResult<&str, Point> {
        let (input, b) = alt((tag("."), tag("$"), tag("'"), digit1, tag("?"), tag("/")))(input)?;

        match b.chars().next().unwrap() {
            '.' => Ok((input, Point::Current)),

            '$' => Ok((input, Point::Last)),

            '0'..='9' => {
                let addr = b.parse().unwrap();
                Ok((input, Point::Abs(addr)))
            }

            '\'' => {
                let (input, mark) = one_of(VALID_MARKS)(input)?;
                Ok((input, Point::Mark(mark)))
            }

            '?' => {
                let (input, re) = escaped(is_not("\\?"), '\\', one_of(BRE_ESCAPES))(input)?;
                let (input, _) = tag("?")(input)?;

                let re = re.to_string().replace("\\?", "?");

                let re = Re::from_str(&re).expect("figure out a way to pass regex err");
                Ok((input, Point::Reb(re)))
            }

            '/' => {
                let (input, re) = escaped(is_not("\\/"), '\\', one_of(FRE_ESCAPES))(input)?;
                let (input, _) = tag("/")(input)?;

                let re = Re::from_str(&re).expect("figure out a way to pass regex err");
                Ok((input, Point::Ref(re)))
            }

            _ => unreachable!(),
        }
    }
}
