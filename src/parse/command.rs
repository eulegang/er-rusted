use crate::cmd::Command;
use crate::syspoint::{Cmd, SysPoint};
use crate::{
    addr::{Address, Offset, Point},
    cmd::SubstFlags,
    re::{Pat, Re},
    Parsable, VALID_MARKS,
};
use std::str::FromStr;

use nom::{
    bytes::complete::{escaped, is_not, tag},
    character::complete::{multispace0, one_of},
    combinator::{all_consuming, cond, opt},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

macro_rules! nom_bail {
    ($input: expr) => {
        nom::Err::Error(nom::error::Error::new($input, nom::error::ErrorKind::Fix))
    };
}

impl Parsable for Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        let (input, addr) = opt(Address::parse)(input)?;

        if addr.is_none() {
            if let (input, Some(cmd)) = opt(Cmd::parse)(input)? {
                return Ok((input, Command::Run(cmd)));
            }

            if let (input, Some(ch)) = opt(one_of("<>"))(input)? {
                return match ch {
                    '>' => Ok((input, Command::NextBuffer)),
                    '<' => Ok((input, Command::PrevBuffer)),
                    _ => unreachable!(),
                };
            }
        }

        let (input, op) = opt(one_of("pdacikjqmtyxswrgv"))(input)?;

        match op {
            Some('p') => Ok((input, Command::Print(addr.unwrap_or(Address::CURRENT)))),

            Some('d') => Ok((input, Command::Delete(addr.unwrap_or(Address::CURRENT)))),

            Some('j') => Ok((
                input,
                Command::Join(addr.unwrap_or(Address::Range {
                    start: Offset::CURRENT,
                    end: Offset::Relf(Point::Current, 1),
                })),
            )),

            Some('w') => {
                let (input, q) = opt(one_of("q"))(input)?;
                let (input, _) = multispace0(input)?;
                let (input, sink) = SysPoint::parse(input)?;

                let addr = addr.unwrap_or(Address::FULL);

                Ok((input, Command::Write(addr, sink, q.is_some())))
            }

            Some('r') => {
                let (input, _) = multispace0(input)?;
                let (input, src) = SysPoint::parse(input)?;

                let offset = addr
                    .unwrap_or(Address::Line(Offset::Nil(Point::Last)))
                    .to_line()
                    .ok_or(nom_bail!(input))?;

                Ok((input, Command::Read(offset, src)))
            }

            Some('q') => Ok((input, Command::Quit)),

            Some('y') => Ok((input, Command::Yank(addr.unwrap_or(Address::CURRENT)))),

            Some('k') => {
                let (input, mark) = one_of(VALID_MARKS)(input)?;
                let offset = addr
                    .unwrap_or(Address::CURRENT)
                    .to_line()
                    .ok_or(nom_bail!(input))?;

                Ok((input, Command::Mark(offset, mark)))
            }

            Some('m') => {
                let (input, offset) = opt(Offset::parse)(input)?;
                Ok((
                    input,
                    Command::Move(
                        addr.unwrap_or(Address::CURRENT),
                        offset.unwrap_or(Offset::CURRENT),
                    ),
                ))
            }

            Some('t') => {
                let (input, offset) = opt(Offset::parse)(input)?;
                Ok((
                    input,
                    Command::Transfer(
                        addr.unwrap_or(Address::CURRENT),
                        offset.unwrap_or(Offset::CURRENT),
                    ),
                ))
            }

            Some('c') => {
                let addr = addr.unwrap_or(Address::CURRENT);
                let (input, text) = opt(preceded(multispace0, parse_str_lit))(input)?;

                Ok((input, Command::Change(addr, text)))
            }

            Some('i') => {
                let offset = addr
                    .unwrap_or(Address::CURRENT)
                    .to_line()
                    .ok_or(nom_bail!(input))?;

                let (input, text) = opt(preceded(multispace0, parse_str_lit))(input)?;

                Ok((input, Command::Insert(offset, text)))
            }

            Some('a') => {
                let offset = addr
                    .unwrap_or(Address::CURRENT)
                    .to_line()
                    .ok_or(nom_bail!(input))?;

                let (input, text) = opt(preceded(multispace0, parse_str_lit))(input)?;

                Ok((input, Command::Append(offset, text)))
            }

            Some('x') => {
                let offset = addr
                    .unwrap_or(Address::CURRENT)
                    .to_line()
                    .ok_or(nom_bail!(input))?;

                Ok((input, Command::Paste(offset)))
            }

            None => {
                let offset = addr
                    .unwrap_or(Address::CURRENT)
                    .to_line()
                    .ok_or(nom_bail!(input))?;

                Ok((input, Command::Nop(offset)))
            }

            Some('s') if input.is_empty() => {
                let addr = addr.unwrap_or(Address::CURRENT);
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
                    .or(Err(nom_bail!(input)))?;

                let (input, prepat) = opt(one_of(&*sep.to_string()))(input)?;

                let (input, pat_str) = opt(escaped(
                    is_not(&*format!("{}\\", sep)),
                    '\\',
                    one_of("\\&%"),
                ))(input)?;

                let pat = if prepat.is_some() {
                    Some(Pat::from_str(pat_str.unwrap_or("")).or(Err(nom_bail!(input)))?)
                } else {
                    None
                };

                let (input, flags_sep) = opt(tag(&*format!("{}", sep)))(input)?;

                let (input, flags) = cond(flags_sep.is_some(), SubstFlags::parse)(input)?;

                let addr = addr.unwrap_or(Address::CURRENT);
                Ok((input, Command::Subst(addr, re, pat, flags)))
            }

            Some('g') => {
                let (input, _) = tag("/")(input)?;

                let (input, re_str) = opt(escaped(
                    is_not("/"),
                    '\\',
                    one_of("\\.+*?()|[]{}^$?\"/dDwWsS"),
                ))(input)?;

                let (input, _) = tag("/")(input)?;

                let re = re_str
                    .map(Re::from_str)
                    .transpose()
                    .or(Err(nom_bail!(input)))?;

                let (input, cmd_list) = separated_list1(
                    delimited(multispace0, tag("\\\n"), multispace0),
                    Command::parse,
                )(input)?;

                let addr = addr.unwrap_or(Address::FULL);

                Ok((input, Command::Global(addr, re, cmd_list)))
            }

            Some('v') => {
                let (input, _) = tag("/")(input)?;

                let (input, re_str) = opt(escaped(
                    is_not("/"),
                    '\\',
                    one_of("\\.+*?()|[]{}^$?\"/dDwWsS"),
                ))(input)?;

                let (input, _) = tag("/")(input)?;

                let re = re_str
                    .map(Re::from_str)
                    .transpose()
                    .or(Err(nom_bail!(input)))?;

                let (input, cmd_list) = separated_list1(
                    delimited(multispace0, tag("\\\n"), multispace0),
                    Command::parse,
                )(input)?;

                let addr = addr.unwrap_or(Address::FULL);

                Ok((input, Command::Void(addr, re, cmd_list)))
            }
            _ => unreachable!(),
        }
    }
}

fn parse_str_lit(input: &str) -> IResult<&str, Vec<String>> {
    let (input, end) = one_of("\"'")(input)?;
    let (input, content) = escaped(
        is_not(format!("\\{}", end).as_str()),
        '\\',
        one_of(format!("n\\{}", end).as_str()),
    )(input)?;
    let (input, _) = tag(end.to_string().as_str())(input)?;

    let content = content.replace(&format!("\\{}", end), &end.to_string());
    let content = content.replace("\\\\", "\\");

    Ok((input, content.split("\\n").map(|s| s.to_string()).collect()))
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, ()> {
        Ok(all_consuming(Command::parse)(input).or(Err(()))?.1)
    }
}
