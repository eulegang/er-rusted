use super::*;

macro_rules! assert_content {
    ($buffer: expr, $content: literal) => {{
        let mut bytes = Vec::new();
        $buffer.write(&mut bytes).unwrap();

        assert_eq!(&String::from_utf8(bytes).unwrap(), $content);
    }};
}

#[test]
fn test_chomp() {
    let mut s = "hello\n".to_string();
    chomp(&mut s);
    assert_eq!(&s, "hello");

    let mut s = "hello\n\n".to_string();
    chomp(&mut s);
    assert_eq!(&s, "hello\n", "only chomp one newline");

    let mut s = "hello\r\n".to_string();
    chomp(&mut s);
    assert_eq!(&s, "hello", "remove cr if cr is before newline");

    let mut s = "".to_string();
    chomp(&mut s);
    assert_eq!(&s, "", "empty string case");

    let mut s = "\r".to_string();
    chomp(&mut s);
    assert_eq!(&s, "\r", "only chomps cr if ends with newline");

    let mut s = "\ra".to_string();
    chomp(&mut s);
    assert_eq!(&s, "\ra", "only chomps cr if ends with newline");
}

const SAMPLE_TEXT: &str = "hello
world
code
here
";

mod insert {
    use super::*;
    #[test]
    fn first() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.insert(1, vec!["foo".to_string(), "bar".to_string()]));
        assert_content!(buffer, "foo\nbar\nhello\nworld\ncode\nhere\n");
        assert_eq!(buffer.cur, 2);
    }

    #[test]
    fn null() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.insert(0, vec!["foo".to_string(), "bar".to_string()]));
        assert_content!(buffer, "foo\nbar\nhello\nworld\ncode\nhere\n");
        assert_eq!(buffer.cur, 2);
    }

    #[test]
    fn last() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.insert(4, vec!["foo".to_string(), "bar".to_string()]));
        assert_content!(buffer, "hello\nworld\ncode\nfoo\nbar\nhere\n");
        assert_eq!(buffer.cur, 5);
    }

    #[test]
    fn bogus() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(!buffer.insert(15, vec!["foo".to_string(), "bar".to_string()]));

        assert!(
            !buffer.insert(5, vec!["foo".to_string(), "bar".to_string()]),
            "buffer: {:?}",
            buffer
        );
    }
}

mod append {
    use super::*;

    #[test]
    fn first() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.append(1, vec!["foo".to_string(), "bar".to_string()]));

        let mut bytes = Vec::new();
        buffer.write(&mut bytes).unwrap();

        assert_eq!(
            &String::from_utf8(bytes).unwrap(),
            "hello\nfoo\nbar\nworld\ncode\nhere\n"
        );

        assert_eq!(buffer.cur, 3);
    }

    #[test]
    fn null() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.append(0, vec!["foo".to_string(), "bar".to_string()]));

        let mut bytes = Vec::new();
        buffer.write(&mut bytes).unwrap();

        assert_eq!(
            &String::from_utf8(bytes).unwrap(),
            "foo\nbar\nhello\nworld\ncode\nhere\n"
        );

        assert_eq!(buffer.cur, 2);
    }

    #[test]
    fn last() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.append(4, vec!["foo".to_string(), "bar".to_string()]));

        let mut bytes = Vec::new();
        buffer.write(&mut bytes).unwrap();
        assert_eq!(
            &String::from_utf8(bytes).unwrap(),
            "hello\nworld\ncode\nhere\nfoo\nbar\n",
        );

        assert_eq!(buffer.cur, 6);
    }

    #[test]
    fn bogus() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(!buffer.append(15, vec!["foo".to_string(), "bar".to_string()]));
        assert!(!buffer.append(5, vec!["foo".to_string(), "bar".to_string()]));
    }
}

mod remove {
    use super::*;

    #[test]
    fn basic() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.remove(1, 3).is_some());

        let mut bytes = Vec::new();
        buffer.write(&mut bytes).unwrap();

        assert_eq!(&String::from_utf8(bytes).unwrap(), "here\n");
    }

    #[test]
    fn underflow() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.remove(0, 3).is_none());
    }

    #[test]
    fn overflow() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.remove(2, 20).is_none());
    }

    #[test]
    fn whole() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.remove(1, 4).is_some());

        let mut bytes = Vec::new();
        buffer.write(&mut bytes).unwrap();

        assert_eq!(&String::from_utf8(bytes).unwrap(), "");
    }

    #[test]
    fn cur() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

        assert!(buffer.remove(3, 3).is_some());

        let mut bytes = Vec::new();
        buffer.write(&mut bytes).unwrap();

        assert_eq!(&String::from_utf8(bytes).unwrap(), "hello\nworld\nhere\n");
        assert_eq!(buffer.cur, 3);
    }
}

mod range {
    use super::*;

    #[test]
    fn valid() {
        let buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();
        assert_eq!(
            buffer.range(1, 2),
            Some(vec!["hello".to_string(), "world".to_string()])
        );
    }

    #[test]
    fn underflow() {
        let buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();
        assert_eq!(
            buffer.range(0, 2),
            Some(vec!["hello".to_string(), "world".to_string()])
        );
    }

    #[test]
    fn overflow() {
        let buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();
        assert_eq!(buffer.range(1, 15), None);
    }
}

mod replace_line {
    use super::*;

    #[test]
    fn valid() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();
        assert_eq!(
            buffer.replace_line(2, "there".to_string()),
            Some("world".to_string()),
        );

        assert_content!(buffer, "hello\nthere\ncode\nhere\n");
    }

    #[test]
    fn bogus() {
        let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();
        assert_eq!(buffer.replace_line(15, "there".to_string()), None);

        assert_content!(buffer, "hello\nworld\ncode\nhere\n");
    }
}

#[test]
fn test_change() {
    let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

    buffer.change(1, 3, vec!["foo".to_string(), "bar".to_string()]);

    let mut bytes = Vec::new();
    buffer.write(&mut bytes).unwrap();

    assert_eq!(&String::from_utf8(bytes).unwrap(), "foo\nbar\nhere\n");
}

#[test]
fn test_lines() {
    let buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

    assert_eq!(buffer.len(), 4);
}
