use lewton::inside_ogg::OggStreamReader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn decode_audio<P: AsRef<Path>>(path: &P) -> anyhow::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut ogg_reader = OggStreamReader::new(BufReader::new(file))?;

    let channels = ogg_reader.ident_hdr.audio_channels as usize;
    let mut pcm_data = Vec::new();

    while let Some(pcm_packets) = ogg_reader.read_dec_packet()? {
        for packet in pcm_packets {
            for frame in packet.chunks(channels) {
                let mono_sample: i16 =
                    (frame.iter().map(|&s| s as i32).sum::<i32>() / channels as i32) as i16;
                let u8_sample = ((mono_sample as i32 + 32768) / 256) as u8;

                pcm_data.push(u8_sample);
            }
        }
    }

    Ok(pcm_data)
}
