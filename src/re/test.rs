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

mod pat {
    use super::*;

    #[test]
    fn parse_replay() {
        assert_eq!(Pat::from_str("%"), Ok(Pat::Replay));
    }

    #[test]
    fn parse_lit() {
        assert_eq!(
            Pat::from_str("hello"),
            Ok(Pat::Expansion(vec![Expansion::Lit("hello".to_string())]))
        );
    }

    #[test]
    fn parse_whole() {
        assert_eq!(
            Pat::from_str("&"),
            Ok(Pat::Expansion(vec![Expansion::Whole]))
        );
    }

    #[test]
    fn parse_pos() {
        assert_eq!(
            Pat::from_str("\\6"),
            Ok(Pat::Expansion(vec![Expansion::Pos(6)]))
        );
    }

    #[test]
    fn parse_compose() {
        assert_eq!(
            Pat::from_str(", &"),
            Ok(Pat::Expansion(vec![
                Expansion::Lit(", ".to_string()),
                Expansion::Whole
            ]))
        );
    }

    #[test]
    fn parse_escape() {
        assert_eq!(
            Pat::from_str("\\%"),
            Ok(Pat::Expansion(vec![Expansion::Lit("%".to_string())])),
            "%"
        );

        assert_eq!(
            Pat::from_str("\\&"),
            Ok(Pat::Expansion(vec![Expansion::Lit("&".to_string())])),
            "&"
        );
    }
}
