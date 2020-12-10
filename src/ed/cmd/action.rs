use super::*;
use crate::ed::addr::{LineResolver, RangeResolver};
use crate::Interp;

impl Command {
    pub(crate) fn invoke(&self, interp: &mut Interp) -> bool {
        use Command::*;

        match self {
            Print(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    print(interp, start, end);
                    true
                } else {
                    false
                }
            }

            Delete(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    delete(interp, start, end);
                    true
                } else {
                    false
                }
            }

            Mark(offset, mark) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.marks.insert(*mark, line);
                    true
                } else {
                    false
                }
            }

            Nop(offset) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.buffer.cur = line;
                    true
                } else {
                    false
                }
            }

            Append(_) | Insert(_) | Change(_) => unreachable!(),
        }
    }

    pub(crate) fn invoke_with_text(&self, interp: &mut Interp, lines: Vec<String>) -> bool {
        use Command::*;

        match self {
            Append(line_ref) => {
                if let Some(line) = line_ref.resolve_line(interp) {
                    interp.buffer.append(line, lines);
                    true
                } else {
                    false
                }
            }
            Insert(line_ref) => {
                if let Some(line) = line_ref.resolve_line(interp) {
                    interp.buffer.insert(line, lines);
                    true
                } else {
                    false
                }
            }
            Change(line_ref) => {
                if let Some((start, end)) = line_ref.resolve_range(interp) {
                    interp.buffer.change(start, end, lines);
                    true
                } else {
                    false
                }
            }

            _ => unreachable!(),
        }
    }
}

fn print(interp: &mut Interp, start: usize, end: usize) {
    for line in start..=end {
        if let Some(l) = interp.buffer.line(line) {
            println!("{}", l)
        }
    }

    interp.buffer.cur = end
}

fn delete(interp: &mut Interp, start: usize, end: usize) {
    interp.buffer.remove(start, end);
    interp.buffer.cur = start;
}
