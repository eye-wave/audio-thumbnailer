use crate::decode::midi::MidiTracks;
use anyhow::anyhow;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub fn draw_midi<P: AsRef<Path>>(
    midi: &MidiTracks,
    out_path: &P,
    size: (u32, u32),
    color: &Rgb<u8>,
    bg_color: &Rgb<u8>,
) -> anyhow::Result<()> {
    if midi.tracks.is_empty() {
        return Err(anyhow!("Decoded audio data is empty."));
    }

    let (width, height) = size;
    if width < 1 || height < 1 {
        return Err(anyhow!("Thumbnail size cannot be 0."));
    }

    let mut img = ImageBuffer::from_fn(width, height, |_, _| *bg_color);

    let note_range = midi.highest_note - midi.lowest_note + 1;
    let width_unit = width as f32 / midi.duration as f32;
    let height_unit = height as f32 / note_range as f32;

    let translate = |x: u32, y: u8| {
        let x = x as f32 * width_unit;
        let y = (note_range - (y - midi.lowest_note)) as f32 * height_unit;

        (x as u32, y as u32)
    };

    for track in midi.tracks.iter() {
        for note in track.iter() {
            let (x1, y1) = translate(note.start, note.key + 1);
            let (x2, y2) = translate(note.start + note.duration, note.key);

            for x in x1..x2 {
                for y in y1..y2 {
                    img.put_pixel(x, y, *color);
                }
            }
        }
    }

    img.save(out_path)?;

    Ok(())
}
