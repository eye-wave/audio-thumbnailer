use clap::ValueEnum;
use image::imageops::FilterType;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, ValueEnum)]
pub enum InterpolationType {
    Lanczos3,
    CatmullRom,
    Gaussian,
    Nearest,
    Triangle,
}

impl Default for InterpolationType {
    fn default() -> Self {
        Self::Triangle
    }
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
#[derive(Clone, ValueEnum)]
pub enum AspectRatio {
    Auto,
    Crop,
    Stretch,
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::Crop
    }
}
