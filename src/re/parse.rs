use super::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::one_of,
    combinator::{all_consuming, eof, map, opt},
    multi::many0,
    sequence::{pair, preceded},
    IResult,
};

impl FromStr for Pat {
    type Err = ();

    fn from_str(s: &str) -> Result<Pat, ()> {
        Ok(all_consuming(pat_parse)(s).or(Err(()))?.1)
    }
}

fn pat_parse(input: &str) -> IResult<&str, Pat> {
    let (_, s) = opt(pair(tag("%"), eof))(input)?;
    if s.is_some() {
        return Ok(("", Pat::Replay));
    }

    let litp = map(is_not("&\\"), |s: &str| Expansion::Lit(s.to_string()));
    let wholep = map(tag("&"), |_| Expansion::Whole);
    let groupp = map(preceded(tag("\\"), one_of("0123456789%&")), |e| {
        if let Some(mag) = e.to_digit(10) {
            Expansion::Pos(mag as usize)
        } else {
            Expansion::Lit(e.to_string())
        }
    });

    let (input, exps) = many0(alt((wholep, groupp, litp)))(input)?;
    Ok((input, Pat::Expansion(exps)))
}
