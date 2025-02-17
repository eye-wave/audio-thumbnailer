mod config;
mod decode;
mod image;

use clap::Parser;
use config::{Args, Config};
use decode::decode_visual_data;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    #[allow(unused_mut)]
    let mut config = Config::default();
    let args = Args::parse();

    if config.debug.enabled {
        println!("Logging into {:?}", config.debug.log_file);
    }

    let input = args.input.clone();
    let output = args.output.clone();

    if !input.exists() {
        eprintln!("Couldn't find input file");
    }

    args.apply_to_config(&mut config);

    decode_visual_data(&input, &config)?.draw_and_save(&output, &config)?;

    Ok(ExitCode::SUCCESS)
}
