use std::path::PathBuf;

use structopt::StructOpt;
use color_eyre::eyre::Result;

mod map;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Opts::from_args();

    let f = std::fs::File::open(opts.input)?;


    Ok(())
}