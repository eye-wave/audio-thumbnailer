use clap::ValueEnum;
use image::imageops::FilterType;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize, ValueEnum)]
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

#[derive(Debug, Clone, Deserialize, ValueEnum)]
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

#[derive(Debug, Clone, Deserialize, ValueEnum)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Webp,
}

impl ImageFormat {
    pub fn to_image_enum(&self) -> image::ImageFormat {
        match self {
            Self::Jpeg => image::ImageFormat::Jpeg,
            Self::Png => image::ImageFormat::Png,
            Self::Webp => image::ImageFormat::WebP,
        }
    }
}
