use crate::{config::Config, decode::midi::MidiDecodeError};

use audio_formats::SYMPHONIA_FORMATS;
use midi::{decode_midi, MidiTracks};
use std::{process::Command, rc::Rc};
use symph::{create_probe, decode_audio, get_cover_art};

pub mod midi;
mod opus;
mod symph;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum VisualData {
    Samples(Vec<u8>),
    Midi(MidiTracks),
    Pixels(Vec<u8>),
}

#[derive(Debug, thiserror::Error)]
pub enum MimeError {
    #[error("Unknown format for path: {0}.")]
    UnknownFormat(Rc<str>),

    #[error("{0}")]
    Io(#[from] std::io::Error),
}

fn get_mime_type(path: &str) -> Result<String, MimeError> {
    let output = Command::new("xdg-mime")
        .args(["query", "filetype", path])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(MimeError::UnknownFormat(Rc::from(path)))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecoderError {
    #[error("Failed to decode visual data.")]
    DecodeFailed,

    #[error("Unsupported format {0}.")]
    UnsupportedFormat(Rc<str>),

    #[error("{0}")]
    Mime(#[from] MimeError),

    #[error("{0}")]
    Midi(#[from] MidiDecodeError),

    #[error("{0}")]
    Symphonia(#[from] symphonia::core::errors::Error),
}

pub fn decode_visual_data(path: &str, config: &Config) -> Result<VisualData, DecoderError> {
    let mime = get_mime_type(path)?;

    if path.ends_with(".opus") {
        if let Ok(samples) = opus::decode_audio(&path) {
            return Ok(VisualData::Samples(samples));
        }
    }

    match mime.as_ref() {
        "audio/x-mpegurl" => {
            unimplemented!()
        }
        "audio/midi" => {
            let tracks = decode_midi(&path)?;
            Ok(VisualData::Midi(tracks))
        }
        mime if SYMPHONIA_FORMATS.contains(&mime) => {
            let mut probe = create_probe(&path)?;
            if !config.cover.no_cover {
                if let Some(image_data) = get_cover_art(&mut probe) {
                    return Ok(VisualData::Pixels(image_data.to_vec()));
                }
            }

            if let Some(samples) = decode_audio(&mut probe, 4) {
                return Ok(VisualData::Samples(samples));
            }

            Err(DecoderError::DecodeFailed)
        }
        mime => Err(DecoderError::UnsupportedFormat(Rc::from(mime))),
    }
}
