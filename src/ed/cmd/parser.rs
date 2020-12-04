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

use Address::*;
use Offset::*;
use Point::*;

impl Parsable for Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        let (input, addr) = opt(Address::parse)(input)?;

        let (input, op) = opt(one_of("pd"))(input)?;

        match op {
            Some('p') => Ok((
                input,
                Command::Print(addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)))),
            )),

            _ => unreachable!(),
        }
    }
}
