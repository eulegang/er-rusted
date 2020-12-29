use super::Parsable;
use crate::cmd::Cmd;
use nom::{bytes::complete::tag, combinator::opt, IResult};

impl Parsable for Cmd {
    fn parse(input: &str) -> IResult<&str, Cmd> {
        let (input, _) = tag("!")(input)?;
        let (input, sig) = opt(tag("!"))(input)?;

        match sig {
            Some("!") => Ok((input, Cmd::Repeat)),
            None => Ok(("", Cmd::System(input.trim().to_string()))),

            _ => unreachable!(),
        }
    }
}
