mod cli;

use anyhow::Result;
use cli::Opt;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate tracing_attributes;

fn main() -> Result<()> {
    let opt = Opt::init_from_args()?;
    debug!("opt={:?}", opt);
    info!("Hello, world!");
    Ok(())
}
