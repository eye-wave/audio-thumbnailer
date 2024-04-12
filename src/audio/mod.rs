mod symphonia;

pub struct AudioDecoder {
    current_file: Option<String>,
}

impl AudioDecoder {
    pub fn new() -> AudioDecoder {
        AudioDecoder { current_file: None }
    }
}

#[cfg(test)]
mod tests {
    use super::AudioDecoder;
    use rstest::rstest;
    use std::path::Path;

    #[cfg(test)]
    #[rstest(
        file,
        case("test/fish.aac"),
        case("test/fish.caf"),
        case("test/fish.flac"),
        case("test/fish.m4a"),
        case("test/fish.mka"),
        case("test/fish.mp3"),
        case("test/fish.ogg"),
        case("test/fish.opus"),
        case("test/fish.wav"),
        case("test/fish.wv")
    )]
    fn test_decode_audio(file: &str) {
        let path = Path::new(file);
        println!("Testing {}", file);

        let mut audio_decoder = AudioDecoder::new();
        let mut probe = audio_decoder.create_probe(path).unwrap();

        if let Some(samples) = audio_decoder.decode_audio(&mut probe) {
            assert!(!samples.is_empty());
            return;
        }

        panic!("Failed to decode audio file: {:?}", path);
    }
}
