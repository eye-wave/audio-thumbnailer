use std::fs::File;
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::io::MediaSourceStreamOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::probe::ProbeResult;
use symphonia::default::get_codecs;
use symphonia::default::get_probe;

pub fn create_probe<P: AsRef<Path>>(path: &P) -> anyhow::Result<ProbeResult> {
    let probe = get_probe();
    let file = File::open(path).unwrap();
    let media_source = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let probe_result = probe.format(
        &Hint::new(),
        media_source,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    Ok(probe_result)
}

pub fn get_cover_art(probe: &mut ProbeResult) -> Option<Vec<u8>> {
    if let Some(metadata) = probe.metadata.get() {
        if let Some(current) = metadata.current() {
            if let Some(visual) = current.visuals().first() {
                return Some(Vec::from(visual.data.clone()));
            }
        }
    }

    None
}

pub fn decode_audio(probe: &mut ProbeResult) -> Option<Vec<f32>> {
    let track = probe.format.default_track().unwrap();

    let codec_registry = get_codecs();
    let mut decoder = codec_registry
        .make(&track.codec_params, &DecoderOptions { verify: false })
        .ok()?;

    let mut samples: Vec<f32> = vec![];

    while let Ok(packet) = probe.format.next_packet() {
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
