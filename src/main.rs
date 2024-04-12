use args::aspect_ratio::parse_aspect_ratio_arg;
use args::interpol::parse_interpol_arg;
use args::size::parse_dimensions;
use args::Args;
use audio::AudioDecoder;
use clap::Parser;
use color::parse_hex_color;
use image::cover_art::load_and_resize;
use image::waveform::draw_waveform;
use std::path::Path;

mod args;
mod audio;
mod color;
mod image;

fn main() {
    let args = Args::parse();
    let aspect_ratio = parse_aspect_ratio_arg(&args.aspect_ratio);
    let interpol = parse_interpol_arg(&args.interpol);
    let color = parse_hex_color(&args.color).unwrap();

    let in_path = Path::new(&args.input);
    let out_path = Path::new(&args.output);

    let mut audio_decoder = AudioDecoder::new();
    let mut probe = audio_decoder.create_probe(in_path).unwrap();

    if !args.no_cover.unwrap_or(false) {
        if let Some(image_data) = audio_decoder.get_cover_art(&mut probe) {
            if let Some(image) = load_and_resize(&image_data, args.size, &aspect_ratio, &interpol) {
                let format = ::image::ImageFormat::Png;
                if image.save_with_format(out_path, format).is_ok() {
                    return;
                }
            }
        }
    }

    let samples = audio_decoder.decode_audio(&mut probe);
    if let Some(samples) = samples {
        let waveform_size = parse_dimensions(&args.waveform_size);
        draw_waveform(&samples, out_path, &waveform_size, &color);
    }
}
