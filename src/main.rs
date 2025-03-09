mod config;
mod decode;
mod image;

use clap::Parser;
use config::Config;
use decode::decode_visual_data;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    #[allow(unused_mut)]
    let config = Config::parse();

    let input = config.input.clone();
    let output = config.output.clone();

    if !input.exists() {
        eprintln!("Couldn't find input file");
    }

    decode_visual_data(input.to_str().unwrap(), &config)?.draw_and_save(&output, &config)?;

    Ok(ExitCode::SUCCESS)
}
