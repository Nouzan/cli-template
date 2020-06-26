use clap_verbosity_flag::Verbosity;
use structopt::StructOpt;
use simplelog::{ Config, TermLogger, TerminalMode };
use anyhow::Result;

#[macro_use]
extern crate log;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()>{
    let opt = Opt::from_args();
    if let Some(level) = opt.verbose.log_level() {
        TermLogger::init(level.to_level_filter(), Config::default(), TerminalMode::Mixed)?;
    }
    debug!("opt={:?}", opt);
    Ok(())
}
