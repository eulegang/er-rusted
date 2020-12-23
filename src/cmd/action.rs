use super::*;
use crate::addr::{LineResolver, RangeResolver};
use crate::interp::Interpreter;
use regex::Captures;
use std::fs::File;
use std::io::ErrorKind;

impl Command {
    pub(crate) fn invoke(&self, interp: &mut Interpreter) -> Result<bool, ()> {
        use Command::*;

        match self {
            Print(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;

                for line in start..=end {
                    if let Some(l) = interp.buffer.line(line) {
                        println!("{}", l)
                    }
                }

                Ok(true)
            }

            Delete(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                interp.buffer.remove(start, end);
                interp.buffer.cur = start;
                Ok(true)
            }

            Mark(offset, mark) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                interp.env.marks.insert(*mark, line);

                Ok(true)
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

                Ok(true)
            }

            Move(addr, offset) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let to = offset.resolve_line(interp).ok_or(())?;
                let lines = interp.buffer.remove(start, end).ok_or(())?.collect();
                interp.buffer.insert(to, lines);
                Ok(true)
            }

            Transfer(addr, offset) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let to = offset.resolve_line(interp).ok_or(())?;
                let lines = interp.buffer.range(start, end).ok_or(())?;
                interp.buffer.insert(to, lines);
                Ok(true)
            }

            Yank(addr) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let lines = interp.buffer.range(start, end).ok_or(())?;

                interp.env.cut = lines;

                Ok(true)
            }

            Paste(offset) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                interp.buffer.insert(line, interp.env.cut.clone());
                Ok(true)
            }

            Write(addr, syncer, quit) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;
                let lines = interp.buffer.range(start, end).ok_or(())?.to_vec();
                syncer.sync(&mut interp.buffer, &interp.env, &lines);

                if let SysPoint::Command(Cmd::System(cmd)) = syncer {
                    interp.env.last_wcmd = Some(cmd.to_string())
                }

                Ok(!*quit)
            }

            Read(offset, src) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                if !interp
                    .buffer
                    .append(line, src.source(&interp.buffer, &interp.env).ok_or(())?)
                {
                    return Err(());
                }

                if let SysPoint::Command(Cmd::System(cmd)) = src {
                    interp.env.last_rcmd = Some(cmd.to_string())
                }

                Ok(true)
            }

            Run(cmd) => {
                if !cmd.run(&interp.env) {
                    return Err(());
                }

                if let Cmd::System(cmd) = cmd {
                    interp.env.last_cmd = Some(cmd.to_string())
                }

                Ok(true)
            }

            Subst(addr, re, pat, flags) => {
                let (start, end) = addr.resolve_range(interp).ok_or(())?;

                let flags = flags.unwrap_or_else(|| {
                    if re.is_none() && pat.is_none() {
                        interp.env.last_flags.unwrap_or_default()
                    } else {
                        Default::default()
                    }
                });

                let re = match (re, &interp.env.last_re) {
                    (Some(re), _) | (None, Some(re)) => re.clone(),
                    (None, None) => return Err(()),
                };

                let pat = match (pat, &interp.env.last_pat) {
                    (Some(Pat::Replay), None) | (None, None) => return Err(()),
                    (Some(Pat::Replay), Some(pat)) | (Some(pat), _) | (None, Some(pat)) => {
                        pat.clone()
                    }
                };

                if !run_subst(&mut interp.buffer, start, end, &re, &pat, &flags) {
                    return Err(());
                }

                interp.env.last_re = Some(re);
                interp.env.last_pat = Some(pat);

                Ok(true)
            }

            Quit => Ok(false),

            Nop(offset) => {
                let line = offset.resolve_line(interp).ok_or(())?;
                interp.buffer.cur = line;
                Ok(true)
            }

            Append(line_ref, Some(lines)) => {
                let line = line_ref.resolve_line(interp).ok_or(())?;
                interp.buffer.append(line, lines.clone());
                Ok(true)
            }

            Insert(line_ref, Some(lines)) => {
                let line = line_ref.resolve_line(interp).ok_or(())?;
                interp.buffer.insert(line, lines.clone());
                Ok(true)
            }

            Change(line_ref, Some(lines)) => {
                let (start, end) = line_ref.resolve_range(interp).ok_or(())?;
                interp.buffer.change(start, end, lines.clone());
                Ok(true)
            }

            NextBuffer => {
                let filename = interp.filelist.get(interp.filepos + 1).ok_or(())?;

                let buffer = match File::open(filename) {
                    Ok(f) => Buffer::read(f).or(Err(()))?,
                    Err(e) if e.kind() == ErrorKind::NotFound => Buffer::default(),
                    Err(_) => Err(())?,
                };

                interp.env.filename = Some(filename.to_string());
                interp.buffer = buffer;
                interp.filepos = interp.filepos + 1;

                Ok(true)
            }

            PrevBuffer => {
                let pos = interp.filepos.checked_sub(1).ok_or(())?;
                let filename = interp.filelist.get(pos).ok_or(())?;

                let buffer = match File::open(filename) {
                    Ok(f) => Buffer::read(f).or(Err(()))?,
                    Err(e) if e.kind() == ErrorKind::NotFound => Buffer::default(),
                    Err(_) => Err(())?,
                };

                interp.env.filename = Some(filename.to_string());
                interp.buffer = buffer;
                interp.filepos = pos;

                Ok(true)
            }

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
