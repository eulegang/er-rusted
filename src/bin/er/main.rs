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
    expression: Option<String>,

    #[structopt(short = "V", long = "visual")]
    visual: bool,
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    if let Some(file) = opt.script {
        let mut script = Script::from_file(&file, opt.inplace, opt.files)
            .wrap_err("failed to build script from file")?;

        script.run()
    } else if let Some(expr) = opt.expression {
        let mut script = Script::from_expr(&expr, opt.inplace, opt.files)
            .wrap_err("failed to build script from expression")?;

        script.run()
    } else {
        if !atty::is(atty::Stream::Stdin) {
            bail!("prompt used noninteractively");
        }

        let visual_env = std::env::var("ER_VISUAL").is_ok();

        if opt.visual || visual_env {
            let mut tui = Tui::new(opt.files).wrap_err("failed to build tui")?;

            tui.run()
        } else {
            let mut repl = Repl::new(opt.files).wrap_err("failed to build ui")?;

            repl.run()
        }
    }
}
