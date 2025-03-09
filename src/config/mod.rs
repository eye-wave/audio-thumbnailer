mod enums;

pub use enums::{AspectRatio, InterpolationType};

use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[command(author, version, about, long_about = None)]
pub struct Config {
    ///input file name
    #[arg(short, long)]
    pub input: PathBuf,

    ///output file name of the generated thumbnail
    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(long, action=ArgAction::SetTrue)]
    no_cover: Option<bool>,

    /// size of the generated cover art
    /// for example size of 64 will generate as 64x64 cover
    /// when using aspect_ratio auto, size will be used as a smaller side of the rectangle
    #[arg(long, short, verbatim_doc_comment)]
    cover_size: Option<u32>,

    /// interpolation algorithm used for scaling the image
    #[arg(long)]
    interpolation: Option<InterpolationType>,

    /// aspect ratio used for image generation
    #[arg(long, short)]
    aspect_ratio: Option<AspectRatio>,

    #[arg(long)]
    waveform_length: Option<u32>,

    #[arg(long)]
    waveform_height: Option<u32>,

    #[arg(long)]
    waveform_fill_color: Option<String>,

    #[arg(long)]
    waveform_bg_color: Option<String>,

    /// generate a waveform if getting a thumbnail fails
    #[arg(long)]
    waveform_on_fail: Option<bool>,
}

impl Config {
    pub fn no_cover(&self) -> bool {
        self.no_cover.unwrap_or(false)
    }

    pub fn aspect_ratio(&self) -> AspectRatio {
        self.aspect_ratio.clone().unwrap_or_default()
    }

    pub fn interpolation(&self) -> InterpolationType {
        self.interpolation.clone().unwrap_or_default()
    }

    pub fn cover_size(&self) -> u32 {
        self.cover_size.unwrap_or(128)
    }

    pub fn waveform_length(&self) -> u32 {
        self.waveform_length.unwrap_or(256)
    }

    pub fn waveform_height(&self) -> u32 {
        self.waveform_height.unwrap_or(120)
    }

    pub fn waveform_fill_color(&self) -> String {
        self.waveform_fill_color.clone().unwrap_or("red".to_owned())
    }

    pub fn waveform_bg_color(&self) -> String {
        self.waveform_bg_color.clone().unwrap_or("#000".to_owned())
    }
}
