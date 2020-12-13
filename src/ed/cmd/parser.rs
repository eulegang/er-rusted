use super::*;
use crate::ed::VALID_MARKS;
use crate::Parsable;
use std::str::FromStr;

use nom::{
    branch::{alt, permutation},
    bytes::complete::{escaped, is_not, tag},
    character::complete::{digit1, one_of},
    combinator::{all_consuming, cond, opt},
    sequence::preceded,
    IResult,
};

impl Parsable for Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        let (input, addr) = opt(Address::parse)(input)?;

        let (input, op) = opt(one_of("pdacikjqmtyxs"))(input)?;

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

            Some('j') => Ok((
                input,
                Command::Join(addr.unwrap_or(Address::Range {
                    start: Offset::Nil(Point::Current),
                    end: Offset::Relf(Point::Current, 1),
                })),
            )),

            Some('q') => Ok((input, Command::Quit)),

            Some('y') => Ok((
                input,
                Command::Yank(addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)))),
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

            Some('m') => {
                let (input, offset) = opt(Offset::parse)(input)?;
                Ok((
                    input,
                    Command::Move(
                        addr.unwrap_or(Address::Line(Offset::Nil(Point::Current))),
                        offset.unwrap_or(Offset::Nil(Point::Current)),
                    ),
                ))
            }

            Some('t') => {
                let (input, offset) = opt(Offset::parse)(input)?;
                Ok((
                    input,
                    Command::Transfer(
                        addr.unwrap_or(Address::Line(Offset::Nil(Point::Current))),
                        offset.unwrap_or(Offset::Nil(Point::Current)),
                    ),
                ))
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

            Some('x') => match addr {
                Some(Address::Line(offset)) => Ok((input, Command::Paste(offset))),
                None => Ok((input, Command::Paste(Offset::Nil(Point::Current)))),
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

            Some('s') if input.is_empty() => {
                let addr = addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)));
                Ok((input, Command::Subst(addr, None, None, None)))
            }

            Some('s') => {
                let (input, sep) = one_of("/^:?")(input)?;
                let (input, re_str) = escaped(
                    is_not(&*format!("{}\\", sep)),
                    '\\',
                    one_of("\\.+*?()|[]{}^$?\"/dDwWsS"),
                )(input)?;

                let re = Re::from_str(re_str).or(Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fix,
                ))))?;

                let (input, pat_str) = opt(preceded(
                    one_of(&*sep.to_string()),
                    escaped(is_not(&*format!("{}\\", sep)), '\\', one_of("\\&%")),
                ))(input)?;

                let pat =
                    pat_str
                        .map(|s| Pat::from_str(s))
                        .transpose()
                        .or(Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Fix,
                        ))))?;

                let (input, flags_sep) = opt(tag(&*format!("{}", sep)))(input)?;

                let (input, flags) = cond(
                    flags_sep.is_some(),
                    permutation((opt(tag("p")), opt(alt((tag("g"), digit1))), opt(tag("p")))),
                )(input)?;

                let flags = flags.map(|(print, occurances, after_print)| {
                    let occurances = match occurances {
                        Some("g") => 0,
                        Some(digits) => digits.parse().unwrap(),
                        None => 1,
                    };

                    let print = print.or(after_print) == Some("p");

                    SubstFlags { print, occurances }
                });

                let addr = addr.unwrap_or(Address::Line(Offset::Nil(Point::Current)));
                Ok((input, Command::Subst(addr, Some(re), pat, flags)))
            }

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
