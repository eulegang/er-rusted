use crate::{
    addr::{Address, Offset, Point},
    cmd::Command,
    Parsable,
};

use nom::combinator::all_consuming;

#[test]
fn cmd_parse_append() {
    let p = all_consuming(Command::parse)(".,+10p").unwrap().1;
    assert_eq!(
        p,
        Command::Print(Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Relf(Point::Current, 10),
        })
    );
}
