use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///input file name
    #[arg(short, long)]
    #[cfg(feature = "config_file")]
    pub input: Option<PathBuf>,

    ///input file name
    #[arg(short, long)]
    #[cfg(not(feature = "config_file"))]
    pub input: PathBuf,

    ///output file name of the generated thumbnail
    #[arg(short, long)]
    #[cfg(feature = "config_file")]
    pub output: Option<PathBuf>,

    ///output file name of the generated thumbnail
    #[arg(short, long)]
    #[cfg(not(feature = "config_file"))]
    pub output: PathBuf,

    /// creates a config directory with an example config file
    #[arg(long, action)]
    #[cfg(feature = "config_file")]
    pub init_config: bool,
}
