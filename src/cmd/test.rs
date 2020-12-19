use crate::{
    addr::{Address, Offset, Point},
    cmd::{Command, Sink},
    Parsable,
};

use std::str::FromStr;

use nom::combinator::all_consuming;

macro_rules! assert_parse {
    ($input: literal, $expected: expr) => {
        assert_eq!(
            all_consuming(Command::parse)($input).map(|cmd| cmd.1),
            Ok($expected)
        )
    };
}

macro_rules! refute_parse {
    ($input: literal) => {{
        let parsed = all_consuming(Command::parse)($input);
        assert!(
            parsed.is_err(),
            "expected parse error but parsed {:?}",
            parsed
        )
    }};
}

mod parse {
    use super::*;

    #[test]
    fn bogus() {
        refute_parse!("foobar");
    }

    mod print {
        use super::*;

        #[test]
        fn default() {
            assert_parse!(
                "p",
                Command::Print(Address::Line(Offset::Nil(Point::Current)))
            );
        }

        #[test]
        fn address() {
            assert_parse!(
                ".,+10p",
                Command::Print(Address::Range {
                    start: Offset::Nil(Point::Current),
                    end: Offset::Relf(Point::Current, 10),
                })
            );
        }
    }

    mod delete {
        use super::*;

        #[test]
        fn default() {
            assert_parse!(
                "d",
                Command::Delete(Address::Line(Offset::Nil(Point::Current)))
            );
        }

        #[test]
        fn address() {
            assert_parse!(
                "1,$d",
                Command::Delete(Address::Range {
                    start: Offset::Nil(Point::Abs(1)),
                    end: Offset::Nil(Point::Last),
                })
            );
        }
    }

    mod nop {
        use super::*;

        #[test]
        fn default() {
            assert_parse!("+5", Command::Nop(Offset::Relf(Point::Current, 5)));
        }

        #[test]
        fn not_address() {
            refute_parse!("-5,+3")
        }
    }

    mod marks {
        use super::*;

        #[test]
        fn default() {
            assert_parse!("ka", Command::Mark(Offset::Nil(Point::Current), 'a'));
        }

        #[test]
        fn with_line() {
            assert_parse!("5ka", Command::Mark(Offset::Nil(Point::Abs(5)), 'a'));
        }

        #[test]
        fn no_address() {
            refute_parse!("1,5ka");
        }
    }

    mod join {
        use super::*;

        #[test]
        fn default() {
            assert_parse!(
                "j",
                Command::Join(Address::Range {
                    start: Offset::Nil(Point::Current),
                    end: Offset::Relf(Point::Current, 1)
                })
            );
        }
    }

    mod r#move {
        use super::*;

        #[test]
        fn default() {
            assert_parse!(
                "m1",
                Command::Move(
                    Address::Line(Offset::Nil(Point::Current)),
                    Offset::Nil(Point::Abs(1))
                )
            );
        }
    }

    mod transfer {
        use super::*;

        #[test]
        fn default() {
            assert_parse!(
                "t1",
                Command::Transfer(
                    Address::Line(Offset::Nil(Point::Current)),
                    Offset::Nil(Point::Abs(1))
                )
            );
        }
    }

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