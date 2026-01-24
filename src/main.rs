mod config;
mod decode;
mod image;

use clap::Parser;
use config::Args;
use decode::decode_visual_data;

use crate::{decode::DecoderError, image::DrawError};

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("{0}")]
    Decode(#[from] DecoderError),

    #[error("{0}")]
    Draw(#[from] DrawError),
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let input = args.input.clone();
    let output = args.output.clone();

    if !input.exists() {
        eprintln!("Couldn't find input file");
    }

    decode_visual_data(input.to_str().unwrap(), &args.config)?
        .draw_and_save(&output, &args.config)?;

    Ok(())
}
