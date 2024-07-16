use std::path::Path;
use symphonia::{create_probe, decode_audio, get_cover_art};

use crate::{config::Config, Error, Result};

mod symphonia;

#[derive(Debug)]
pub enum VisualData {
    AudioData(Vec<u8>),
    ImageData(Vec<u8>),
}

pub fn decode_visual_data<P: AsRef<Path>>(path: &P, config: &Config) -> Result<VisualData> {
    let ext = path.as_ref().extension().and_then(|ext| ext.to_str());
    match ext {
        Some(ext) => match ext {
            "m3u" => Ok(VisualData::ImageData(vec![])),
            _ => {
                let mut probe = create_probe(&path).expect("Failed to create audio decoder");
                if !config.cover_settings.no_cover {
                    if let Some(image_data) = get_cover_art(&mut probe) {
                        return Ok(VisualData::ImageData(image_data));
                    }
                }

                if let Some(samples) = decode_audio(&mut probe) {
                    return Ok(VisualData::AudioData(samples));
                }

                Err(Error::Custom("Failed to decode visual data".to_string()))
            }
        },
        None => Err(Error::Custom("Unsupported format".to_string())),
    }
}
