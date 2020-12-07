use er_rusted::{ed::Command, Interp};
use rustyline::{error::ReadlineError, Editor};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    files: Vec<String>,
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    let mut interp = Interp::new(opt.files);

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("* ");

        match readline {
            Ok(line) => match Command::from_str(&line) {
                Ok(cmd) => println!("{:?}", cmd),
                Err(_) => eprintln!("Invalid command"),
            },

            Err(ReadlineError::Interrupted) => {
                break;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
