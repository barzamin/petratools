use std::path::PathBuf;

use structopt::StructOpt;
use color_eyre::eyre::Result;

mod data;
mod map;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Opts::from_args();

    let mapfile = std::fs::read_to_string(opts.input)?;
    let mapfile = map::parse(mapfile)?;
    println!("{:#?}", mapfile);

    Ok(())
}