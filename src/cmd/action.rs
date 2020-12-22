use super::*;
use crate::addr::{LineResolver, RangeResolver};
use crate::interp::{Action, Interpreter};
use regex::Captures;

impl Command {
    pub(crate) fn invoke(&self, interp: &mut Interpreter) -> Result<Action, ()> {
        use Command::*;

        match self {
            Print(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;

                for line in start..=end {
                    if let Some(l) = interp.buffer.line(line) {
                        println!("{}", l)
                    }
                }

                Ok(Action::Nop)
            }

            Delete(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                interp.buffer.remove(start, end);
                interp.buffer.cur = start;
                Ok(Action::Nop)
            }

            Mark(offset, mark) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                interp.marks.insert(*mark, line);
                Ok(Action::Nop)
            }

            Join(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;

                let lines: Vec<String> = interp.buffer.remove(start, end).ok_or(())?.collect();
                let mut it = lines.into_iter();

                if let Some(mut insert) = it.next() {
                    while let Some(line) = it.next() {
                        insert.push(' ');
                        insert.push_str(line.trim_start());
                    }

                    interp.buffer.insert(start, vec![insert]);
                }

                Ok(Action::Nop)
            }

            Move(addr, offset) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let to = offset.resolve_line(interp).ok_or(())?;
                let lines = interp.buffer.remove(start, end).ok_or(())?.collect();
                interp.buffer.insert(to, lines);
                Ok(Action::Nop)
            }

            Transfer(addr, offset) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let to = offset.resolve_line(interp).ok_or(())?;
                let lines = interp.buffer.range(start, end).ok_or(())?;
                interp.buffer.insert(to, lines);
                Ok(Action::Nop)
            }

            Yank(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                interp.cut = interp.buffer.range(start, end).ok_or(())?;
                Ok(Action::Nop)
            }

            Paste(offset) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                interp.buffer.insert(line, interp.cut.clone());
                Ok(Action::Nop)
            }

            Write(addr, syncer, quit) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let lines = interp.buffer.range(start, end).ok_or(())?;
                syncer.sync(interp, &lines);
                let res = if *quit {
                    Ok(Action::Quit)
                } else {
                    Ok(Action::Nop)
                };

                if let SysPoint::Command(Cmd::System(cmd)) = syncer {
                    interp.last_wcmd = Some(cmd.to_string())
                }

                res
            }

            Read(offset, src) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                let res = interp.buffer.append(line, src.source(interp).ok_or(())?);

                if let SysPoint::Command(Cmd::System(cmd)) = src {
                    interp.last_rcmd = Some(cmd.to_string())
                }

                if res {
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Run(cmd) => {
                let res = cmd.run(interp);

                if let Cmd::System(cmd) = cmd {
                    interp.last_cmd = Some(cmd.to_string())
                }

                if res {
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Subst(addr, re, pat, flags) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;

                let flags = flags.unwrap_or_else(|| {
                    if re.is_none() && pat.is_none() {
                        interp.last_flags.unwrap_or_default()
                    } else {
                        Default::default()
                    }
                });

                let re = match (re, &interp.last_re) {
                    (Some(re), _) | (None, Some(re)) => re.clone(),
                    (None, None) => return Err(()),
                };

                let pat = match (pat, &interp.last_pat) {
                    (Some(Pat::Replay), None) | (None, None) => return Err(()),
                    (Some(Pat::Replay), Some(pat)) | (Some(pat), _) | (None, Some(pat)) => {
                        pat.clone()
                    }
                };

                let result = run_subst(interp, start, end, &re, &pat, &flags);

                interp.last_re = Some(re);
                interp.last_pat = Some(pat);

                if result {
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Quit => Ok(Action::Quit),

            Nop(offset) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                interp.buffer.cur = line;
                Ok(Action::Nop)
            }

            Append(_) | Insert(_) | Change(_) => unreachable!(),
        }
    }

    pub(crate) fn invoke_with_text(
        &self,
        interp: &mut Interpreter,
        lines: Vec<String>,
    ) -> Result<Action, ()> {
        use Command::*;

        match self {
            Append(line_ref) => {
                let line = line_ref.resolve_line(interp).ok_or(())?;
                interp.buffer.append(line, lines);
                Ok(Action::Nop)
            }
            Insert(line_ref) => {
                let line = line_ref.resolve_line(interp).ok_or(())?;
                interp.buffer.insert(line, lines);
                Ok(Action::Nop)
            }
            Change(line_ref) => {
                let (start, end) = line_ref.resolve_range(interp).ok_or(())?;
                interp.buffer.change(start, end, lines);
                Ok(Action::Nop)
            }

            _ => unreachable!(),
        }
    }
}

fn run_subst(
    interp: &mut Interpreter,
    start: usize,
    end: usize,
    re: &Re,
    pat: &Pat,
    flags: &SubstFlags,
) -> bool {
    let mut replaced = false;

    if !pat.compatible(re) {
        return false;
    }

    for i in start..=end {
        let line = if let Some(line) = interp.buffer.line(i) {
            line.clone()
        } else {
            continue;
        };

        let replaced = re
            .replacen(&line, flags.occurances, |cap: &Captures| {
                replaced = true;
                pat.expand(&cap)
            })
            .to_string();

        if flags.print {
            println!("{}", replaced);
        }

        interp.buffer.replace_line(i, replaced);
    }

    replaced
}
