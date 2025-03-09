mod enums;
mod structs;

pub use enums::{AspectRatio, InterpolationType};
pub use structs::*;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///input file name
    #[arg(short, long)]
    pub input: PathBuf,

    ///output file name of the generated thumbnail
    #[arg(short, long)]
    pub output: PathBuf,

    #[clap(flatten)]
    pub config: Config,
}
