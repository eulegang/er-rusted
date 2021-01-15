use super::*;
use crate::ui::tui::action::*;
use crate::ui::tui::motion::*;
use crossterm::event::KeyEvent;

pub(crate) fn process_line_edit(key: KeyEvent, tui: &mut Tui) -> eyre::Result<()> {
    if key.code == KeyCode::Enter {
        Run.invoke(tui)?;
        return Ok(());
    }

    let (mag, buf) = parts(&tui.key_buffer);

    let cur = match key.code {
        KeyCode::Char(ch) => ch,
        _ => return Ok(()),
    };

    let key_len = tui.key_buffer.len();

    let change = buf.contains('c');
    let del = buf.contains('d') || change;

    if let Some(motion) = map_motion(cur, buf, tui) {
        let shift = Shift { motion, mag };
        if let SealedMotion::Search(search) = motion {
            tui.search = Some(if cur == ',' { search.reverse() } else { search })
        }

        if del {
            shift.to_cut().invoke(tui)?;
        } else {
            shift.invoke(tui)?;
        }
    } else {
        if let Some(action) = process_edit_bare(cur, buf, mag) {
            action.invoke(tui)?;
        }
    }

    if tui.key_buffer.len() == key_len {
        KeyBuffer::Clear.invoke(tui)?;
    }

    if change {
        SetMode(Mode::LineInsert).invoke(tui)?;
    }

    Ok(())
}

fn map_motion(ch: char, ctx: &str, tui: &Tui) -> Option<SealedMotion> {
    match ctx.chars().last() {
        Some('f') => return Some(Search::ForwardFind(ch).into()),
        Some('F') => return Some(Search::BackwardFind(ch).into()),
        Some('t') => return Some(Search::ForwardTo(ch).into()),
        Some('T') => return Some(Search::BackwardTo(ch).into()),
        _ => (),
    }

    match ch {
        'w' => Some(CClass::ForwardWord.into()),
        'b' => Some(CClass::BackwardWord.into()),
        'W' => Some(CClass::ForwardBlank.into()),
        'B' => Some(CClass::BackwardBlank.into()),

        '0' => Some(Absolute::First.into()),
        '$' => Some(Absolute::Last.into()),
        'h' => Some(Relative::Left.into()),
        'l' => Some(Relative::Right.into()),

        ';' => tui.search.map(|s| s.into()),
        ',' => tui.search.map(|s| s.reverse().into()),

        _ => return None,
    }
}

fn process_edit_bare(cur: char, ctx: &str, mag: usize) -> Option<SealedAction<SealedMotion>> {
    let action: SealedAction<SealedMotion> = match (cur, ctx) {
        ('k', _) => History::Past.into(),
        ('j', _) => History::Recent.into(),
        ('i', _) => Transition::Insert.into(),
        ('I', _) => Transition::HardInsert.into(),
        ('a', _) => Transition::Append.into(),
        ('A', _) => Transition::HardAppend.into(),
        ('D', _) => Edit::CutRest.into(),
        ('x', _) => Edit::CutTil(Some(mag)).into(),
        ('d', "d") => Edit::CutAll.into(),
        (ch, _) if ch.is_digit(10) || "FfTtdc".contains(ch) => KeyBuffer::Push(ch).into(),

        _ => return None,
    };

    Some(action)
}

fn parts(key_buffer: &str) -> (usize, &str) {
    let mut i = 0;
    for ch in key_buffer.chars() {
        if !ch.is_digit(10) {
            break;
        }
        i += 1;
    }

    (key_buffer[..i].parse().unwrap_or(1), &key_buffer[i..])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(parts("123df'"), (123, "df'"));
        assert_eq!(parts("df'"), (1, "df'"));
        assert_eq!(parts("2"), (2, ""));
    }
}
