use crate::{Error, Result};
use csscolorparser::Color;
use image::{DynamicImage, ImageFormat};
use plotters::style::RGBAColor;
use std::path::Path;

pub mod cover_art;
pub mod waveform;

pub fn write_image<P: AsRef<Path>>(image: DynamicImage, path: &P) -> Result<()> {
    let format =
        path.as_ref()
            .extension()
            .and_then(|extension| match extension.to_str().unwrap() {
                "jpg" => Some(ImageFormat::Jpeg),
                "jpeg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => None,
            });

    match format {
        Some(format) => Ok(image.save_with_format(path, format)?),
        _ => Err(Error::Custom("Invalid image format".to_string())),
    }
}

pub fn parse_color(color: &str) -> Result<RGBAColor> {
    let [r, g, b, a] = color.parse::<Color>()?.to_rgba8();
    let alpha = (a as f64) / (u8::MAX as f64);

    Ok(RGBAColor(r, g, b, alpha))
}
