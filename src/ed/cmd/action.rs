use super::*;
use crate::ed::addr::{LineResolver, RangeResolver};
use crate::Interp;

impl Command {
    pub(crate) fn invoke(&self, interp: &mut Interp) -> CommandResult {
        use Command::*;

        match self {
            Print(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    print(interp, start, end);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Delete(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    delete(interp, start, end);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Mark(offset, mark) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.marks.insert(*mark, line);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Join(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    join(interp, start, end);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Move(addr, offset) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(to) = offset.resolve_line(interp) {
                        let lines = match interp.buffer.remove(start, end) {
                            Some(d) => d.collect::<Vec<String>>(),
                            None => return CommandResult::Failed,
                        };

                        interp.buffer.insert(to, lines);
                        CommandResult::Success
                    } else {
                        CommandResult::Failed
                    }
                } else {
                    CommandResult::Failed
                }
            }

            Quit => CommandResult::Quit,

            Nop(offset) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.buffer.cur = line;
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Append(_) | Insert(_) | Change(_) => unreachable!(),
        }
    }

    pub(crate) fn invoke_with_text(
        &self,
        interp: &mut Interp,
        lines: Vec<String>,
    ) -> CommandResult {
        use Command::*;

        match self {
            Append(line_ref) => {
                if let Some(line) = line_ref.resolve_line(interp) {
                    interp.buffer.append(line, lines);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }
            Insert(line_ref) => {
                if let Some(line) = line_ref.resolve_line(interp) {
                    interp.buffer.insert(line, lines);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }
            Change(line_ref) => {
                if let Some((start, end)) = line_ref.resolve_range(interp) {
                    interp.buffer.change(start, end, lines);
                    CommandResult::Success
                } else {
                    CommandResult::Failed
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

fn join(interp: &mut Interp, start: usize, end: usize) {
    let lines = match interp.buffer.remove(start, end) {
        Some(d) => d.collect::<Vec<String>>(),
        None => return,
    };

    let mut it = lines.into_iter();

    if let Some(mut insert) = it.next() {
        while let Some(line) = it.next() {
            insert.push(' ');
            insert.push_str(line.trim_start());
        }

        interp.buffer.insert(start, vec![insert]);
    }
}
