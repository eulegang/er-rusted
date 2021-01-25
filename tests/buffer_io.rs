use er_rusted::Buffer;

#[test]
fn test_read() {
    let content: &str = "hello\nworld";

    let buf = Buffer::read(content.as_bytes()).unwrap();

    assert_eq!(buf.cursor(), 1, "buffer starts before the first line");
    assert_eq!(buf.len(), 2, "buffer starts before the first line");

    assert_eq!(buf.line(1), Some("hello"), "first line");
    assert_eq!(buf.line(2), Some("world"), "second line");

    let content: &str = "hello\nworld\n";

    let buf = Buffer::read(content.as_bytes()).unwrap();

    assert_eq!(buf.cursor(), 1, "buffer starts before the first line");
    assert_eq!(buf.len(), 2, "buffer starts before the first line");

    assert_eq!(buf.line(1), Some("hello"), "first line");
    assert_eq!(buf.line(2), Some("world"), "second line");

    let content: &str = "hello\r\nworld\r\n";

    let buf = Buffer::read(content.as_bytes()).unwrap();

    assert_eq!(buf.cursor(), 1, "buffer starts before the first line");
    assert_eq!(buf.len(), 2, "buffer starts before the first line");

    assert_eq!(buf.line(1), Some("hello"), "first line");
    assert_eq!(buf.line(2), Some("world"), "second line");
}

#[test]
fn test_write() {
    const CONTENT: &str = "hello\nworld";
    let mut buf = Buffer::read(CONTENT.as_bytes()).unwrap();

    let mut bytes = Vec::new();
    let written = buf.write(&mut bytes).expect("write to work");
    assert_eq!(written, 12);

    let out = String::from_utf8(bytes).expect("content should be ascii");
    assert_eq!(&out, "hello\nworld\n");
}

#[test]
fn empty_append() {
    const CONTENT: &str = "";

    let mut buf = Buffer::read(CONTENT.as_bytes()).unwrap();

    assert!(buf.append(0, vec!["hello".to_string(), "there".to_string()]));
    let mut bytes = Vec::new();
    buf.write(&mut bytes).expect("write to work");
    let out = String::from_utf8(bytes).expect("content should be ascii");
    assert_eq!(&out, "hello\nthere\n");
}

#[test]
fn empty_insert() {
    const CONTENT: &str = "";

    let mut buf = Buffer::read(CONTENT.as_bytes()).unwrap();

    assert!(buf.insert(0, vec!["hello".to_string(), "there".to_string()]));
    let mut bytes = Vec::new();
    buf.write(&mut bytes).expect("write to work");
    let out = String::from_utf8(bytes).expect("content should be ascii");
    assert_eq!(&out, "hello\nthere\n");
}
