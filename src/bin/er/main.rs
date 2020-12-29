use er_rusted::ui::{Repl, UI};
use eyre::WrapErr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    files: Vec<String>,
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    let mut repl = Repl::new(opt.files).wrap_err("failed to build ui")?;

    repl.run()
}
