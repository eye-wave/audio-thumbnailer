use crate::config::Config;
use anyhow::anyhow;
use midi::{decode_midi, MidiTracks};
use std::path::Path;
use symphonia::{create_probe, decode_audio, get_cover_art};

pub mod midi;
mod opus;
mod symphonia;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum VisualData {
    Samples(Vec<u8>),
    Midi(MidiTracks),
    Pixels(Vec<u8>),
}

pub fn decode_visual_data<P: AsRef<Path>>(path: &P, config: &Config) -> anyhow::Result<VisualData> {
    let ext = path.as_ref().extension().and_then(|ext| ext.to_str());
    match ext {
        Some(ext) => match ext {
            "m3u" => {
                unimplemented!()
            }
            "midi" | "mid" | "rmi" => {
                let tracks = decode_midi(path)?;
                Ok(VisualData::Midi(tracks))
            }
            "wv" => {
                unimplemented!()
            }
            "opus" => {
                let samples = opus::decode_audio(path)?;
                Ok(VisualData::Samples(samples))
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

                Err(anyhow!("Failed to decode visual data."))
            }
        },
        None => Err(anyhow!("Unsupported format.")),
    }
}
