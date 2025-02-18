use std::fs::File;
use std::path::Path;

pub fn decode_audio<P: AsRef<Path>>(path: &P) -> anyhow::Result<Vec<u8>> {
    let file = File::open(path)?;
    let (raw, _) = ogg_opus::decode::<_, 16000>(file)?;

    let samples = raw
        .iter()
        .map(|sample| (((*sample as i32 + 32768) * 255) / 65535) as u8)
        .collect();

    Ok(samples)
}
