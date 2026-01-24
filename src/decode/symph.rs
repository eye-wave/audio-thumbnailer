use std::fs::File;
use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::default::{get_codecs, get_probe};

pub fn create_probe<P: AsRef<Path>>(
    path: &P,
) -> Result<ProbeResult, symphonia::core::errors::Error> {
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
                return Some(visual.data.to_vec());
            }
        }
    }

    None
}

pub fn decode_audio(probe: &mut ProbeResult, downsample_factor: usize) -> Option<Vec<u8>> {
    let track = probe.format.default_track()?;

    let codec_registry = get_codecs();
    let mut decoder = codec_registry
        .make(&track.codec_params, &DecoderOptions { verify: false })
        .ok()?;

    let estimated_samples = track
        .codec_params
        .n_frames
        .map(|frames| frames as usize / downsample_factor)
        .unwrap_or(1000);

    let mut samples: Vec<u8> = Vec::with_capacity(estimated_samples);
    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    while let Ok(packet) = probe.format.next_packet() {
        let audio_buf = decoder.decode(&packet).unwrap();

        let spec = *audio_buf.spec();
        let duration = audio_buf.capacity() as u64;
        let channels = spec.channels.count();

        match &mut sample_buf {
            Some(buf) if buf.capacity() >= duration as usize => {}
            _ => {
                sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
            }
        }

        if let Some(buf) = &mut sample_buf {
            buf.copy_interleaved_ref(audio_buf);
            let interleaved = buf.samples();

            for i in (0..interleaved.len()).step_by(channels * downsample_factor) {
                let sample = ((interleaved[i] + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                samples.push(sample);
            }
        }
    }

    Some(samples)
}
