use super::*;
use crate::ui::tui::action::*;
use crate::ui::tui::motion::*;
use crossterm::event::KeyEvent;
use std::cmp::{max, min};

pub(crate) fn process_line_edit(event: Event, tui: &mut Tui) -> eyre::Result<bool> {
    let key = match event {
        Event::Key(key) => key,
        _ => return Ok(true),
    };

    let digits: Option<usize> = tui.key_buffer.parse().ok();

    let op = tui.key_buffer.chars().last();

    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        tui.mode = Mode::Cmd;
        tui.hide_cursor()?;
        tui.history.reset();
        tui.cmd.clear();
        tui.key_buffer.clear();

        tui.draw_cmd()?;
        return Ok(true);
    }

    let result = if tui.key_buffer.contains("d") || tui.key_buffer.contains("c") {
        let root = tui.cursor;
        let res = match op {
            Some(ch) if "FfTt".contains(ch) => process_search(key, ch, digits, tui),
            _ => process_basic_motion(key, digits, tui),
        };

        if root != tui.cursor {
            let (low, high) = (min(root, tui.cursor), max(root, tui.cursor));

            tui.cmd.drain(low..=high);
            tui.cursor = low;

            tui.key_buffer.clear();
            tui.draw_cmd()?;
            tui.draw_cursor()?;
        }

        res
    } else {
        match op {
            Some(ch) if "FfTt".contains(ch) => process_search(key, ch, digits, tui),
            _ => process_bare(key, digits, tui),
        }
    };

    if !result? {
        return Ok(false);
    }

    tui.stdout.flush()?;
    Ok(true)
}

fn process_bare(key: KeyEvent, digits: Option<usize>, tui: &mut Tui) -> eyre::Result<bool> {
    let mut appended = false;
    match key.code {
        KeyCode::Enter => {
            Run.invoke(tui)?;
        }

        KeyCode::Char('k') => {
            History::Past.invoke(tui)?;
        }

        KeyCode::Char('j') => {
            History::Recent.invoke(tui)?;
        }

        KeyCode::Char('h') => {
            let mag = digits.unwrap_or(1);
            let motion = Relative::Left;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('l') => {
            let mag = digits.unwrap_or(1);
            let motion = Relative::Right;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('i') => {
            Transition::Insert.invoke(tui)?;
        }

        KeyCode::Char('I') => {
            Transition::HardInsert.invoke(tui)?;
        }

        KeyCode::Char('a') => {
            Transition::Append.invoke(tui)?;
        }

        KeyCode::Char('A') => {
            Transition::HardAppend.invoke(tui)?;
        }

        KeyCode::Char('D') => {
            Edit::CutRest.invoke(tui)?;
        }

        KeyCode::Char('x') => {
            Edit::CutTil(digits).invoke(tui)?;
        }

        KeyCode::Char('w') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::ForwardWord;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('b') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::BackwardWord;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('W') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::ForwardBlank;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('B') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::BackwardBlank;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char(';') => {
            if let Some(motion) = tui.search {
                let mag = digits.unwrap_or(1);
                Shift { mag, motion }.invoke(tui)?;
            }
        }

        KeyCode::Char(',') => {
            if let Some(motion) = tui.search {
                let motion = motion.reverse();
                let mag = digits.unwrap_or(1);
                Shift { mag, motion }.invoke(tui)?;
            }
        }

        KeyCode::Char('0') if tui.key_buffer.is_empty() => {
            let mag = 1;
            let motion = Absolute::First;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('$') => {
            let mag = 1;
            let motion = Absolute::Last;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char(digit) if digit.is_digit(10) => {
            tui.key_buffer.push(digit);
            tui.draw_cmd()?;
            appended = true
        }

        KeyCode::Char(ch) if "FfTtdc".contains(ch) => {
            tui.key_buffer.push(ch);
            tui.draw_cmd()?;
            appended = true
        }

        _ => (),
    }

    if !appended {
        tui.key_buffer.clear()
    }

    Ok(true)
}

fn process_search(
    key: KeyEvent,
    op: char,
    digits: Option<usize>,
    tui: &mut Tui,
) -> eyre::Result<bool> {
    let key = match key.code {
        KeyCode::Char(key) => key,
        _ => return Ok(true),
    };

    match op {
        'f' => {
            let motion = Search::ForwardFind(key);
            let mag = digits.unwrap_or(1);
            Shift { mag, motion }.invoke(tui)?;

            tui.search = Some(motion);
            tui.draw_cursor()?;
        }

        'F' => {
            let motion = Search::BackwardFind(key);
            let mag = digits.unwrap_or(1);
            Shift { mag, motion }.invoke(tui)?;

            tui.search = Some(motion);
            tui.draw_cursor()?;
        }

        't' => {
            let motion = Search::ForwardTo(key);
            let mag = digits.unwrap_or(1);
            Shift { mag, motion }.invoke(tui)?;

            tui.search = Some(motion);
            tui.draw_cursor()?;
        }

        'T' => {
            let motion = Search::BackwardTo(key);
            let mag = digits.unwrap_or(1);
            Shift { mag, motion }.invoke(tui)?;

            tui.search = Some(motion);
            tui.draw_cursor()?;
        }

        _ => unreachable!(),
    }

    tui.key_buffer.clear();
    tui.draw_cmd()?;

    Ok(true)
}

fn process_basic_motion(key: KeyEvent, digits: Option<usize>, tui: &mut Tui) -> eyre::Result<bool> {
    match key.code {
        KeyCode::Char('w') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::ForwardWord;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('b') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::BackwardWord;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('W') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::ForwardBlank;
            Shift { mag, motion }.invoke(tui)?;
        }

        KeyCode::Char('B') => {
            let mag = digits.unwrap_or(1);
            let motion = CClass::BackwardBlank;
            Shift { mag, motion }.invoke(tui)?;
        }

        _ => (),
    }

    Ok(true)
}
