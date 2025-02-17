use super::{AspectRatio, Config, InterpolationType};
use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
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

impl Args {
    pub fn apply_to_config(&self, config: &mut Config) {
        if let Some(v) = self.no_cover {
            config.cover_settings.no_cover = v
        }

        if let Some(v) = self.size {
            config.cover_settings.size = v
        }

        if let Some(v) = self.interpolation.as_ref() {
            config.cover_settings.interpolation = v.clone()
        }

        if let Some(v) = self.aspect_ratio.as_ref() {
            config.cover_settings.aspect_ratio = v.clone()
        }

        if let Some(v) = self.waveform_length {
            config.waveform_settings.length = v
        }

        if let Some(v) = self.waveform_height {
            config.waveform_settings.height = v
        }

        config
            .waveform_settings
            .fill_color
            .clone_from(&self.waveform_fill_color);

        config
            .waveform_settings
            .bg_color
            .clone_from(&self.waveform_bg_color);

        if let Some(v) = self.waveform_on_fail {
            config.cover_settings.waveform_on_fail = v
        }
    }
}
