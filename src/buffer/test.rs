use super::*;

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

#[test]
fn test_remove() {
    let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

    buffer.remove(1, 3);

    let mut bytes = Vec::new();
    buffer.write(&mut bytes).unwrap();

    assert_eq!(&String::from_utf8(bytes).unwrap(), "here\n");
}

#[test]
fn test_insert() {
    let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

    buffer.insert(1, vec!["foo".to_string(), "bar".to_string()]);

    let mut bytes = Vec::new();
    buffer.write(&mut bytes).unwrap();

    assert_eq!(
        &String::from_utf8(bytes).unwrap(),
        "foo\nbar\nhello\nworld\ncode\nhere\n"
    );
}

#[test]
fn test_append() {
    let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

    buffer.append(1, vec!["foo".to_string(), "bar".to_string()]);

    let mut bytes = Vec::new();
    buffer.write(&mut bytes).unwrap();

    assert_eq!(
        &String::from_utf8(bytes).unwrap(),
        "hello\nfoo\nbar\nworld\ncode\nhere\n"
    );
}

#[test]
fn test_change() {
    let mut buffer = Buffer::read(SAMPLE_TEXT.as_bytes()).unwrap();

    buffer.change(1, 3, vec!["foo".to_string(), "bar".to_string()]);

    let mut bytes = Vec::new();
    buffer.write(&mut bytes).unwrap();

    assert_eq!(&String::from_utf8(bytes).unwrap(), "foo\nbar\nhere\n");
}
