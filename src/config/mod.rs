mod args;
mod file;

use parsers::{aspect_ratio::AspectRatio, interpol::InterpolationType};
use std::path::PathBuf;

#[derive(Debug,Deserialize)]
pub struct Config {
    cover_settings: CoverSettings,
    audio_analyzer_settings: AudioAnalyzerSettings,
    waveform_settings: WaveformSettings,
    thumbnail_settings: ThumbnailsSettings,
    debug: Option<DebugSettings>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cover_settings: CoverSettings::default(),
            audio_analyzer_settings: AudioAnalyzerSettings::default(),
            thumbnail_settings: ThumbnailsSettings::default(),
            waveform_settings: WaveformSettings::default(),
            debug: None,
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct CoverSettings {
    size: u16,
    quality: u8,
    interpolation: InterpolationType,
    aspect_ratio: AspectRatio
}

impl Default for CoverSettings {
    fn default() -> Self {
        Self {
            aspect_ratio: AspectRatio::default(),
            interpolation: InterpolationType::default(),
            quality: 80,
            size: 64
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct AudioAnalyzerSettings {
    fft_enabled: bool,
    fft_size: u64,
}

impl Default for AudioAnalyzerSettings {
    fn default() -> Self {
        Self {
            fft_enabled: true,
            fft_size: 2048
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct WaveformSettings {
    length: u16,
    height: u16,
    sample_style: String,
    sample_smoothing: f32,
    fill_type: String,
    fill_colors: Option<Vec<String>>,
    fill_color: Option<String>,
    fill_texture: Option<PathBuf>,
    stroke_type: String,
    stroke_texture: Option<String>,
    stroke_color: Option<String>,
    stroke_colors: Option<String>,
    stroke_width: Option<u8>,
}

impl Default for WaveformSettings {
    fn default() -> Self {
        Self {
            length: 200,
            height: 80,
            sample_style: "bars".to_string(),
            sample_smoothing: 1.0,
            fill_type: "solid".to_string(),
            fill_colors: None,
            fill_color: None,
            fill_texture: None,
            stroke_type: "solid".to_string(),
            stroke_texture: None,
            stroke_color: None,
            stroke_colors: None,
            stroke_width: None,
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct ThumbnailsSettings {
    waveform_on_fail: bool,
    thumbnail_format: String,
    overlay: String,
}

impl Default for ThumbnailsSettings {
    fn default() -> Self {
        Self {
            waveform_on_fail: true,
            thumbnail_format: "jpeg".to_string(),
            overlay: "none".to_string(),
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct DebugSettings {
    enabled: bool,
    log_file: Option<PathBuf>
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            log_file: None
        }
    }
}

pub mod parsers;
pub use args::*;
pub use file::*;
use serde_derive::Deserialize;
