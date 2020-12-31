use er_rusted::ui::{Repl, Script, UI};
use eyre::{bail, WrapErr};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    files: Vec<String>,

    #[structopt(short = "f", long = "file", name = "file")]
    script: Option<String>,
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    if let Some(file) = opt.script {
        let mut script = Script::from_file(&file, opt.files).wrap_err("failed to build ui")?;

        script.run()
    } else {
        if !atty::is(atty::Stream::Stdin) {
            bail!("prompt used noninteractively");
        }

        let mut repl = Repl::new(opt.files).wrap_err("failed to build ui")?;

        repl.run()
    }
}
