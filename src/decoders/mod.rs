use std::path::Path;

mod symphonia;

pub fn decode_audio(path: &Path) -> Option<Vec<f32>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("opus") => unimplemented!(),
        Some("wv") => unimplemented!(),
        _ => symphonia::decode_audio(path),
    }
}

#[cfg(test)]
mod tests {
    use super::decode_audio;
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

        if let Some(samples) = decode_audio(path) {
            assert!(!samples.is_empty());
        } else {
            panic!("Failed to decode audio file: {:?}", path);
        }
    }
}
