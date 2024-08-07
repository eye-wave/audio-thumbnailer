use crate::{config::Config, decode::VisualData, Result};
use cover_art::load_and_resize;
use csscolorparser::Color;
use image::{DynamicImage, ImageFormat};
use plotters::style::RGBAColor;
use std::path::Path;
use waveform::draw_waveform;

pub mod cover_art;
pub mod waveform;

impl VisualData {
    pub fn draw_and_save<P: AsRef<Path>>(&self, path: &P, config: &Config) -> Result<()> {
        match self {
            Self::Samples(samples) => {
                let w = config.waveform_settings.length;
                let h = config.waveform_settings.height;

                let color = config
                    .waveform_settings
                    .fill_color
                    .clone()
                    .unwrap_or("red".to_string());

                let color = parse_color(&color)?;

                draw_waveform(samples, &path, &(w, h), &color);

                Ok(())
            }
            Self::Pixels(image_data) => {
                let image = load_and_resize(image_data, &config.cover_settings)?;
                write_image(image, &path, &config)?;

                Ok(())
            }
        }
    }
}

pub fn write_image<P: AsRef<Path>>(image: DynamicImage, path: &P, config: &Config) -> Result<()> {
    let format =
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext {
                "jpg" => Some(ImageFormat::Jpeg),
                "jpeg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => None
            })
            .unwrap_or(config.cover_settings.image_format.to_image_enum());

    Ok(image.save_with_format(path, format)?)
}

pub fn parse_color(color: &str) -> Result<RGBAColor> {
    let [r, g, b, a] = color.parse::<Color>()?.to_rgba8();
    let alpha = (a as f64) / (u8::MAX as f64);

    Ok(RGBAColor(r, g, b, alpha))
}
