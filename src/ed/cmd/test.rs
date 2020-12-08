use super::*;
use crate::Parsable;
use nom::combinator::all_consuming;

use Command::*;

#[test]
fn cmd_parse_append() {
    let p = all_consuming(Command::parse)(".,+10p").unwrap().1;
    assert_eq!(
        p,
        Print(Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Relf(Point::Current, 10),
        })
    );
}
