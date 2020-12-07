use super::*;
use crate::Parsable;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{digit0, digit1, one_of},
    combinator::{all_consuming, opt},
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

            Some('d') => Ok((
                input,
                Command::Print(addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)))),
            )),

            None => match addr {
                Some(Address::Line(offset)) => Ok((input, Command::Nop(offset))),
                None => Ok((input, Command::Nop(Offset::Nil(Point::Current)))),
                Some(addr) => Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fix,
                ))),
            },

            _ => unreachable!(),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, ()> {
        Ok(all_consuming(Command::parse)(input).or(Err(()))?.1)
    }
}
