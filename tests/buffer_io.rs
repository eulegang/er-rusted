use er_rusted::Buffer;

#[test]
fn test_read() {
    const CONTENT: &str = "hello\nworld";

    let buf = Buffer::read(CONTENT.as_bytes()).unwrap();

    assert_eq!(buf.cursor(), 0, "buffer starts before the first line");
    assert_eq!(buf.lines(), 2, "buffer starts before the first line");

    assert_eq!(buf.line(1), Some("hello"), "first line");
    assert_eq!(buf.line(2), Some("world"), "second line");
}
