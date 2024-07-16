use std::path::Path;
use symphonia::{create_probe, decode_audio, get_cover_art};

use crate::{config::Config, Error, Result};

mod symphonia;

#[derive(Debug)]
pub enum VisualData {
    Samples(Vec<u8>),
    Pixels(Vec<u8>),
}

pub fn decode_visual_data<P: AsRef<Path>>(path: &P, config: &Config) -> Result<VisualData> {
    let ext = path.as_ref().extension().and_then(|ext| ext.to_str());
    match ext {
        Some(ext) => match ext {
            "m3u" => {
                unimplemented!()
            }
            "midi" | "mid" | "rmi" => {
                unimplemented!()
            }
            "wv" => {
                unimplemented!()
            }
            "opus" => {
                unimplemented!()
            }
            _ => {
                let mut probe = create_probe(&path).expect("Failed to create audio decoder");
                if !config.cover_settings.no_cover {
                    if let Some(image_data) = get_cover_art(&mut probe) {
                        return Ok(VisualData::Pixels(image_data));
                    }
                }

                if let Some(samples) = decode_audio(&mut probe) {
                    return Ok(VisualData::Samples(samples));
                }

                Err(Error::Custom("Failed to decode visual data"))
            }
        },
        None => Err(Error::Custom("Unsupported format")),
    }
}
