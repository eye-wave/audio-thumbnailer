mod args;
mod enums;
mod file;

pub use args::*;
pub use enums::*;

use std::path::PathBuf;

#[cfg(feature = "config_file")]
#[derive(serde_derive::Deserialize)]
pub struct ConfigDeserialize {
    pub cover_settings: Option<CoverSettings>,

    #[cfg(feature = "colored_waveform")]
    pub audio_analyzer_settings: Option<AudioAnalyzerSettings>,

    #[cfg(feature = "3d_wavetables")]
    pub wavetable_settings: Option<WavetableSettings>,

    pub waveform_settings: Option<WaveformSettings>,
    pub thumbnail_settings: Option<ThumbnailsSettings>,
    pub debug: Option<DebugSettings>,
}

#[derive(Debug, Default)]
pub struct Config {
    pub cover_settings: CoverSettings,

    #[cfg(feature = "colored_waveform")]
    pub audio_analyzer_settings: AudioAnalyzerSettings,

    #[cfg(feature = "3d_wavetables")]
    pub wavetable_settings: WavetableSettings,

    pub waveform_settings: WaveformSettings,
    pub thumbnail_settings: ThumbnailsSettings,
    pub debug: DebugSettings,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "config_file", derive(serde_derive::Deserialize))]
pub struct CoverSettings {
    pub no_cover: bool,
    pub size: u32,
    pub interpolation: InterpolationType,
    pub aspect_ratio: AspectRatio,
}

impl Default for CoverSettings {
    fn default() -> Self {
        Self {
            no_cover: false,
            aspect_ratio: AspectRatio::default(),
            interpolation: InterpolationType::default(),
            size: 64,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg(feature = "3d_wavetables")]
#[cfg_attr(
    all(feature = "config_file", feature = "colored_waveform"),
    derive(serde_derive::Deserialize)
)]
pub struct AudioAnalyzerSettings {
    pub fft_enabled: bool,
    pub fft_size: u64,
}

#[cfg(feature = "3d_wavetables")]
impl Default for AudioAnalyzerSettings {
    fn default() -> Self {
        Self {
            fft_enabled: true,
            fft_size: 2048,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg(feature = "3d_wavetables")]
#[cfg_attr(
    all(feature = "config_file", feature = "3d_wavetables"),
    derive(serde_derive::Deserialize)
)]
pub struct WavetableSettings {
    horizontal_rotation: f32,
    vertical_rotation: f32,
    width: f32,
    height: f32,
    y_offset: f32,
}

#[cfg(feature = "3d_wavetables")]
impl Default for WavetableSettings {
    fn default() -> Self {
        Self {
            horizontal_rotation: -0.24,
            vertical_rotation: 1.22,
            width: 0.72,
            height: 0.1,
            y_offset: 0.05,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "config_file", derive(serde_derive::Deserialize))]
pub struct WaveformSettings {
    pub length: u32,
    pub height: u32,
    // pub sample_style: String,
    // pub sample_smoothing: f32,
    // pub fill_type: String,
    // pub fill_colors: Option<Vec<String>>,
    pub fill_color: Option<String>,
    // pub fill_texture: Option<PathBuf>,
    // pub stroke_type: String,
    // pub stroke_texture: Option<String>,
    // pub stroke_color: Option<String>,
    // pub stroke_colors: Option<String>,
    // pub stroke_width: Option<u8>,
}

impl Default for WaveformSettings {
    fn default() -> Self {
        Self {
            length: 200,
            height: 80,
            // sample_style: "bars".to_string(),
            // sample_smoothing: 1.0,
            // fill_type: "solid".to_string(),
            // fill_colors: None,
            fill_color: None,
            // fill_texture: None,
            // stroke_type: "solid".to_string(),
            // stroke_texture: None,
            // stroke_color: None,
            // stroke_colors: None,
            // stroke_width: None,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "config_file", derive(serde_derive::Deserialize))]
pub struct ThumbnailsSettings {
    pub waveform_on_fail: bool,
    // pub overlay: String,
}

impl Default for ThumbnailsSettings {
    fn default() -> Self {
        Self {
            waveform_on_fail: true,
            // overlay: "none".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "config_file", derive(serde_derive::Deserialize))]
pub struct DebugSettings {
    pub enabled: bool,
    pub log_file: Option<PathBuf>,
}
