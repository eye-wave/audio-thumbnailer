mod config;
mod decode;
mod error;
mod image;

use clap::Parser;
use config::{Args, Config};
use decode::decode_visual_data;

#[cfg(feature = "config_file")]
use config::ConfigDeserialize;

pub use error::{Error, Result};

fn main() -> Result<()> {
    #[allow(unused_mut)]
    let mut config = Config::default();
    let args = Args::parse();

    if config.debug.enabled {
        println!("Logging into {:?}", config.debug.log_file);
    }

    let input = args.input.clone();
    let output = args.output.clone();

    #[cfg(feature = "config_file")]
    {
        if args.init_config {
            ConfigDeserialize::create_file()?;
            return Ok(());
        }

        if input.is_none() && output.is_none() {
            let error_message =
                "\x1b[31merror:\x1b[0m the following required arguments were not provided:
\t\x1b[32m--input <INPUT>\x1b[0m
\t\x1b[32m--output <OUTPUT>\x1b[0m
\n\x1b[4mUsage:\x1b[0m audio-thumbnailer --input <INPUT> --output <OUTPUT>
For more information, try '--help'.";

            println!("{error_message}");
            return Ok(());
        }

        match ConfigDeserialize::load() {
            Ok(conf) => config = conf,
            Err(err) => {
                println!("{}", err);
            }
        }
    };

    #[cfg(feature = "config_file")]
    let input = input.unwrap();
    #[cfg(feature = "config_file")]
    let output = output.unwrap();

    if !input.exists() {
        return Err(Error::Custom(format!(
            "Couldn't find input file '{input:?}'"
        )));
    }

    args.apply_to_config(&mut config);

    decode_visual_data(&input, &config)?.draw_and_save(&output, &config)?;

    Ok(())
}
