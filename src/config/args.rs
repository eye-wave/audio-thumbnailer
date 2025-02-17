use super::{AspectRatio, Config, InterpolationType};
use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///input file name
    #[arg(short, long)]
    pub input: PathBuf,

    ///output file name of the generated thumbnail
    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(long, action=ArgAction::SetTrue)]
    pub no_cover: Option<bool>,

    /// size of the generated cover art
    /// for example size of 64 will generate as 64x64 cover
    /// when using aspect_ratio auto, size will be used as a smaller side of the rectangle
    #[arg(long, short, verbatim_doc_comment)]
    pub size: Option<u32>,

    /// interpolation algorithm used for scaling the image
    #[arg(long)]
    pub interpolation: Option<InterpolationType>,

    /// aspect ratio used for image generation
    #[arg(long, short)]
    pub aspect_ratio: Option<AspectRatio>,

    #[arg(long)]
    pub waveform_length: Option<u32>,

    #[arg(long)]
    pub waveform_height: Option<u32>,

    #[arg(long)]
    pub waveform_fill_color: Option<String>,

    #[arg(long)]
    pub waveform_bg_color: Option<String>,

    /// generate a waveform if getting a thumbnail fails
    #[arg(long)]
    pub waveform_on_fail: Option<bool>,
}

macro_rules! apply_config {
    ($left:expr,$right:expr) => {
        if let Some(v) = $left.as_ref() {
            $right = v.clone();
        }
    };
}

macro_rules! copy_to_config {
    ($left:expr,$right:expr) => {
        $left.clone_from(&$right);
    };
}

impl Args {
    pub fn apply_to_config(&self, config: &mut Config) {
        apply_config!(self.no_cover, config.cover_settings.no_cover);
        apply_config!(self.size, config.cover_settings.size);
        apply_config!(self.interpolation, config.cover_settings.interpolation);
        apply_config!(self.aspect_ratio, config.cover_settings.aspect_ratio);
        apply_config!(self.waveform_length, config.waveform_settings.length);
        apply_config!(self.waveform_height, config.waveform_settings.height);
        apply_config!(
            self.waveform_on_fail,
            config.cover_settings.waveform_on_fail
        );

        copy_to_config!(
            config.waveform_settings.fill_color,
            self.waveform_fill_color
        );
        copy_to_config!(config.waveform_settings.bg_color, self.waveform_bg_color);
    }
}
