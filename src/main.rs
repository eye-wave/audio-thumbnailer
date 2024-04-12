use args::aspect_ratio::parse_aspect_ratio_arg;
use args::interpol::parse_interpol_arg;
use args::size::parse_dimensions;
use args::Args;
use clap::Parser;
use color::parse_hex_color;
use decoders::decode_audio;
use demuxers::get_cover_art;
use image::cover_art::load_and_resize;
use image::waveform::draw_waveform;
use std::path::Path;

mod args;
mod color;
mod decoders;
mod demuxers;
mod image;

fn main() {
    let args = Args::parse();
    let aspect_ratio = parse_aspect_ratio_arg(&args.aspect_ratio);
    let interpol = parse_interpol_arg(&args.interpol);
    let color = parse_hex_color(&args.color).unwrap();

    let in_path = Path::new(&args.input);
    let out_path = Path::new(&args.output);

    if !args.no_cover.unwrap_or(false) {
        if let Some(image_data) = get_cover_art(in_path) {
            if let Some(image) = load_and_resize(&image_data, args.size, &aspect_ratio, &interpol) {
                image
                    .save_with_format(out_path, ::image::ImageFormat::Png)
                    .unwrap();
                return;
            }
        }
    }

    let samples = decode_audio(in_path);
    if let Some(samples) = samples {
        let waveform_size = parse_dimensions(&args.waveform_size);
        draw_waveform(&samples, out_path, &waveform_size, &color);
    }
}
