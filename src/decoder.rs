use rodio::{source::Source, Decoder};
use std::{fs::File, io::BufReader};

pub fn decode_audio_from_file(reader: BufReader<File>) -> Option<Vec<f32>> {
    match Decoder::new(reader) {
        Ok(buffer) => {
            let samples = buffer.convert_samples::<f32>();
            let samples: Vec<f32> = samples.collect();
            Some(samples)
        }
        Err(err) => {
            println!("{:?}", err);
            None
        }
    }
}
