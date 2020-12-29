use super::Parsable;
use crate::syspoint::{Cmd, SysPoint};
use nom::{combinator::opt, IResult};

impl Parsable for SysPoint {
    fn parse(input: &str) -> IResult<&str, SysPoint> {
        if let (input, Some(cmd)) = opt(Cmd::parse)(input)? {
            return Ok((input, SysPoint::Command(cmd)));
        }

        if input.trim().is_empty() {
            return Ok(("", SysPoint::Filename));
        }

        Ok(("", SysPoint::File(input.to_string())))
    }
}
