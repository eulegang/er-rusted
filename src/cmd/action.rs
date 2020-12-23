use super::*;
use crate::addr::{LineResolver, RangeResolver};
use crate::interp::{Action, Env};
use crate::Buffer;
use regex::Captures;

impl Command {
    pub(crate) fn invoke(&self, buffer: &mut Buffer, env: &Env) -> Result<Vec<Action>, ()> {
        use Command::*;

        match self {
            Print(addr) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;

                for line in start..=end {
                    if let Some(l) = buffer.line(line) {
                        println!("{}", l)
                    }
                }

                Ok(vec![])
            }

            Delete(addr) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;
                buffer.remove(start, end);
                buffer.cur = start;
                Ok(vec![])
            }

            Mark(offset, mark) => {
                let line = offset.resolve_line(buffer, env).ok_or(())?;

                Ok(vec![Action::Mark(*mark, line)])
            }

            Join(addr) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;

                let lines: Vec<String> = buffer.remove(start, end).ok_or(())?.collect();
                let mut it = lines.into_iter();

                if let Some(mut insert) = it.next() {
                    while let Some(line) = it.next() {
                        insert.push(' ');
                        insert.push_str(line.trim_start());
                    }

                    buffer.insert(start, vec![insert]);
                }

                Ok(vec![])
            }

            Move(addr, offset) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;
                let to = offset.resolve_line(buffer, env).ok_or(())?;
                let lines = buffer.remove(start, end).ok_or(())?.collect();
                buffer.insert(to, lines);
                Ok(vec![])
            }

            Transfer(addr, offset) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;
                let to = offset.resolve_line(buffer, env).ok_or(())?;
                let lines = buffer.range(start, end).ok_or(())?;
                buffer.insert(to, lines);
                Ok(vec![])
            }

            Yank(addr) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;
                let lines = buffer.range(start, end).ok_or(())?;

                Ok(vec![Action::SetCut(lines)])
            }

            Paste(offset) => {
                let line = offset.resolve_line(buffer, env).ok_or(())?;
                buffer.insert(line, env.cut.clone());
                Ok(vec![])
            }

            Write(addr, syncer, quit) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;
                let lines = buffer.range(start, end).ok_or(())?.to_vec();
                syncer.sync(buffer, env, &lines);
                let mut actions = Vec::with_capacity(2);

                if let SysPoint::Command(Cmd::System(cmd)) = syncer {
                    actions.push(Action::SetWCmd(cmd.to_string()))
                }

                if *quit {
                    actions.push(Action::Quit);
                }

                Ok(actions)
            }

            Read(offset, src) => {
                let line = offset.resolve_line(buffer, env).ok_or(())?;
                if !buffer.append(line, src.source(buffer, env).ok_or(())?) {
                    return Err(());
                }

                if let SysPoint::Command(Cmd::System(cmd)) = src {
                    Ok(vec![Action::SetRCmd(cmd.to_string())])
                } else {
                    Ok(vec![])
                }
            }

            Run(cmd) => {
                if !cmd.run(env) {
                    return Err(());
                }

                if let Cmd::System(cmd) = cmd {
                    Ok(vec![Action::SetCmd(cmd.to_string())])
                } else {
                    Ok(vec![])
                }
            }

            Subst(addr, re, pat, flags) => {
                let (start, end) = addr.resolve_range(buffer, env).ok_or(())?;

                let flags = flags.unwrap_or_else(|| {
                    if re.is_none() && pat.is_none() {
                        env.last_flags.unwrap_or_default()
                    } else {
                        Default::default()
                    }
                });

                let re = match (re, &env.last_re) {
                    (Some(re), _) | (None, Some(re)) => re.clone(),
                    (None, None) => return Err(()),
                };

                let pat = match (pat, &env.last_pat) {
                    (Some(Pat::Replay), None) | (None, None) => return Err(()),
                    (Some(Pat::Replay), Some(pat)) | (Some(pat), _) | (None, Some(pat)) => {
                        pat.clone()
                    }
                };

                if !run_subst(buffer, start, end, &re, &pat, &flags) {
                    return Err(());
                }

                Ok(vec![Action::SetRe(re), Action::SetPat(pat)])
            }

            Quit => Ok(vec![Action::Quit]),

            Nop(offset) => {
                let line = offset.resolve_line(buffer, env).ok_or(())?;
                buffer.cur = line;
                Ok(vec![])
            }

            Append(line_ref, Some(lines)) => {
                let line = line_ref.resolve_line(buffer, env).ok_or(())?;
                buffer.append(line, lines.clone());
                Ok(vec![])
            }

            Insert(line_ref, Some(lines)) => {
                let line = line_ref.resolve_line(buffer, env).ok_or(())?;
                buffer.insert(line, lines.clone());
                Ok(vec![])
            }

            Change(line_ref, Some(lines)) => {
                let (start, end) = line_ref.resolve_range(buffer, env).ok_or(())?;
                buffer.change(start, end, lines.clone());
                Ok(vec![])
            }

            NextBuffer => Ok(vec![Action::Next]),
            PrevBuffer => Ok(vec![Action::Prev]),

            Append(_, None) | Insert(_, None) | Change(_, None) => {
                panic!("Content must be injected into a, c or i before invoking")
            }
        }
    }
}

fn run_subst(
    buffer: &mut Buffer,
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
        let line = if let Some(line) = buffer.line(i) {
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

        buffer.replace_line(i, replaced);
    }

    replaced
}
