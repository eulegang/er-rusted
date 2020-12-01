use super::*;
use std::str::FromStr;

#[test]
fn re_eq() {
    let plus = Re::from_str("a+").unwrap();
    let mux = Re::from_str("aa*").unwrap();
    let plus_prime = Re::from_str("a+").unwrap();

    assert!(plus != mux);
    assert_eq!(plus, plus_prime);
}

#[test]
fn operates_like_regex() {
    let plus = Re::from_str("a+").unwrap();

    assert!(plus.is_match("ahhhh"));
}
