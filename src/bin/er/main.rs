use er_rusted::ui::{Repl, Script, Tui, UI};
use eyre::{bail, WrapErr};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    files: Vec<String>,

    #[structopt(short, long)]
    inplace: Option<String>,

    #[structopt(short = "f", long = "file", name = "file")]
    script: Option<String>,

    #[structopt(short = "e", long = "expr", name = "expr", conflicts_with("file"))]
    expressions: Vec<String>,

    #[structopt(short = "T", long = "disable-visual")]
    disable_visual: bool,
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    if let Some(file) = opt.script {
        let mut script = Script::from_file(&file, opt.inplace, opt.files)
            .wrap_err("failed to build script from file")?;

        script.run()
    } else if !opt.expressions.is_empty() {
        let mut script = Script::from_expr(opt.expressions, opt.inplace, opt.files)
            .wrap_err("failed to build script from expression")?;

        script.run()
    } else {
        if !atty::is(atty::Stream::Stdin) {
            bail!("prompt used noninteractively");
        }

        if !opt.disable_visual {
            let mut tui = Tui::new(opt.files).wrap_err("failed to build tui")?;

            tui.run()
        } else {
            let mut repl = Repl::new(opt.files).wrap_err("failed to build ui")?;

            repl.run()
        }
    }
}
