mod audio;
mod config;
mod error;
mod image;

use audio::AudioDecoder;
use clap::Parser;
use config::{Args, Config};
use image::{cover_art::load_and_resize, parse_color, waveform::draw_waveform, write_image};

pub use error::{Error, Result};

fn main() -> Result<()> {
    let config = Config::default();
    let args = Args::parse();

    let debug_options = config.debug.unwrap_or_default();
    if debug_options.enabled {
        println!("Logging into {:?}", debug_options.log_file);
    }

    #[cfg(not(feature = "config_file"))]
    let input = args.input;
    #[cfg(feature = "config_file")]
    let input = args.input.unwrap();

    #[cfg(not(feature = "config_file"))]
    let output = args.output;
    #[cfg(feature = "config_file")]
    let output = args.output.unwrap();

    #[cfg(feature = "config_file")]
    {
        if args.init_config {
            Config::create_file()?;
            return Ok(());
        } else {
            if args.input.is_none() && args.output.is_none() {
                let error_message =
                    "\x1b[31merror:\x1b[0m the following required arguments were not provided:
\t\x1b[32m--input <INPUT>\x1b[0m
\t\x1b[32m--output <OUTPUT>\x1b[0m
\n\x1b[4mUsage:\x1b[0m audio-thumbnailer --input <INPUT> --output <OUTPUT>
For more information, try '--help'.";

                println!("{error_message}");
                return Ok(());
            }
        }

        match Config::load() {
            Ok(conf) => config = conf,
            Err(err) => {
                println!("{}", err);
            }
        }
    };

    if !input.exists() {
        return Err(Error::Custom(format!(
            "Couldn't find input file '{input:?}'"
        )));
    }

    let mut audio_decoder = AudioDecoder::new();
    let mut probe = audio_decoder
        .create_probe(&input)
        .expect("Failed to create audio decoder");

    let thumbnail_settings = config.thumbnail_settings.unwrap_or_default();
    let cover_settings = config.cover_settings.unwrap_or_default();
    if !cover_settings.no_cover {
        if let Some(image_data) = audio_decoder.get_cover_art(&mut probe) {
            match load_and_resize(&image_data, &cover_settings) {
                Ok(image) => {
                    write_image(image, &output)?;
                    return Ok(());
                }
                Err(err) => {
                    if !thumbnail_settings.waveform_on_fail {
                        return Err(err);
                    }
                }
            }
        }
    }

    let waveform_settings = config.waveform_settings.unwrap_or_default();
    if let Some(samples) = audio_decoder.decode_audio(&mut probe) {
        let w = waveform_settings.length;
        let h = waveform_settings.height;

        let color = waveform_settings.fill_color.unwrap_or("red".to_owned());
        let color = parse_color(&color)?;

        draw_waveform(&samples, &output, &(w, h), &color);

        return Ok(());
    }

    Ok(())
}
