use std::fs::File;
use std::path::Path;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_codecs;
use symphonia::default::get_probe;
use symphonia_core::audio::SampleBuffer;
use symphonia_core::codecs::DecoderOptions;
use symphonia_core::formats::FormatOptions;
use symphonia_core::io::MediaSourceStreamOptions;
use symphonia_core::meta::MetadataOptions;
use symphonia_core::probe::Hint;

pub fn decode_audio(path: &Path) -> Option<Vec<f32>> {
    let codec_registry = get_codecs();
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

    let track = format_reader.format.default_track().unwrap();
    let mut decoder = codec_registry
        .make(&track.codec_params, &DecoderOptions::default())
        .unwrap();
    let mut samples: Vec<f32> = vec![];

    while let Ok(packet) = format_reader.format.next_packet() {
        let audio_buf = decoder.decode(&packet).unwrap();

        let spec = *audio_buf.spec();
        let duration = audio_buf.capacity() as u64;
        let mut sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));

        if let Some(buf) = &mut sample_buf {
            buf.copy_interleaved_ref(audio_buf);
            samples.extend(buf.samples());
        }
    }

    Some(samples)
}
