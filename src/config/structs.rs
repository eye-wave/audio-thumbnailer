use super::{AspectRatio, InterpolationType};
use clap::{ArgAction, Parser};

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

    /// Fill color for the waveform. If more than one color is specified waveform will be drawn with a gradient.
    #[clap(long, default_value = "red", num_args = ..=5)]
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
