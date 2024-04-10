use args::aspect_ratio::parse_aspect_ratio_arg;
use args::interpol::parse_interpol_arg;
use args::size::parse_dimensions;
use args::Args;
use clap::Parser;
use color::parse_hex_color;
use decoder::decode_audio_from_file;
use image::DynamicImage;
use images::wavetable::{draw_wavetable, is_wavetable};
use images::{
    cover_art::{extract_cover_art_data, load_and_resize, read_tags},
    waveform::draw_waveform,
};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
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
        let mut reader = BufReader::new(file);
        let mut test_chunk = vec![0u8; 256];

        reader.read_exact(&mut test_chunk).unwrap();
        reader.seek(SeekFrom::Start(0)).unwrap();

        let samples = decode_audio_from_file(reader);

        if let Some(samples) = samples {
            if is_wavetable(&test_chunk) {
                let wavetable_size = parse_dimensions(&args.wavetable_size);
                draw_wavetable(&samples, out_path, &wavetable_size, &color);
            } else {
                let waveform_size = parse_dimensions(&args.waveform_size);
                draw_waveform(&samples, out_path, &waveform_size, &color);
            }
        }
    }
}
