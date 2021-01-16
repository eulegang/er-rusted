use super::*;
use crate::interp::{scratch::StdoutScratchPad, Interpreter};

macro_rules! assert_content {
    ($buffer: expr, $content: literal) => {{
        let mut bytes = Vec::new();
        $buffer.write(&mut bytes).unwrap();

        assert_eq!(&String::from_utf8(bytes).unwrap(), $content);
    }};
}

mod g {
    use super::*;

    const CONTENT: &str = r"
foobar
bar
barfoo
";

    #[test]
    fn delete() {
        let cmd = Command::from_str("g/foo/d").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "bar\n");
    }

    #[test]
    fn transfer() {
        let cmd = Command::from_str("g/foo/t$").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "foobar\nbar\nbarfoo\nfoobar\nbarfoo\n");
    }

    #[test]
    fn move_last() {
        let cmd = Command::from_str("g/foo/m$").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));
        assert_content!(interp.buffer, "bar\nfoobar\nbarfoo\n");
    }

    #[test]
    fn move_first() {
        let cmd = Command::from_str("g/foo/m0").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "barfoo\nfoobar\nbar\n");
    }

    const CODE: &str = r"
fn foo() -> usize {
    0
}

fn bar() -> usize {
    0
}";

    #[test]
    fn rearrange() {
        let cmd = Command::from_str("g/bar/.,/\\}/m0").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CODE.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(
            interp.buffer,
            "fn bar() -> usize {\n    0\n}\nfn foo() -> usize {\n    0\n}\n\n"
        );
    }
}

mod v {
    use super::*;

    const CONTENT: &str = r"
foobar
bar
barfoo
";

    #[test]
    fn delete() {
        let cmd = Command::from_str("v/foo/d").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "foobar\nbarfoo\n");
    }
}

mod m {
    use super::*;

    const CONTENT: &str = r"
foobar
bar
needle
barfoo
";

    #[test]
    fn first() {
        let cmd = Command::from_str("/needle/m0").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "needle\nfoobar\nbar\nbarfoo\n");
    }

    #[test]
    fn last() {
        let cmd = Command::from_str("/needle/m$").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "foobar\nbar\nbarfoo\nneedle\n");
    }
}

mod j {
    use super::*;

    const CONTENT: &str = r"
foobar
bar
needle
barfoo
";

    #[test]
    fn basic() {
        let cmd = Command::from_str("1,3j").expect("should parse");

        let mut interp =
            Interpreter::from_reader::<StdoutScratchPad, &[u8]>(CONTENT.trim().as_bytes())
                .expect("should read");

        assert_eq!(Ok(true), interp.exec(&cmd));

        assert_content!(interp.buffer, "foobar bar needle\nbarfoo\n");
    }
}
