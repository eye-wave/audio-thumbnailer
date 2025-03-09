use crate::config::Config;
use anyhow::anyhow;
use midi::{decode_midi, MidiTracks};
use std::process::Command;
use symphonia::{create_probe, decode_audio, get_cover_art};

pub mod midi;
mod opus;
mod symphonia;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum VisualData {
    Samples(Vec<f32>),
    Midi(MidiTracks),
    Pixels(Vec<u8>),
}

fn get_mime_type(path: &str) -> anyhow::Result<String> {
    let output = Command::new("xdg-mime")
        .args(["query", "filetype", path])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(anyhow!(""))
    }
}

pub fn decode_visual_data(path: &str, config: &Config) -> anyhow::Result<VisualData> {
    let mime = get_mime_type(path)?;

    match mime.as_ref() {
        "audio/x-mpegurl" => {
            unimplemented!()
        }
        "audio/midi" => {
            let tracks = decode_midi(&path)?;
            Ok(VisualData::Midi(tracks))
        }
        "audio/x-wavpack" => {
            unimplemented!()
        }
        "audio/x-opus+ogg" => {
            let samples = opus::decode_audio(&path)?;
            Ok(VisualData::Samples(samples))
        }
        "audio/aac" | "audio/flac" | "audio/mp2" | "audio/mp4" | "audio/mpeg" | "audio/x-aiff"
        | "audio/x-caf" | "audio/x-vorbis+ogg" | "audio/x-wav" => {
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
        _ => Err(anyhow!("Unsupported format.")),
    }
}
