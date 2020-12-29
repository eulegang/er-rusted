use crate::cmd::SubstFlags;

use super::Parsable;

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    IResult,
};

impl Parsable for SubstFlags {
    fn parse(input: &str) -> IResult<&str, SubstFlags> {
        let (input, (print, occurances, after_print)) =
            permutation((opt(tag("p")), opt(alt((tag("g"), digit1))), opt(tag("p"))))(input)?;

        let occurances = match occurances {
            Some("g") => 0,
            Some(digits) => digits.parse().unwrap(),
            None => 1,
        };

        let print = print.or(after_print) == Some("p");

        Ok((input, SubstFlags { print, occurances }))
    }
}
