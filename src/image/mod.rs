use crate::{config::Config, decode::VisualData};
use cover_art::load_and_resize;
use image::{DynamicImage, ImageFormat, Rgb};
use midi::draw_midi;
use std::path::Path;
use waveform::draw_waveform;

mod cover_art;
mod midi;
mod waveform;

fn parse_color(color: &str) -> anyhow::Result<Rgb<u8>> {
    let parsed = csscolorparser::parse(color)?;
    let rgba = parsed.to_rgba8();
    let rgb = Rgb([rgba[0], rgba[1], rgba[2]]);

    Ok(rgb)
}

impl VisualData {
    pub fn draw_and_save<P: AsRef<Path>>(&self, path: &P, config: &Config) -> anyhow::Result<()> {
        match self {
            Self::Samples(samples) => {
                let w = config.waveform_settings.length;
                let h = config.waveform_settings.height;

                let bg_color = config
                    .waveform_settings
                    .bg_color
                    .clone()
                    .and_then(|c| parse_color(&c).ok())
                    .unwrap_or(Rgb([0; 3]));

                draw_waveform(samples, &path, (w, h), &bg_color)?;

                Ok(())
            }
            Self::Pixels(image_data) => {
                let image = load_and_resize(image_data, &config.cover_settings)?;
                write_image(image, &path, config)?;

                Ok(())
            }
            Self::Midi(midi) => {
                let w = config.waveform_settings.length;
                let h = config.waveform_settings.height;

                let color = config
                    .waveform_settings
                    .fill_color
                    .clone()
                    .and_then(|c| parse_color(&c).ok())
                    .unwrap_or(Rgb([0xff, 0, 0]));

                let bg_color = config
                    .waveform_settings
                    .bg_color
                    .clone()
                    .and_then(|c| parse_color(&c).ok())
                    .unwrap_or(Rgb([0; 3]));

                draw_midi(midi, &path, (w, h), &color, &bg_color)?;

                Ok(())
            }
        }
    }
}

pub fn write_image<P: AsRef<Path>>(
    image: DynamicImage,
    path: &P,
    config: &Config,
) -> anyhow::Result<()> {
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
        .unwrap_or(config.cover_settings.image_format.to_image_enum());

    Ok(image.save_with_format(path, format)?)
}
