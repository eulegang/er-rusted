use super::Parsable;
use crate::VALID_MARKS;
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{digit1, one_of},
    IResult,
};

use crate::addr::Point;
use crate::re::Re;
use std::str::FromStr;

const FRE_ESCAPES: &str = "\\.+*?()|[]{}^$?\"/dDwWsS";
const BRE_ESCAPES: &str = "\\.+*?()|[]{}^$?\"dDwWsS";

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
