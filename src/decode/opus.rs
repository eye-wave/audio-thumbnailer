use std::fs::File;
use std::path::Path;

pub fn decode_audio<P: AsRef<Path>>(path: &P) -> anyhow::Result<Vec<f32>> {
    let file = File::open(path)?;
    let (raw, _) = ogg_opus::decode::<_, 16000>(file)?;

    let samples = raw
        .iter()
        .map(|sample| *sample as f32 / i16::MAX as f32)
        .collect();

    Ok(samples)
}
