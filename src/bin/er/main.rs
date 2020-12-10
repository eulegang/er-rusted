use er_rusted::{
    ed::{Command, CommandResult},
    Interp,
};
use eyre::WrapErr;
use rustyline::{error::ReadlineError, Editor, Helper};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    files: Vec<String>,
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    let mut interp = Interp::new(opt.files).wrap_err("failed to build")?;

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                let cmd = match Command::from_str(&line) {
                    Ok(cmd) => cmd,
                    Err(_) => {
                        eprintln!("< Invalid command");
                        continue;
                    }
                };

                let result = if cmd.needs_text() {
                    let lines = match read_text_mode(&mut rl) {
                        Ok(lines) => lines,
                        Err(ReadlineError::Interrupted) => continue,
                        Err(ReadlineError::Eof) => break,
                        Err(err) => {
                            eprintln!("< error: {:?}", err);
                            break;
                        }
                    };

                    interp.exec_with_text(cmd, lines)
                } else {
                    interp.exec(cmd)
                };

                match result {
                    CommandResult::Failed => eprintln!("< Failed"),
                    CommandResult::Success => (),
                    CommandResult::Quit => break,
                };
            }

            Err(ReadlineError::Interrupted) => {
                continue;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                eprintln!("< error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn read_text_mode<T: Helper>(rl: &mut Editor<T>) -> Result<Vec<String>, ReadlineError> {
    let mut buf = Vec::new();
    loop {
        let line = rl.readline("")?;

        if line.as_str() == "." {
            break;
        }

        buf.push(line);
    }

    Ok(buf)
}
