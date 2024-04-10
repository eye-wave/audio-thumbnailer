mod images;
use std::path::Path;

use aspect_ratio::parse_aspect_ratio_arg;
use clap::Parser;
use image::DynamicImage;
use images::cover_art::{extract_cover_art_data, load_and_resize, read_tags};
use interpol::parse_interpol_arg;

mod aspect_ratio;
mod interpol;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file name
    #[arg(short, long)]
    input: String,

    /// output file name of the generated thumbnail
    #[arg(short, long)]
    output: String,

    /// quality of the generated image ( 0 = the worst, 10 = lossless )
    #[arg(short, long)]
    quality: Option<u8>,

    /// how to treat aspect ratio
    /// "crop" - crops edges to get 1:1 aspect ratio
    /// "stretch" - stretches image to 1:1
    /// "auto" - leaves as is
    /// default = "auto"
    #[arg(short, long, verbatim_doc_comment)]
    aspect_ratio: Option<String>,

    ///thumbnail size
    #[arg(short, long, default_value_t = 64)]
    size: u8,

    /// what interpolation algorithm to use
    /// "lanczos3", "gaussian", "nearest", "triangle",
    /// default = "lanczos3"
    #[arg(short = 'p', long, verbatim_doc_comment)]
    interpol: Option<String>,
}

fn main() {
    let args = Args::parse();
    let aspect_ratio = parse_aspect_ratio_arg(&args.aspect_ratio);
    let interpol = parse_interpol_arg(&args.interpol);

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
    }
}
