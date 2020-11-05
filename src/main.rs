use anyhow::Result;
use clap_verbosity_flag::Verbosity;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};
use structopt::StructOpt;

#[macro_use]
extern crate log;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let log_config = ConfigBuilder::new().build();
    if let Some(level) = opt.verbose.log_level() {
        TermLogger::init(level.to_level_filter(), log_config, TerminalMode::Mixed)?;
    }
    debug!("opt={:?}", opt);
    Ok(())
}
