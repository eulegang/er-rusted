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
