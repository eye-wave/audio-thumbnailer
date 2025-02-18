mod args;
mod enums;

pub use args::*;
pub use enums::*;

use std::path::PathBuf;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Default)]
pub struct Config {
    pub cover_settings: CoverSettings,
    pub waveform_settings: WaveformSettings,
    pub debug: DebugSettings,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct CoverSettings {
    pub no_cover: bool,
    pub size: u32,
    pub interpolation: InterpolationType,
    pub aspect_ratio: AspectRatio,
    pub waveform_on_fail: bool,
    pub image_format: ImageFormat,
}

impl Default for CoverSettings {
    fn default() -> Self {
        Self {
            no_cover: false,
            aspect_ratio: AspectRatio::default(),
            interpolation: InterpolationType::default(),
            waveform_on_fail: true,
            image_format: ImageFormat::Jpeg,
            size: 64,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct WaveformSettings {
    pub length: u32,
    pub height: u32,
    pub fill_color: Option<String>,
    pub bg_color: Option<String>,
}

impl Default for WaveformSettings {
    fn default() -> Self {
        Self {
            length: 200,
            height: 80,
            fill_color: None,
            bg_color: None,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Default, Clone)]
pub struct DebugSettings {
    pub enabled: bool,
    pub log_file: Option<PathBuf>,
}
