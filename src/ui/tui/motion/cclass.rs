use super::Motion;

pub enum CClass {
    ForwardWord,
    BackwardWord,
    ForwardBlank,
    BackwardBlank,
}

impl Motion for CClass {
    fn move_cursor(&self, buffer: &str, cursor: usize) -> Option<usize> {
        match self {
            CClass::ForwardWord => fword(buffer, cursor),
            CClass::BackwardWord => bword(buffer, cursor),
            CClass::ForwardBlank => fblank(buffer, cursor),
            CClass::BackwardBlank => bblank(buffer, cursor),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum CharClass {
    Core,
    NonBlank,
    Blank,
}

impl From<char> for CharClass {
    fn from(ch: char) -> CharClass {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => CharClass::Core,
            ' ' | '\n' | '\r' | '\t' => CharClass::Blank,
            _ => CharClass::NonBlank,
        }
    }
}

fn fword(buf: &str, pos: usize) -> Option<usize> {
    let mut it = buf.chars().skip(pos);

    let ch_type = match it.next() {
        Some(ch) => CharClass::from(ch),
        None => return None,
    };

    let mut result = pos;

    while let Some(next) = it.next() {
        result += 1;
        let cur_type = CharClass::from(next);

        if cur_type == CharClass::Blank || cur_type != ch_type {
            break;
        }
    }

    Some(result)
}

fn bword(buf: &str, pos: usize) -> Option<usize> {
    let skips = match buf.len().checked_sub(pos + 1) {
        Some(s) => s,
        None => return Some(buf.len() - 1),
    };

    let mut it = buf.chars();
    for _ in 0..skips {
        it.next_back();
    }

    match it.next_back() {
        Some(_) => (),
        None => return Some(pos),
    };

    let mut result = pos;

    let stable_type = match it.next_back() {
        Some(ch) => CharClass::from(ch),
        None => return Some(pos),
    };
    result -= 1;

    while let Some(next) = it.next_back() {
        let cur_type = CharClass::from(next);

        if (cur_type == CharClass::Blank) || (cur_type != stable_type) {
            return Some(result);
        }

        result -= 1;
    }

    Some(0)
}

fn fblank(buf: &str, pos: usize) -> Option<usize> {
    let mut it = buf.chars().skip(pos);

    let ch_type = match it.next() {
        Some(ch) => CharClass::from(ch),
        None => return Some(pos),
    };

    let mut result = pos;

    if ch_type != CharClass::Blank {
        while let Some(next) = it.next() {
            let cur_type = CharClass::from(next);
            result += 1;

            if cur_type == CharClass::Blank {
                break;
            }
        }
    }

    while let Some(next) = it.next() {
        let cur_type = CharClass::from(next);
        result += 1;

        if cur_type != CharClass::Blank {
            break;
        }
    }

    Some(result)
}

fn bblank(buf: &str, pos: usize) -> Option<usize> {
    let skips = match buf.len().checked_sub(pos + 1) {
        Some(s) => s,
        None => return Some(buf.len() - 1),
    };

    let mut it = buf.chars();
    for _ in 0..skips {
        it.next_back();
    }

    let mut result = pos;

    let _ = match it.next_back() {
        Some(first) => CharClass::from(first),
        None => return Some(pos),
    };

    let next_type = match it.next_back() {
        Some(second) => CharClass::from(second),
        None => return Some(pos),
    };

    result -= 1;

    if next_type != CharClass::Blank {
        while let Some(ch) = it.next_back() {
            let cur_type = CharClass::from(ch);

            if cur_type == CharClass::Blank {
                break;
            }

            result -= 1
        }
    } else {
        while let Some(ch) = it.next_back() {
            let cur_type = CharClass::from(ch);
            result -= 1;

            if cur_type != CharClass::Blank {
                break;
            }
        }

        while let Some(ch) = it.next_back() {
            let cur_type = CharClass::from(ch);

            if cur_type == CharClass::Blank {
                break;
            }

            result -= 1
        }
    }

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fword() {
        let buf = "%s/user/author/";
        let motion = CClass::ForwardWord;

        assert_eq!(Some(1), motion.move_cursor(buf, 0), "from %");
        assert_eq!(Some(2), motion.move_cursor(buf, 1), "from s subst");
        assert_eq!(Some(3), motion.move_cursor(buf, 2), "from first /");
        assert_eq!(Some(7), motion.move_cursor(buf, 3), "from user");
        assert_eq!(Some(8), motion.move_cursor(buf, 7), "from second /");
        assert_eq!(Some(14), motion.move_cursor(buf, 8), "from author");
        assert_eq!(
            Some(14),
            motion.move_cursor(buf, 14),
            "from last / (end of string should stay put)"
        );
    }

    #[test]
    fn test_bword() {
        let buf = "%s/user/author/";
        let motion = CClass::BackwardWord;

        assert_eq!(Some(14), motion.move_cursor(buf, 20), "overflown");
        assert_eq!(Some(8), motion.move_cursor(buf, 14), "from last /");
        assert_eq!(Some(7), motion.move_cursor(buf, 8), "from author");
        assert_eq!(Some(3), motion.move_cursor(buf, 7), "from second /");
        assert_eq!(Some(2), motion.move_cursor(buf, 3), "from user");
        assert_eq!(Some(1), motion.move_cursor(buf, 2), "from first /");
        assert_eq!(Some(0), motion.move_cursor(buf, 1), "from s subst");
        assert_eq!(Some(0), motion.move_cursor(buf, 0), "0 should loop");
    }

    #[test]
    fn test_fblank() {
        let buf = "g/#TODO: /d";
        let motion = CClass::ForwardBlank;

        assert_eq!(Some(9), motion.move_cursor(buf, 0), "from g");
        assert_eq!(Some(10), motion.move_cursor(buf, 9), "from /");
        assert_eq!(Some(10), motion.move_cursor(buf, 10), "should loop end");
    }

    #[test]
    fn test_bblank() {
        let buf = "g/#TODO: /d";
        let motion = CClass::BackwardBlank;

        assert_eq!(Some(9), motion.move_cursor(buf, 10), "from d");
        assert_eq!(Some(0), motion.move_cursor(buf, 9), "from /");
        assert_eq!(Some(0), motion.move_cursor(buf, 0), "begin should loop");
    }
}
