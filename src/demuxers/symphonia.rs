use std::fs::File;
use std::path::Path;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;
use symphonia_core::formats::FormatOptions;
use symphonia_core::io::MediaSourceStreamOptions;
use symphonia_core::meta::MetadataOptions;
use symphonia_core::probe::Hint;

pub fn get_cover_art(path: &Path) -> Option<Vec<u8>> {
    // let codec_registry = get_codecs();
    let probe = get_probe();

    let file = File::open(path).unwrap();
    let media_source = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let mut format_reader = probe
        .format(
            &Hint::new(),
            media_source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .expect("unsupported format");

    if let Some(metadata) = format_reader.metadata.get() {
        if let Some(current) = metadata.current() {
            if let Some(visual) = current.visuals().first() {
                return Some(Vec::from(visual.data.clone()));
            }
        }
    }

    None
}
