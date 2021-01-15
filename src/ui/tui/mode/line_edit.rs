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

    let op = buf.chars().last();
    let key_len = tui.key_buffer.len();

    let change = buf.contains('c');
    let del = buf.contains('d') || change;

    if let Some(motion) = map_motion(cur, op, tui) {
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
        process_edit_bare(cur, op, mag, tui)?;
    }

    if tui.key_buffer.len() == key_len {
        KeyBuffer::Clear.invoke(tui)?;
    }

    if change {
        SetMode(Mode::LineInsert).invoke(tui)?;
    }

    Ok(())
}

fn map_motion(ch: char, op: Option<char>, tui: &Tui) -> Option<SealedMotion> {
    match (ch, op) {
        (_, Some('f')) => Some(Search::ForwardFind(ch).into()),
        (_, Some('F')) => Some(Search::BackwardFind(ch).into()),
        (_, Some('t')) => Some(Search::ForwardTo(ch).into()),
        (_, Some('T')) => Some(Search::BackwardTo(ch).into()),

        ('w', _) => Some(CClass::ForwardWord.into()),
        ('b', _) => Some(CClass::BackwardWord.into()),
        ('W', _) => Some(CClass::ForwardBlank.into()),
        ('B', _) => Some(CClass::BackwardBlank.into()),

        ('0', _) => Some(Absolute::First.into()),
        ('$', _) => Some(Absolute::Last.into()),
        ('h', _) => Some(Relative::Left.into()),
        ('l', _) => Some(Relative::Right.into()),

        (';', _) => tui.search.map(|s| s.into()),
        (',', _) => tui.search.map(|s| s.reverse().into()),

        _ => return None,
    }
}

fn process_edit_bare(cur: char, op: Option<char>, mag: usize, tui: &mut Tui) -> eyre::Result<()> {
    match (cur, op) {
        ('k', _) => History::Past.invoke(tui)?,
        ('j', _) => History::Recent.invoke(tui)?,
        ('i', _) => Transition::Insert.invoke(tui)?,
        ('I', _) => Transition::HardInsert.invoke(tui)?,
        ('a', _) => Transition::Append.invoke(tui)?,
        ('A', _) => Transition::HardAppend.invoke(tui)?,
        ('D', _) => Edit::CutRest.invoke(tui)?,
        ('x', _) => Edit::CutTil(Some(mag)).invoke(tui)?,
        ('d', Some('d')) => Edit::CutAll.invoke(tui)?,

        (ch, _) if ch.is_digit(10) || "FfTtdc".contains(ch) => {
            KeyBuffer::Push(ch).invoke(tui)?;
        }

        _ => (),
    }

    Ok(())
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
