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

mod insert {
    use super::*;

    #[test]
    fn default() {
        assert_parse!("i", Command::Insert(Offset::Nil(Point::Current), None))
    }

    #[test]
    fn inline() {
        assert_parse!(
            "i 'foobar\\nbaz'",
            Command::Insert(
                Offset::Nil(Point::Current),
                Some(vec!["foobar".to_string(), "baz".to_string()])
            )
        );
    }

    #[test]
    fn escaped_quote() {
        assert_parse!(
            "i 'foo\\'bar\\nbaz'",
            Command::Insert(
                Offset::Nil(Point::Current),
                Some(vec!["foo'bar".to_string(), "baz".to_string()])
            )
        );
    }
}

mod append {
    use super::*;

    #[test]
    fn default() {
        assert_parse!("a", Command::Append(Offset::Nil(Point::Current), None))
    }

    #[test]
    fn inline() {
        assert_parse!(
            "a 'foobar\\nbaz'",
            Command::Append(
                Offset::Nil(Point::Current),
                Some(vec!["foobar".to_string(), "baz".to_string()])
            )
        );
    }

    #[test]
    fn inline_empty() {
        assert_parse!(
            "a ''",
            Command::Append(Offset::Nil(Point::Current), Some(vec!["".to_string()]))
        );
    }

    #[test]
    fn inline_empty_dquote() {
        assert_parse!(
            "a \"\"",
            Command::Append(Offset::Nil(Point::Current), Some(vec!["".to_string()]))
        );
    }

    #[test]
    fn escaped_quote() {
        assert_parse!(
            "a 'foo\\'bar\\nbaz'",
            Command::Append(
                Offset::Nil(Point::Current),
                Some(vec!["foo'bar".to_string(), "baz".to_string()])
            )
        );
    }
}

mod change {
    use super::*;

    #[test]
    fn default() {
        assert_parse!(
            "c",
            Command::Change(Address::Line(Offset::Nil(Point::Current)), None)
        )
    }

    #[test]
    fn inline() {
        assert_parse!(
            "c 'foobar\\nbaz'",
            Command::Change(
                Address::Line(Offset::Nil(Point::Current)),
                Some(vec!["foobar".to_string(), "baz".to_string()])
            )
        );
    }

    #[test]
    fn escaped_quote() {
        assert_parse!(
            "c 'foo\\'bar\\nbaz'",
            Command::Change(
                Address::Line(Offset::Nil(Point::Current)),
                Some(vec!["foo'bar".to_string(), "baz".to_string()])
            )
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

mod subst {
    use super::*;
    use crate::ed::cmd::SubstFlags;
    use crate::ed::re::Pat;

    #[test]
    fn default() {
        assert_parse!(
            "s/./-/",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(".")),
                Some(Pat::from_str("-").unwrap()),
                Some(SubstFlags {
                    occurances: 1,
                    print: false
                })
            )
        );
    }

    #[test]
    fn delete_space() {
        assert_parse!(
            "s/ //",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(" ")),
                Some(Pat::from_str("").unwrap()),
                Some(SubstFlags {
                    occurances: 1,
                    print: false
                })
            )
        );
    }

    #[test]
    fn invalid_flags() {
        refute_parse!("s/ //10g");
    }

    #[test]
    fn flags() {
        assert_parse!(
            "s/ //10",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(" ")),
                Some(Pat::from_str("").unwrap()),
                Some(SubstFlags {
                    occurances: 10,
                    print: false
                })
            )
        );

        assert_parse!(
            "s/ //10p",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(" ")),
                Some(Pat::from_str("").unwrap()),
                Some(SubstFlags {
                    occurances: 10,
                    print: true
                })
            )
        );

        assert_parse!(
            "s/ //10p",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(" ")),
                Some(Pat::from_str("").unwrap()),
                Some(SubstFlags {
                    occurances: 10,
                    print: true
                })
            )
        );

        assert_parse!(
            "s/ //gp",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(" ")),
                Some(Pat::from_str("").unwrap()),
                Some(SubstFlags {
                    occurances: 0,
                    print: true
                })
            )
        );
        assert_parse!(
            "s/ //p",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(" ")),
                Some(Pat::from_str("").unwrap()),
                Some(SubstFlags {
                    occurances: 1,
                    print: true
                })
            )
        );
    }

    #[test]
    fn test_prev_pat() {
        assert_parse!(
            "s/.*/%/",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                Some(re!(".*")),
                Some(Pat::Replay),
                Some(SubstFlags {
                    occurances: 1,
                    print: false
                })
            )
        );
    }

    #[test]
    fn test_prev_regex() {
        assert_parse!(
            "s//foobar/",
            Command::Subst(
                Address::Line(Offset::Nil(Point::Current)),
                None,
                Some(Pat::from_str("foobar").unwrap()),
                Some(SubstFlags {
                    occurances: 1,
                    print: false
                })
            )
        );
    }
}

mod yank {
    use super::*;

    #[test]
    fn default() {
        assert_parse!(
            "y",
            Command::Yank(Address::Line(Offset::Nil(Point::Current)),)
        );
    }

    #[test]
    fn with_address() {
        assert_parse!(
            "1,$y",
            Command::Yank(Address::Range {
                start: Offset::Nil(Point::Abs(1)),
                end: Offset::Nil(Point::Last),
            })
        );
    }
}

mod paste {
    use super::*;

    #[test]
    fn default() {
        assert_parse!("x", Command::Paste(Offset::Nil(Point::Current)));
    }

    #[test]
    fn with_line() {
        assert_parse!("-5x", Command::Paste(Offset::Relb(Point::Current, 5)));
    }

    #[test]
    fn no_address() {
        refute_parse!("-5,$x");
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
                SysPoint::Filename,
                false
            ))
        );
    }

    #[test]
    fn default_and_quit() {
        assert_eq!(
            Command::from_str("wq"),
            Ok(Command::Write(
                Address::Range {
                    start: Offset::Nil(Point::Abs(1)),
                    end: Offset::Nil(Point::Last),
                },
                SysPoint::Filename,
                true
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
                SysPoint::File("foobar.txt".to_string()),
                false
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
                SysPoint::Command(Cmd::System("rustfmt %".to_string())),
                false
            ))
        );
    }

    #[test]
    fn specified_command_and_quit() {
        assert_eq!(
            Command::from_str("wq !rustfmt %"),
            Ok(Command::Write(
                Address::Range {
                    start: Offset::Nil(Point::Abs(1)),
                    end: Offset::Nil(Point::Last),
                },
                SysPoint::Command(Cmd::System("rustfmt %".to_string())),
                true
            ))
        );
    }
}

mod quit {
    use super::*;

    #[test]
    fn default() {
        assert_parse!("q", Command::Quit);
    }
}
