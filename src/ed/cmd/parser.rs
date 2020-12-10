use super::*;
use crate::ed::VALID_MARKS;
use crate::Parsable;
use std::str::FromStr;

use nom::{
    character::complete::one_of,
    combinator::{all_consuming, opt},
    IResult,
};

impl Parsable for Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        let (input, addr) = opt(Address::parse)(input)?;

        let (input, op) = opt(one_of("pdacik"))(input)?;

        match op {
            Some('p') => Ok((
                input,
                Command::Print(addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)))),
            )),

            Some('d') => Ok((
                input,
                Command::Delete(addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)))),
            )),

            Some('c') => Ok((
                input,
                Command::Change(addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)))),
            )),

            Some('k') => {
                let (input, mark) = one_of(VALID_MARKS)(input)?;
                match addr {
                    Some(Address::Line(offset)) => Ok((input, Command::Mark(offset, mark))),
                    None => Ok((input, Command::Mark(Offset::Nil(Point::Current), mark))),
                    Some(_) => Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Fix,
                    ))),
                }
            }

            Some('i') => match addr {
                Some(Address::Line(offset)) => Ok((input, Command::Insert(offset))),
                None => Ok((input, Command::Insert(Offset::Nil(Point::Current)))),
                Some(_) => Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fix,
                ))),
            },

            Some('a') => match addr {
                Some(Address::Line(offset)) => Ok((input, Command::Append(offset))),
                None => Ok((input, Command::Append(Offset::Nil(Point::Current)))),
                Some(_) => Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fix,
                ))),
            },

            None => match addr {
                Some(Address::Line(offset)) => Ok((input, Command::Nop(offset))),
                None => Ok((input, Command::Nop(Offset::Nil(Point::Current)))),
                Some(_) => Err(nom::Err::Error(nom::error::Error::new(
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
