use super::*;
use crate::ed::addr::{LineResolver, RangeResolver};
use crate::Interp;
use regex::Captures;

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

            Transfer(addr, offset) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(to) = offset.resolve_line(interp) {
                        if let Some(lines) = interp.buffer.range(start, end) {
                            interp.buffer.insert(to, lines);
                            CommandResult::Success
                        } else {
                            CommandResult::Failed
                        }
                    } else {
                        CommandResult::Failed
                    }
                } else {
                    CommandResult::Failed
                }
            }

            Yank(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(lines) = interp.buffer.range(start, end) {
                        interp.cut = lines;
                        CommandResult::Success
                    } else {
                        CommandResult::Failed
                    }
                } else {
                    CommandResult::Failed
                }
            }

            Paste(offset) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.buffer.insert(line, interp.cut.clone());
                    CommandResult::Success
                } else {
                    CommandResult::Failed
                }
            }

            Subst(addr, re, pat) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    let re = match (re, &interp.last_re) {
                        (Some(re), _) | (None, Some(re)) => re.clone(),
                        (None, None) => return CommandResult::Failed,
                    };

                    let pat = match (pat, &interp.last_pat) {
                        (Some(Pat::Replay), None) | (None, None) => return CommandResult::Failed,
                        (Some(Pat::Replay), Some(pat)) | (Some(pat), _) | (None, Some(pat)) => {
                            pat.clone()
                        }
                    };

                    let result = run_subst(interp, start, end, &re, &pat);

                    interp.last_re = Some(re);
                    interp.last_pat = Some(pat);

                    result
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

fn run_subst(interp: &mut Interp, start: usize, end: usize, re: &Re, pat: &Pat) -> CommandResult {
    let mut replaced = false;

    if !pat.compatible(re) {
        return CommandResult::Failed;
    }

    for i in start..=end {
        let line = if let Some(line) = interp.buffer.line(i) {
            line.clone()
        } else {
            continue;
        };

        let replaced = re
            .replace_all(&line, |cap: &Captures| {
                replaced = true;
                pat.expand(&cap)
            })
            .to_string();

        interp.buffer.replace_line(i, replaced);
    }

    if replaced {
        CommandResult::Success
    } else {
        CommandResult::Failed
    }
}
