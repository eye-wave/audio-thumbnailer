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
    #[clap(long, default_value = "black")]
    pub bg_color: String,

    #[clap(long, default_value = "red")]
    pub fill_color: Vec<String>,

    #[clap(long, default_value_t = 256)]
    pub length: u32,

    #[clap(long, default_value_t = 96)]
    pub height: u32,

    #[clap(long, default_value_t = false, action = ArgAction::SetTrue)]
    pub waveform_on_fail: bool,
}

#[derive(Parser, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CoverConfig {
    #[clap(long, default_value_t = false, action = ArgAction::SetTrue)]
    pub no_cover: bool,

    #[clap(long, default_value_t = 128)]
    pub size: u32,

    #[clap(long, default_value = "crop")]
    pub aspect_ratio: AspectRatio,

    #[clap(long, default_value = "triangle")]
    pub interpolation: InterpolationType,
}
