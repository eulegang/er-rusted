use super::*;
use crate::interp::scratch::ScratchPad;
use crate::interp::Interpreter;
use crate::syspoint::{Cmd, SysPoint};
use regex::Captures;
use std::cmp::{max, min};
use std::fs::File;
use std::io::ErrorKind;

use crate::resolve::{LineResolver, RangeResolver};

#[derive(Debug)]
pub enum MarkMod {
    After {
        start: usize,
        delta: i64,
    },
    Nil,
    Range {
        start: usize,
        end: usize,
        delta: i64,
    },
}

impl Command {
    pub(crate) fn invoke<S: ScratchPad>(
        &self,
        interp: &mut Interpreter<S>,
    ) -> Result<(bool, MarkMod), InvocationError> {
        use Command::*;

        match self {
            Print(addr) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                for line in start..=end {
                    if let Some(l) = interp.buffer.line(line) {
                        interp.scratch.print(l);
                    }
                }

                Ok((true, MarkMod::Nil))
            }

            Scroll(offset, num) => {
                let line = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let num = num.or(interp.env.scroll).unwrap_or(22);
                let pad = digits(interp.buffer.lines());

                for pos in line..(line + num) {
                    if let Some(l) = interp.buffer.line(pos) {
                        let out = format!("{:width$} {}", pos, l, width = pad);
                        interp.scratch.print(&out);
                        interp.buffer.set_cursor(pos);
                    }
                }

                Ok((true, MarkMod::Nil))
            }

            Delete(addr) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                interp.buffer.remove(start, end);
                interp.buffer.set_cursor(start);

                let markmod = MarkMod::After {
                    start,
                    delta: 0 - (1 + end - start) as i64,
                };

                Ok((true, markmod))
            }

            Mark(offset, mark) => {
                let line = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                interp.buffer.make_mark(*mark, line);

                Ok((true, MarkMod::Nil))
            }

            Join(addr) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                let lines: Vec<String> = interp
                    .buffer
                    .remove(start, end)
                    .ok_or(InvocationError::AddressNonResolvable)?
                    .collect();
                let mut it = lines.into_iter();

                if let Some(mut insert) = it.next() {
                    while let Some(line) = it.next() {
                        insert.push(' ');
                        insert.push_str(line.trim_start());
                    }

                    interp.buffer.insert(start, vec![insert]);
                }

                Ok((
                    true,
                    MarkMod::After {
                        start,
                        delta: 0 - (end - start) as i64,
                    },
                ))
            }

            Move(addr, offset) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let target = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                if start <= target && target <= end {
                    return Err(InvocationError::InvalidTarget);
                }

                let to = if target > end {
                    target - (1 + (end - start))
                } else {
                    target
                };

                let lines = interp
                    .buffer
                    .remove(start, end)
                    .ok_or(InvocationError::AddressNonResolvable)?
                    .collect();
                interp.buffer.append(to, lines);

                let delta = if start < to {
                    -1 - (end - start) as i64
                } else {
                    1 + (end - start) as i64
                };

                Ok((
                    true,
                    MarkMod::Range {
                        start: min(start, to + 1),
                        end: max(to + 1, end),
                        delta,
                    },
                ))
            }

            Transfer(addr, offset) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let to = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let lines = interp
                    .buffer
                    .range(start, end)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                interp.buffer.append(to, lines);

                let markmod = MarkMod::After {
                    start: to,
                    delta: 1 + (end - start) as i64,
                };
                Ok((true, markmod))
            }

            Yank(addr) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let lines = interp
                    .buffer
                    .range(start, end)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                interp.env.cut = lines;

                Ok((true, MarkMod::Nil))
            }

            Paste(offset) => {
                let line = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                interp.buffer.append(line, interp.env.cut.clone());

                let markmod = MarkMod::After {
                    start: line,
                    delta: (interp.env.cut.len() as i64),
                };

                Ok((true, markmod))
            }

            Write(addr, syncer, quit) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let lines = interp
                    .buffer
                    .range(start, end)
                    .ok_or(InvocationError::AddressNonResolvable)?
                    .to_vec();
                syncer.sync(&mut interp.buffer, &interp.env, &lines);

                if let SysPoint::Command(Cmd::System(cmd)) = syncer {
                    interp.env.last_wcmd = Some(cmd.to_string())
                }

                Ok((!*quit, MarkMod::Nil))
            }

            Read(offset, src) => {
                let line = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let lines = src
                    .source(&interp.buffer, &interp.env)
                    .ok_or(InvocationError::UnableToSource)?;
                let delta = lines.len();
                if !interp.buffer.append(line, lines) {
                    return Err(InvocationError::InvalidInsertion);
                }

                if let SysPoint::Command(Cmd::System(cmd)) = src {
                    interp.env.last_rcmd = Some(cmd.to_string())
                }

                Ok((
                    true,
                    MarkMod::After {
                        start: line,
                        delta: delta as i64,
                    },
                ))
            }

            Run(cmd) => {
                if !cmd.run(&interp.env, &mut interp.scratch) {
                    return Err(InvocationError::FailedCommand);
                }

                if let Cmd::System(cmd) = cmd {
                    interp.env.last_cmd = Some(cmd.to_string())
                }

                Ok((true, MarkMod::Nil))
            }

            Edit(point) => {
                match point {
                    SysPoint::Filename => {
                        let filename = interp
                            .env
                            .filename
                            .as_ref()
                            .ok_or(InvocationError::MissingFilename)?;
                        let read = File::open(filename).or(Err(InvocationError::ReadFile))?;
                        interp
                            .buffer
                            .load(read)
                            .or(Err(InvocationError::ReadFile))?;
                    }

                    SysPoint::File(filename) => {
                        let read = File::open(filename).or(Err(InvocationError::ReadFile))?;
                        interp
                            .buffer
                            .load(read)
                            .or(Err(InvocationError::ReadFile))?;
                        interp.env.filename = Some(filename.clone());
                    }

                    SysPoint::Command(cmd) => {
                        let read = cmd.read(&interp.env).or(Err(InvocationError::ReadFile))?;
                        interp
                            .buffer
                            .load(&*read)
                            .or(Err(InvocationError::ReadFile))?;
                        interp.env.filename = None
                    }
                };

                Ok((true, MarkMod::Nil))
            }

            Subst(addr, re, pat, flags) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                let flags = flags.unwrap_or_else(|| {
                    if re.is_none() && pat.is_none() {
                        interp.env.last_flags.unwrap_or_default()
                    } else {
                        Default::default()
                    }
                });

                let re = match (re, &interp.env.last_re) {
                    (Some(re), _) | (None, Some(re)) => re.clone(),
                    (None, None) => return Err(InvocationError::MissingPattern),
                };

                let pat = match (pat, &interp.env.last_pat) {
                    (Some(Pat::Replay), None) | (None, None) => {
                        return Err(InvocationError::MissingPattern)
                    }
                    (Some(Pat::Replay), Some(pat)) | (Some(pat), _) | (None, Some(pat)) => {
                        pat.clone()
                    }
                };

                if !run_subst(
                    &mut interp.buffer,
                    start,
                    end,
                    &re,
                    &pat,
                    &flags,
                    &mut interp.scratch,
                ) {
                    return Err(InvocationError::DidNotReplace);
                }

                interp.env.last_re = Some(re);
                interp.env.last_pat = Some(pat);

                Ok((true, MarkMod::Nil))
            }

            Quit => Ok((false, MarkMod::Nil)),

            Global(addr, re, cmd_list) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                let re = match (re, &interp.env.last_re) {
                    (Some(re), _) | (None, Some(re)) => re.clone(),
                    (None, None) => Err(InvocationError::MissingPattern)?,
                };

                let mut marked = Vec::new();
                for pos in start..=end {
                    if let Some(line) = interp.buffer.line(pos) {
                        if re.is_match(line) {
                            marked.push(pos);
                        }
                    }
                }

                for mark_idx in 0..marked.len() {
                    interp.buffer.set_cursor(marked[mark_idx]);
                    for cmd in cmd_list {
                        let (cont, markmod) = cmd.invoke(interp)?;
                        if !cont {
                            return Ok((false, MarkMod::Nil));
                        }

                        markmod.modify(&mut marked);
                    }
                }

                Ok((true, MarkMod::Nil))
            }

            Void(addr, re, cmd_list) => {
                let (start, end) = addr
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;

                let re = match (re, &interp.env.last_re) {
                    (Some(re), _) | (None, Some(re)) => re.clone(),
                    (None, None) => Err(InvocationError::MissingPattern)?,
                };

                let mut marked = Vec::new();
                for pos in start..=end {
                    if let Some(line) = interp.buffer.line(pos) {
                        if !re.is_match(line) {
                            marked.push(pos);
                        }
                    }
                }

                for mark_idx in 0..marked.len() {
                    interp.buffer.set_cursor(marked[mark_idx]);
                    for cmd in cmd_list {
                        let (cont, markmod) = cmd.invoke(interp)?;
                        if !cont {
                            return Ok((false, MarkMod::Nil));
                        }

                        markmod.modify(&mut marked);
                    }
                }

                Ok((true, MarkMod::Nil))
            }

            Nop(offset) => {
                let line = offset
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                interp.buffer.set_cursor(line);
                Ok((true, MarkMod::Nil))
            }

            Append(line_ref, Some(lines)) => {
                let line = line_ref
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let delta = lines.len() as i64;
                interp.buffer.append(line, lines.clone());
                Ok((true, MarkMod::After { start: line, delta }))
            }

            Insert(line_ref, Some(lines)) => {
                let line = line_ref
                    .resolve_line(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let delta = lines.len() as i64;
                interp.buffer.insert(line, lines.clone());
                Ok((
                    true,
                    MarkMod::After {
                        start: line.checked_sub(1).unwrap_or(0),
                        delta,
                    },
                ))
            }

            Change(line_ref, Some(lines)) => {
                let (start, end) = line_ref
                    .resolve_range(&interp.buffer)
                    .ok_or(InvocationError::AddressNonResolvable)?;
                let delta = lines.len() as i64 - (1 + end - start) as i64;
                interp.buffer.change(start, end, lines.clone());
                Ok((true, MarkMod::After { start, delta }))
            }

            NextBuffer => {
                let filename = interp
                    .filelist
                    .get(interp.filepos + 1)
                    .ok_or(InvocationError::ArgFetch)?;

                let buffer = match File::open(filename) {
                    Ok(f) => Buffer::read(f).or(Err(InvocationError::ReadFile))?,
                    Err(e) if e.kind() == ErrorKind::NotFound => Buffer::default(),
                    Err(_) => Err(InvocationError::ReadFile)?,
                };

                interp.env.filename = Some(filename.to_string());
                interp.buffer = buffer;
                interp.filepos = interp.filepos + 1;

                Ok((true, MarkMod::Nil))
            }

            PrevBuffer => {
                let pos = interp
                    .filepos
                    .checked_sub(1)
                    .ok_or(InvocationError::ArgFetch)?;

                let filename = interp.filelist.get(pos).ok_or(InvocationError::ArgFetch)?;

                let buffer = match File::open(filename) {
                    Ok(f) => Buffer::read(f).or(Err(InvocationError::ReadFile))?,
                    Err(e) if e.kind() == ErrorKind::NotFound => Buffer::default(),
                    Err(_) => Err(InvocationError::ReadFile)?,
                };

                interp.env.filename = Some(filename.to_string());
                interp.buffer = buffer;
                interp.filepos = pos;

                Ok((true, MarkMod::Nil))
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
    scratch: &mut impl ScratchPad,
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
            scratch.print(&replaced);
        }

        buffer.replace_line(i, replaced);
    }

    replaced
}

impl MarkMod {
    pub fn modify(&self, marks: &mut [usize]) {
        if matches!(self, MarkMod::Nil) {
            return;
        }

        for mark in marks {
            if self.check(*mark) {
                self.diff(mark)
            }
        }
    }

    fn check(&self, check: usize) -> bool {
        match self {
            MarkMod::Nil => unreachable!(),
            MarkMod::After { start, .. } => *start < check,
            MarkMod::Range { start, end, .. } => *start < check && check <= *end,
        }
    }

    fn diff(&self, mark: &mut usize) {
        let delta = match self {
            MarkMod::Nil => unreachable!(),
            MarkMod::After { delta, .. } => *delta,
            MarkMod::Range { delta, .. } => *delta,
        };

        let mag = delta.abs() as usize;

        if delta > 0 {
            *mark += mag;
        } else {
            *mark -= mag;
        }
    }
}

fn digits(mut x: usize) -> usize {
    let mut result = 0;
    loop {
        if x == 0 {
            break result;
        }

        x /= 10;
        result += 1;
    }
}
