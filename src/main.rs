mod config;
mod error;

use clap::Parser;
use config::{Args, Config};

#[cfg(feature = "config_file")]
use config::Config;

pub use error::{Error, Result};

fn main() -> Result<()> {
    #[allow(unused_mut)]
    let mut config = Config::default();
    let args = Args::parse();

    #[cfg(feature = "config_file")]
    {
        match Config::load() {
            Ok(conf) => config = conf,
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    println!("{config:#?}");
    println!("{args:#?}");

    Ok(())
}
