use super::{Command, Sink};
use crate::{
    addr::{Address, Offset, Point},
    cmd::SubstFlags,
    re::{Pat, Re},
    Parsable, VALID_MARKS,
};
use std::str::FromStr;

use nom::{
    branch::{alt, permutation},
    bytes::complete::{escaped, is_not, tag},
    character::complete::{digit1, multispace0, one_of},
    combinator::{all_consuming, cond, eof, opt},
    IResult,
};

impl Parsable for Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        let (input, addr) = opt(Address::parse)(input)?;

        let (input, op) = opt(one_of("pdacikjqmtyxsw"))(input)?;

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

            Some('w') => {
                let (input, _) = multispace0(input)?;
                let (input, sink) = Sink::parse(input)?;

                let addr = addr.unwrap_or(Address::Range {
                    start: Offset::Nil(Point::Abs(1)),
                    end: Offset::Nil(Point::Last),
                });

                Ok((input, Command::Write(addr, sink)))
            }

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
                let (input, re_str) = opt(escaped(
                    is_not(&*format!("{}\\", sep)),
                    '\\',
                    one_of("\\.+*?()|[]{}^$?\"/dDwWsS"),
                ))(input)?;

                let re = re_str
                    .map(Re::from_str)
                    .transpose()
                    .or(Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Fix,
                    ))))?;

                let (input, prepat) = opt(one_of(&*sep.to_string()))(input)?;

                let (input, pat_str) = opt(escaped(
                    is_not(&*format!("{}\\", sep)),
                    '\\',
                    one_of("\\&%"),
                ))(input)?;

                let pat = if prepat.is_some() {
                    Some(Pat::from_str(pat_str.unwrap_or("")).or(Err(nom::Err::Error(
                        nom::error::Error::new(input, nom::error::ErrorKind::Fix),
                    )))?)
                } else {
                    None
                };

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
                Ok((input, Command::Subst(addr, re, pat, flags)))
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

impl Parsable for Sink {
    fn parse(input: &str) -> IResult<&str, Sink> {
        let (input, sel) = opt(alt((tag("!"), eof)))(input)?;

        match sel {
            Some("") => Ok((input, Sink::Filename)),
            None => Ok(("", Sink::File(input.to_string()))),
            Some("!") => Ok(("", Sink::Command(input.trim().to_string()))),

            _ => unreachable!(),
        }
    }
}
