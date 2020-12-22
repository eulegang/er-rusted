use super::*;
use crate::addr::{LineResolver, RangeResolver};
use crate::interp::{Action, Interpreter};
use regex::Captures;

impl Command {
    pub(crate) fn invoke(&self, interp: &mut Interpreter) -> Result<Action, ()> {
        use Command::*;

        match self {
            Print(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    print(interp, start, end);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Delete(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    delete(interp, start, end);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Mark(offset, mark) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.marks.insert(*mark, line);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Join(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    join(interp, start, end);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Move(addr, offset) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(to) = offset.resolve_line(interp) {
                        let lines = match interp.buffer.remove(start, end) {
                            Some(d) => d.collect::<Vec<String>>(),
                            None => return Err(()),
                        };

                        interp.buffer.insert(to, lines);
                        Ok(Action::Nop)
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }

            Transfer(addr, offset) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(to) = offset.resolve_line(interp) {
                        if let Some(lines) = interp.buffer.range(start, end) {
                            interp.buffer.insert(to, lines);
                            Ok(Action::Nop)
                        } else {
                            Err(())
                        }
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }

            Yank(addr) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(lines) = interp.buffer.range(start, end) {
                        interp.cut = lines;
                        Ok(Action::Nop)
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }

            Paste(offset) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.buffer.insert(line, interp.cut.clone());
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            Write(addr, syncer, quit) => {
                if let Some((start, end)) = addr.resolve_range(interp) {
                    if let Some(lines) = interp.buffer.range(start, end) {
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
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }

            Read(offset, src) => {
                if let Some(line) = offset.resolve_line(interp) {
                    let res = match src.source(interp) {
                        Some(lines) => {
                            if interp.buffer.append(line, lines) {
                                Ok(Action::Nop)
                            } else {
                                Err(())
                            }
                        }
                        None => Err(()),
                    };

                    if let SysPoint::Command(Cmd::System(cmd)) = src {
                        interp.last_rcmd = Some(cmd.to_string())
                    }

                    res
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
                if let Some((start, end)) = addr.resolve_range(interp) {
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
                } else {
                    Err(())
                }
            }

            Quit => Ok(Action::Quit),

            Nop(offset) => {
                if let Some(line) = offset.resolve_line(interp) {
                    interp.buffer.cur = line;
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
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
                if let Some(line) = line_ref.resolve_line(interp) {
                    interp.buffer.append(line, lines);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }
            Insert(line_ref) => {
                if let Some(line) = line_ref.resolve_line(interp) {
                    interp.buffer.insert(line, lines);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }
            Change(line_ref) => {
                if let Some((start, end)) = line_ref.resolve_range(interp) {
                    interp.buffer.change(start, end, lines);
                    Ok(Action::Nop)
                } else {
                    Err(())
                }
            }

            _ => unreachable!(),
        }
    }
}

fn print(interp: &mut Interpreter, start: usize, end: usize) {
    for line in start..=end {
        if let Some(l) = interp.buffer.line(line) {
            println!("{}", l)
        }
    }

    interp.buffer.cur = end
}

fn delete(interp: &mut Interpreter, start: usize, end: usize) {
    interp.buffer.remove(start, end);
    interp.buffer.cur = start;
}

fn join(interp: &mut Interpreter, start: usize, end: usize) {
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

    if replaced {
        true
    } else {
        false
    }
}
