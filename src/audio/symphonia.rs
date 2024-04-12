use super::AudioDecoder;
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
use symphonia_core::probe::ProbeResult;

impl AudioDecoder {
    pub fn create_probe(&mut self, path: &Path) -> Option<ProbeResult> {
        if path.extension().is_some_and(|ext| ext == "wv") {
            unimplemented!()
        }

        let probe = get_probe();
        let file = File::open(path).unwrap();
        let media_source =
            MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        if let Ok(probe_result) = probe.format(
            &Hint::new(),
            media_source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        ) {
            self.current_file = Some(path.display().to_string());
            return Some(probe_result);
        }

        None
    }

    pub fn get_cover_art(&mut self, probe: &mut ProbeResult) -> Option<Vec<u8>> {
        if let Some(metadata) = probe.metadata.get() {
            if let Some(current) = metadata.current() {
                if let Some(visual) = current.visuals().first() {
                    return Some(Vec::from(visual.data.clone()));
                }
            }
        }

        None
    }

    pub fn decode_audio(&mut self, probe: &mut ProbeResult) -> Option<Vec<f32>> {
        if let Some(current) = &self.current_file {
            match current {
                s if s.ends_with(".opus") => unimplemented!(),
                s if s.ends_with(".wv") => unimplemented!(),
                _ => {}
            };
        }

        let track = probe.format.default_track().unwrap();

        let codec_registry = get_codecs();
        let mut decoder = codec_registry
            .make(&track.codec_params, &DecoderOptions::default())
            .unwrap();
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

        self.current_file = None;
        Some(samples)
    }
}
