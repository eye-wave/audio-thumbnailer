use std::fs::File;
use std::path::Path;

pub fn decode_audio<P: AsRef<Path>>(path: &P) -> Result<Vec<u8>, ogg_opus::Error> {
    let file = File::open(path)?;
    let (raw, _) = ogg_opus::decode::<_, 16000>(file)?;

    let mut samples = Vec::with_capacity(raw.len());

    const I16_MAX_F32: f32 = i16::MAX as f32;

    for sample in raw.iter() {
        let normalized = *sample as f32 / I16_MAX_F32;
        let u8_sample = ((normalized + 1.0) * 127.5) as u8;
        samples.push(u8_sample);
    }

    Ok(samples)
}
