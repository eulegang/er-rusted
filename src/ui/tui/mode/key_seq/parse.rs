use super::*;
use crate::ui::tui::motion::*;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::streaming::is_a,
    character::streaming::{anychar, one_of},
    combinator::{map, opt},
    IResult,
};

impl FromStr for KeySeq {
    type Err = KeySeqErr;

    fn from_str(s: &str) -> Result<KeySeq, KeySeqErr> {
        match parse_key_seq(s) {
            Ok(("", key_seq)) => Ok(key_seq),
            Ok((_, _)) => Err(KeySeqErr::Failed),
            Err(nom::Err::Incomplete(_)) => Err(KeySeqErr::Insufficient),
            Err(nom::Err::Error(_)) => Err(KeySeqErr::Failed),
            Err(nom::Err::Failure(_)) => Err(KeySeqErr::Failed),
        }
    }
}

fn parse_key_seq(input: &str) -> IResult<&str, KeySeq> {
    let (input, num) = parse_num(input)?;
    let (input, action) = parse_action(input)?;

    Ok((input, KeySeq { num, action }))
}

fn parse_num(orig: &str) -> IResult<&str, usize> {
    let (input, first) = opt(one_of("123456789"))(orig)?;
    let first = match first {
        Some(ch) => ch,
        None => return Ok((input, 1)),
    };

    let (input, rest) = opt(is_a("0123456789"))(input)?;

    match (first, rest) {
        (_, Some(rest)) => Ok((input, orig[..rest.len() + 1].parse().unwrap_or(1))),
        (ch, None) => Ok((input, ch.to_digit(10).unwrap() as usize)),
    }
}

fn parse_action(input: &str) -> IResult<&str, KSAction> {
    let parse_move = map(alt((parse_range_basic, parse_range_search)), |motion| {
        KSAction::Move(motion)
    });

    alt((parse_nonmove_action, parse_move))(input)
}

fn parse_nonmove_action(input: &str) -> IResult<&str, KSAction> {
    let (input, ch) = one_of("jkgGiIaAxXCDrdc")(input)?;

    match ch {
        'k' => Ok((input, KSAction::History(History::Past))),
        'j' => Ok((input, KSAction::History(History::Recent))),
        'g' => Ok((input, KSAction::History(History::Last))),
        'G' => Ok((input, KSAction::History(History::Current))),

        'i' => Ok((input, KSAction::Transition(Transition::Insert))),
        'I' => Ok((input, KSAction::Transition(Transition::HardInsert))),
        'a' => Ok((input, KSAction::Transition(Transition::Append))),
        'A' => Ok((input, KSAction::Transition(Transition::HardAppend))),

        'x' => Ok((
            input,
            KSAction::Delete(Range::Motion(Relative::Right.into())),
        )),
        'X' => Ok((
            input,
            KSAction::Delete(Range::Motion(Relative::Left.into())),
        )),

        'D' => Ok((
            input,
            KSAction::Delete(Range::Motion(Absolute::Last.into())),
        )),
        'C' => Ok((
            input,
            KSAction::Change(Range::Motion(Absolute::Last.into())),
        )),

        'r' => {
            let (input, ch) = anychar(input)?;
            Ok((input, KSAction::Replace(ch)))
        }

        'd' => {
            let (input, range) = alt((
                parse_range_whole('d'),
                parse_range_basic,
                parse_range_search,
            ))(input)?;

            Ok((input, KSAction::Delete(range)))
        }

        'c' => {
            let (input, range) = alt((
                parse_range_whole('c'),
                parse_range_basic,
                parse_range_search,
            ))(input)?;

            Ok((input, KSAction::Change(range)))
        }

        _ => unreachable!(),
    }
}

fn parse_range_whole(whole: char) -> impl Fn(&str) -> IResult<&str, Range> {
    move |input: &str| {
        let (input, _) = nom::character::streaming::char(whole)(input)?;

        Ok((input, Range::Whole))
    }
}

fn parse_range_search(input: &str) -> IResult<&str, Range> {
    let (input, ch) = one_of("fFtT")(input)?;
    let (input, search) = match ch {
        'f' => {
            let (input, ch) = anychar(input)?;
            (input, Search::ForwardFind(ch).into())
        }

        'F' => {
            let (input, ch) = anychar(input)?;
            (input, Search::BackwardFind(ch).into())
        }

        't' => {
            let (input, ch) = anychar(input)?;
            (input, Search::ForwardTo(ch).into())
        }

        'T' => {
            let (input, ch) = anychar(input)?;
            (input, Search::BackwardTo(ch).into())
        }

        _ => unreachable!(),
    };

    Ok((input, Range::Motion(search)))
}

fn parse_range_basic(input: &str) -> IResult<&str, Range> {
    let (input, ch) = one_of("hl0$wWbB;,")(input)?;

    let range = match ch {
        'h' => Range::Motion(Relative::Left.into()),
        'l' => Range::Motion(Relative::Right.into()),
        '0' => Range::Motion(Absolute::First.into()),
        '$' => Range::Motion(Absolute::Last.into()),

        'w' => Range::Motion(CClass::ForwardWord.into()),
        'W' => Range::Motion(CClass::ForwardBlank.into()),
        'b' => Range::Motion(CClass::BackwardWord.into()),
        'B' => Range::Motion(CClass::BackwardBlank.into()),

        ';' => Range::RepeatSearch,
        ',' => Range::RepeatRevSearch,

        _ => unreachable!(),
    };

    Ok((input, range))
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_parse {
        ($example: literal, $expected: expr) => {
            assert_eq!(
                KeySeq::from_str($example),
                Ok($expected),
                "expected \"{}\" to parse",
                $example
            )
        };
    }
    macro_rules! refute_parse {
        ($example: literal, $expected: expr) => {
            assert_eq!(
                KeySeq::from_str($example),
                Err($expected),
                "didn't expect \"{}\" to parse",
                $example
            )
        };
    }

    #[test]
    fn num_quant() {
        let action = KSAction::Move(Range::Motion(Relative::Right.into()));
        assert_parse!("l", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Relative::Right.into()));
        assert_parse!("5l", KeySeq { num: 5, action });

        let action = KSAction::Move(Range::Motion(Relative::Right.into()));
        assert_parse!("10l", KeySeq { num: 10, action });
    }

    #[test]
    fn just_num() {
        refute_parse!("5", KeySeqErr::Insufficient);
    }

    #[test]
    fn too_far() {
        refute_parse!("5lrest is foobar", KeySeqErr::Failed);
        refute_parse!("5@", KeySeqErr::Failed);
    }

    #[test]
    fn move_relative() {
        let action = KSAction::Move(Range::Motion(Relative::Right.into()));
        assert_parse!("l", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Relative::Left.into()));
        assert_parse!("h", KeySeq { num: 1, action });
    }

    #[test]
    fn move_absolute() {
        let action = KSAction::Move(Range::Motion(Absolute::First.into()));
        assert_parse!("0", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Absolute::Last.into()));
        assert_parse!("$", KeySeq { num: 1, action });
    }

    #[test]
    fn move_cclass() {
        let action = KSAction::Move(Range::Motion(CClass::ForwardWord.into()));
        assert_parse!("w", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(CClass::ForwardBlank.into()));
        assert_parse!("W", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(CClass::BackwardWord.into()));
        assert_parse!("b", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(CClass::BackwardBlank.into()));
        assert_parse!("B", KeySeq { num: 1, action });
    }

    #[test]
    fn move_repeat_searches() {
        let action = KSAction::Move(Range::RepeatSearch);
        assert_parse!(";", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::RepeatRevSearch);
        assert_parse!(",", KeySeq { num: 1, action });
    }

    #[test]
    fn history() {
        let action = KSAction::History(History::Past);
        assert_parse!("k", KeySeq { num: 1, action });
        let action = KSAction::History(History::Recent);
        assert_parse!("j", KeySeq { num: 1, action });
        let action = KSAction::History(History::Last);
        assert_parse!("g", KeySeq { num: 1, action });
        let action = KSAction::History(History::Current);
        assert_parse!("G", KeySeq { num: 1, action });
    }

    #[test]
    fn transition() {
        let action = KSAction::Transition(Transition::Insert);
        assert_parse!("i", KeySeq { num: 1, action });
        let action = KSAction::Transition(Transition::Append);
        assert_parse!("a", KeySeq { num: 1, action });
        let action = KSAction::Transition(Transition::HardInsert);
        assert_parse!("I", KeySeq { num: 1, action });
        let action = KSAction::Transition(Transition::HardAppend);
        assert_parse!("A", KeySeq { num: 1, action });
    }

    #[test]
    fn rel_cut() {
        let action = KSAction::Delete(Range::Motion(Relative::Right.into()));
        assert_parse!("x", KeySeq { num: 1, action });
        let action = KSAction::Delete(Range::Motion(Relative::Left.into()));
        assert_parse!("X", KeySeq { num: 1, action });
    }

    #[test]
    fn rest_of_line() {
        let action = KSAction::Delete(Range::Motion(Absolute::Last.into()));
        assert_parse!("D", KeySeq { num: 1, action });

        let action = KSAction::Change(Range::Motion(Absolute::Last.into()));
        assert_parse!("C", KeySeq { num: 1, action });
    }

    #[test]
    fn replace_char() {
        let action = KSAction::Replace('x');
        assert_parse!("rx", KeySeq { num: 1, action });

        refute_parse!("r", KeySeqErr::Insufficient);
    }

    #[test]
    fn delete() {
        let action = KSAction::Delete(Range::Whole);
        assert_parse!("dd", KeySeq { num: 1, action });

        let action = KSAction::Delete(Range::Motion(CClass::ForwardWord.into()));
        assert_parse!("dw", KeySeq { num: 1, action });

        let action = KSAction::Delete(Range::Motion(Absolute::Last.into()));
        assert_parse!("d$", KeySeq { num: 1, action });

        let action = KSAction::Change(Range::Motion(Search::BackwardFind(';').into()));
        assert_parse!("cF;", KeySeq { num: 1, action });

        refute_parse!("dc", KeySeqErr::Failed);
        refute_parse!("c", KeySeqErr::Insufficient);
    }

    #[test]
    fn change() {
        let action = KSAction::Change(Range::Whole);
        assert_parse!("cc", KeySeq { num: 1, action });

        let action = KSAction::Change(Range::Motion(CClass::ForwardWord.into()));
        assert_parse!("cw", KeySeq { num: 1, action });

        let action = KSAction::Change(Range::Motion(Absolute::Last.into()));
        assert_parse!("c$", KeySeq { num: 1, action });

        let action = KSAction::Change(Range::Motion(Search::ForwardFind(';').into()));
        assert_parse!("cf;", KeySeq { num: 1, action });

        refute_parse!("cd", KeySeqErr::Failed);
        refute_parse!("c", KeySeqErr::Insufficient);
    }

    #[test]
    fn move_find() {
        let action = KSAction::Move(Range::Motion(Search::ForwardFind('/').into()));
        assert_parse!("f/", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Search::ForwardFind('\'').into()));
        assert_parse!("f'", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Search::BackwardFind('/').into()));
        assert_parse!("F/", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Search::ForwardTo('/').into()));
        assert_parse!("t/", KeySeq { num: 1, action });

        let action = KSAction::Move(Range::Motion(Search::BackwardTo('/').into()));
        assert_parse!("T/", KeySeq { num: 1, action });
    }
}
