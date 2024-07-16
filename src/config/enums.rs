use image::imageops::FilterType;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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

impl InterpolationType {
    pub fn to_filter_type(&self) -> FilterType {
        match self {
            Self::CatmullRom => FilterType::CatmullRom,
            Self::Gaussian => FilterType::Gaussian,
            Self::Lanczos3 => FilterType::Lanczos3,
            Self::Nearest => FilterType::Nearest,
            Self::Triangle => FilterType::Triangle,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
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
