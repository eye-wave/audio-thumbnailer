use args::aspect_ratio::parse_aspect_ratio_arg;
use args::interpol::parse_interpol_arg;
use args::size::parse_dimensions;
use args::Args;
use clap::Parser;
use color::parse_hex_color;
use decoder::decode_audio_from_file;
use image::DynamicImage;
use images::cover_art::{extract_cover_art_data, load_and_resize, read_tags};
use images::waveform::draw_waveform;
use std::fs::File;
use std::{io::BufReader, path::Path};

mod args;
mod color;
mod decoder;
mod images;

fn main() {
    let args = Args::parse();
    let aspect_ratio = parse_aspect_ratio_arg(&args.aspect_ratio);
    let interpol = parse_interpol_arg(&args.interpol);
    let color = parse_hex_color(&args.color).unwrap();

    let in_path = Path::new(&args.input);
    let out_path = Path::new(&args.output);

    let tags = read_tags(in_path);

    let cover_art: Option<DynamicImage> = if let Some(tags) = tags {
        if let Some(buffer) = extract_cover_art_data(&tags) {
            load_and_resize(buffer, args.size, &aspect_ratio, &interpol)
        } else {
            None
        }
    } else {
        None
    };

    if let Some(cover_art) = cover_art {
        cover_art
            .save_with_format(out_path, image::ImageFormat::Png)
            .unwrap();

        return;
    }

    if let Ok(file) = File::open(in_path) {
        let reader = BufReader::new(file);
        let samples = decode_audio_from_file(reader);

        if let Some(samples) = samples {
            let waveform_size = parse_dimensions(&args.waveform_size);
            draw_waveform(&samples, out_path, &waveform_size, &color);
        }
    }
}
