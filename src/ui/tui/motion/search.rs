use super::Motion;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Search {
    ForwardFind(char),
    BackwardFind(char),
    ForwardTo(char),
    BackwardTo(char),
}

impl Motion for Search {
    fn move_cursor(&self, buffer: &str, cursor: usize) -> Option<usize> {
        match self {
            Search::ForwardFind(ch) => ffind(buffer, cursor, *ch),
            Search::BackwardFind(ch) => bfind(buffer, cursor, *ch),
            Search::ForwardTo(ch) => ffind(buffer, cursor, *ch).map(|i| i - 1),
            Search::BackwardTo(ch) => bfind(buffer, cursor, *ch).map(|i| i + 1),
        }
    }
}

impl Search {
    pub(crate) fn reverse(&self) -> Search {
        match self {
            Search::ForwardFind(ch) => Search::BackwardFind(*ch),
            Search::BackwardFind(ch) => Search::ForwardFind(*ch),
            Search::ForwardTo(ch) => Search::BackwardTo(*ch),
            Search::BackwardTo(ch) => Search::ForwardTo(*ch),
        }
    }
}

fn ffind(buf: &str, pos: usize, ch: char) -> Option<usize> {
    let mut it = buf.chars().skip(pos);

    if it.next().is_none() {
        return None;
    }

    let mut result = pos;
    while let Some(n) = it.next() {
        result += 1;

        if n == ch {
            return Some(result);
        }
    }

    None
}

fn bfind(buf: &str, pos: usize, ch: char) -> Option<usize> {
    let skips = match buf.len().checked_sub(pos + 1) {
        Some(s) => s,
        None => return None,
    };

    let mut it = buf.chars();
    for _ in 0..skips + 1 {
        if it.next_back().is_none() {
            return None;
        }
    }

    let mut result = pos - 1;
    while let Some(n) = it.next_back() {
        if n == ch {
            return Some(result);
        }

        if let Some(s) = result.checked_sub(1) {
            result = s;
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ffind() {
        let search = Search::ForwardFind('/');
        let buf = "%s/user/author/gp";

        assert_eq!(Some(2), search.move_cursor(buf, 0), "from begin");
        assert_eq!(Some(7), search.move_cursor(buf, 2), "from first /");
        assert_eq!(Some(14), search.move_cursor(buf, 7), "from second /");
        assert_eq!(None, search.move_cursor(buf, 14), "last / should be last");
    }

    #[test]
    fn test_fto() {
        let search = Search::ForwardTo('/');
        let buf = "%s/user/author/gp";

        assert_eq!(Some(1), search.move_cursor(buf, 0), "from begin");
        assert_eq!(Some(6), search.move_cursor(buf, 2), "from first /");
        assert_eq!(Some(6), search.move_cursor(buf, 6), "from first /");
        assert_eq!(Some(13), search.move_cursor(buf, 7), "from second /");
        assert_eq!(None, search.move_cursor(buf, 14), "last / should be last");
    }

    #[test]
    fn test_bfind() {
        let search = Search::BackwardFind('/');
        let buf = "%s/user/author/gp";

        assert_eq!(Some(14), search.move_cursor(buf, 16), "from end");
        assert_eq!(Some(7), search.move_cursor(buf, 14), "from last /");
        assert_eq!(Some(2), search.move_cursor(buf, 7), "from second /");
        assert_eq!(None, search.move_cursor(buf, 2), "first / should end");
    }

    #[test]
    fn test_bto() {
        let search = Search::BackwardTo('/');
        let buf = "%s/user/author/gp";

        assert_eq!(Some(15), search.move_cursor(buf, 16), "from end");
        assert_eq!(Some(8), search.move_cursor(buf, 14), "from last /");
        assert_eq!(Some(3), search.move_cursor(buf, 7), "from second /");
        assert_eq!(None, search.move_cursor(buf, 2), "first / should end");
    }
}
