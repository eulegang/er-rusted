use crate::ed::parse::Parsable;
use crate::ed::prelude::*;

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

mod invoke;
mod parse;

mod util {
    use crate::ed::syspoint::Cmd;

    #[test]
    fn replace_default() {
        assert_eq!(
            Cmd::System("rustfmt %".to_string())
                .replace_filename(Some("src/lib.rs"), None)
                .as_deref(),
            Some("rustfmt src/lib.rs")
        );

        assert_eq!(
            Cmd::System("rustfmt \\%".to_string())
                .replace_filename(Some("src/lib.rs"), None)
                .as_deref(),
            Some("rustfmt %")
        );

        assert_eq!(
            Cmd::System("rustfmt \\\\%".to_string())
                .replace_filename(Some("src/lib.rs"), None)
                .as_deref(),
            Some("rustfmt \\src/lib.rs")
        );

        assert_eq!(
            Cmd::System("rustfmt \\\\\\%".to_string())
                .replace_filename(Some("src/lib.rs"), None)
                .as_deref(),
            Some("rustfmt \\%")
        );
    }

    #[test]
    fn replace_repeat() {
        assert_eq!(
            Cmd::Repeat
                .replace_filename(Some("src/lib.rs"), None)
                .as_deref(),
            None
        );

        assert_eq!(
            Cmd::Repeat
                .replace_filename(Some("src/lib.rs"), Some("rustc %"))
                .as_deref(),
            Some("rustc src/lib.rs")
        );
    }
}
