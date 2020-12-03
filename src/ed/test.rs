use super::*;
use crate::re::Re;
use std::str::FromStr;

#[test]
fn test_default_addresses() {
    assert_eq!(
        Command::Append(String::new()).default_address(),
        Address::Line(Offset::Nil(Point::Current))
    );

    assert_eq!(
        Command::Insert(String::new()).default_address(),
        Address::Line(Offset::Nil(Point::Current))
    );

    assert_eq!(
        Command::Change(String::new()).default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Nil(Point::Current)
        }
    );

    assert_eq!(
        Command::Move(Default::default()).default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Nil(Point::Current)
        }
    );

    assert_eq!(
        Command::Delete.default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Nil(Point::Current)
        }
    );

    assert_eq!(
        Command::Join.default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Relf(Point::Current, 1)
        }
    );

    assert_eq!(
        Command::Mark('a').default_address(),
        Address::Line(Offset::Nil(Point::Current))
    );

    assert_eq!(
        Command::Move(Default::default()).default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Nil(Point::Current)
        }
    );

    assert_eq!(
        Command::Read("file".to_string()).default_address(),
        Address::Line(Offset::Nil(Point::Last))
    );

    assert_eq!(
        Command::Write("file".to_string()).default_address(),
        Address::Range {
            start: Offset::Nil(Point::Abs(1)),
            end: Offset::Nil(Point::Last)
        }
    );

    assert_eq!(
        Command::Substitute(Re::from_str("a+").unwrap(), "b".to_string()).default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Nil(Point::Current)
        }
    );

    assert_eq!(
        Command::Copy(Default::default()).default_address(),
        Address::Range {
            start: Offset::Nil(Point::Current),
            end: Offset::Nil(Point::Current)
        }
    );
}
