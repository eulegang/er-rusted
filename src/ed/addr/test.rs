use super::*;
use crate::Parsable;
use nom::combinator::all_consuming;
use std::str::FromStr;

#[test]
fn parse_point() {
    let p = all_consuming(Point::parse)(".").unwrap().1;
    assert_eq!(p, Point::Current);

    let p = all_consuming(Point::parse)("$").unwrap().1;
    assert_eq!(p, Point::Last);

    let p = all_consuming(Point::parse)("42").unwrap().1;
    assert_eq!(p, Point::Abs(42));

    let p = all_consuming(Point::parse)("'a").unwrap().1;
    assert_eq!(p, Point::Mark('a'));

    let p = all_consuming(Point::parse)("/^\\}/").unwrap().1;
    assert_eq!(p, Point::Ref(re!("^\\}")));

    let p = all_consuming(Point::parse)("?func *\\(\\??").unwrap().1;
    assert_eq!(p, Point::Reb(re!("func *\\(?")));
}

#[test]
fn parse_offset() {
    let p = all_consuming(Offset::parse)(".").unwrap().1;
    assert_eq!(p, Offset::Nil(Point::Current), ".");

    let p = all_consuming(Offset::parse)(".+1").unwrap().1;
    assert_eq!(p, Offset::Relf(Point::Current, 1), ".+1");

    let p = all_consuming(Offset::parse)(".+").unwrap().1;
    assert_eq!(p, Offset::Relf(Point::Current, 1), ".+");

    let p = all_consuming(Offset::parse)("$-1").unwrap().1;
    assert_eq!(p, Offset::Relb(Point::Last, 1), "$-1");

    let p = all_consuming(Offset::parse)("/^\\}/-1").unwrap().1;
    assert_eq!(p, Offset::Relb(Point::Ref(re!("^\\}")), 1), "$-1");

    all_consuming(Offset::parse)("").expect_err("Should not parse empty content");
}

#[test]
fn parse_address() {
    let p = all_consuming(Address::parse)(".").unwrap().1;
    assert_eq!(p, Address::Line(Offset::Nil(Point::Current)), ".");

    let p = all_consuming(Address::parse)(";+").unwrap().1;
    assert_eq!(
        p,
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Relf(Point::Current, 1)
        },
        ";+"
    );

    let p = all_consuming(Address::parse)(",").unwrap().1;
    assert_eq!(
        p,
        Address::Range {
            start: Offset::Nil(Point::Abs(1)),
            end: Offset::Nil(Point::Last)
        },
        ","
    );

    let p = all_consuming(Address::parse)("?\\{$?-,/^\\}/").unwrap().1;
    assert_eq!(
        p,
        Address::Range {
            start: Offset::Relb(Point::Reb(re!("\\{$")), 1),
            end: Offset::Nil(Point::Ref(re!("^\\}"))),
        },
        "?\\{{$?,/^\\}}/"
    );

    all_consuming(Address::parse)("").expect_err("Should not parse empty content");
}
