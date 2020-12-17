use crate::{
    addr::{Address, Offset, Point},
    cmd::{Command, Sink},
    Parsable,
};

use std::str::FromStr;

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

mod parse {
    use super::*;

    mod write {
        use super::*;

        #[test]
        fn default() {
            assert_eq!(
                Command::from_str("w"),
                Ok(Command::Write(
                    Address::Range {
                        start: Offset::Nil(Point::Abs(1)),
                        end: Offset::Nil(Point::Last),
                    },
                    Sink::Filename
                ))
            );
        }

        #[test]
        fn specified_file() {
            assert_eq!(
                Command::from_str("w foobar.txt"),
                Ok(Command::Write(
                    Address::Range {
                        start: Offset::Nil(Point::Abs(1)),
                        end: Offset::Nil(Point::Last),
                    },
                    Sink::File("foobar.txt".to_string())
                ))
            );
        }

        #[test]
        fn specified_command() {
            assert_eq!(
                Command::from_str("w !rustfmt %"),
                Ok(Command::Write(
                    Address::Range {
                        start: Offset::Nil(Point::Abs(1)),
                        end: Offset::Nil(Point::Last),
                    },
                    Sink::Command("rustfmt %".to_string())
                ))
            );
        }
    }
}

mod util {
    use crate::cmd::replace_file;

    #[test]
    fn replace_default() {
        assert_eq!(
            &replace_file("rustfmt %", Some("src/lib.rs")),
            "rustfmt src/lib.rs"
        );

        assert_eq!(
            &replace_file("rustfmt \\%", Some("src/lib.rs")),
            "rustfmt %"
        );

        assert_eq!(
            &replace_file("rustfmt \\\\%", Some("src/lib.rs")),
            "rustfmt \\src/lib.rs"
        );

        assert_eq!(
            &replace_file("rustfmt \\\\\\%", Some("src/lib.rs")),
            "rustfmt \\%"
        );
    }
}
