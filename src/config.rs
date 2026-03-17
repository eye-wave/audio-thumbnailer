use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};
use image::imageops::FilterType;

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

    #[clap(flatten)]
    pub config: Config,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, ValueEnum, Default)]
pub enum InterpolationType {
    Lanczos3,
    CatmullRom,
    Gaussian,
    Nearest,
    #[default]
    Triangle,
}

impl From<InterpolationType> for FilterType {
    fn from(value: InterpolationType) -> Self {
        match value {
            InterpolationType::CatmullRom => FilterType::CatmullRom,
            InterpolationType::Gaussian => FilterType::Gaussian,
            InterpolationType::Lanczos3 => FilterType::Lanczos3,
            InterpolationType::Nearest => FilterType::Nearest,
            InterpolationType::Triangle => FilterType::Triangle,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, ValueEnum, Default)]
pub enum AspectRatio {
    Auto,
    #[default]
    Crop,
    Stretch,
}

#[derive(Parser)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Config {
    #[clap(flatten)]
    pub waveform: WaveformConfig,

    #[clap(flatten)]
    pub cover: CoverConfig,
}

#[derive(Parser, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct WaveformConfig {
    /// Background color for the waveform.
    #[clap(long, default_value = "black")]
    pub bg_color: String,

    /// Fill color for the waveform. If more than one color is specified waveform will be drawn with a vertical gradient.
    /// Up to 20 colors, evenly spaced.
    ///
    /// example: --fill-color red blue
    #[clap(long, default_value = "red", num_args = ..=20)]
    pub fill_color: Vec<String>,

    /// Length of the waveform thumbnail in pixels
    #[clap(long, default_value_t = 256)]
    pub length: u32,

    /// Height of the waveform thumbnail in pixels
    #[clap(long, default_value_t = 96)]
    pub height: u32,

    /// Generate waveform if album cover is not found.
    #[clap(long, default_value_t = false, action = ArgAction::SetTrue)]
    pub waveform_on_fail: bool,
}

#[derive(Parser, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CoverConfig {
    /// Do not look for album covers, just draw the waveform.
    #[clap(long, default_value_t = false, action = ArgAction::SetTrue)]
    pub no_cover: bool,

    /// Both width and weight for a album cover
    #[clap(long, default_value_t = 128)]
    pub size: u32,

    #[clap(long, default_value = "crop")]
    pub aspect_ratio: AspectRatio,

    #[clap(long, default_value = "triangle")]
    pub interpolation: InterpolationType,
}
