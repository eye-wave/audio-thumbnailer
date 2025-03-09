use crate::{config::Config, decode::VisualData};
use color::{parse_color, Gradient};
use cover_art::load_and_resize;
use image::{DynamicImage, ImageFormat};
use midi::draw_midi;
use std::path::Path;
use waveform::draw_waveform;

pub mod color;

mod cover_art;
mod midi;
mod waveform;

impl VisualData {
    pub fn draw_and_save<P: AsRef<Path>>(&self, path: &P, config: &Config) -> anyhow::Result<()> {
        match self {
            Self::Samples(samples) => {
                let w = config.waveform.length;
                let h = config.waveform.height;

                let gradient = Gradient::new(&config.waveform.fill_color);
                let bg_color = parse_color(&config.waveform.bg_color)?;

                draw_waveform(samples, &path, (w, h), &gradient, &bg_color)?;

                Ok(())
            }
            Self::Pixels(image_data) => {
                let image = load_and_resize(image_data, &config.cover)?;
                write_image(image, &path)?;

                Ok(())
            }
            Self::Midi(midi) => {
                let w = config.waveform.length;
                let h = config.waveform.height;

                let color = parse_color(&config.waveform.fill_color[0])?;
                let bg_color = parse_color(&config.waveform.bg_color)?;

                draw_midi(midi, &path, (w, h), &color, &bg_color)?;

                Ok(())
            }
        }
    }
}

pub fn write_image<P: AsRef<Path>>(image: DynamicImage, path: &P) -> anyhow::Result<()> {
    let format = path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| match ext {
            "jpg" => Some(ImageFormat::Jpeg),
            "jpeg" => Some(ImageFormat::Jpeg),
            "png" => Some(ImageFormat::Png),
            _ => None,
        })
        .unwrap_or(image::ImageFormat::Jpeg);

    Ok(image.save_with_format(path, format)?)
}
