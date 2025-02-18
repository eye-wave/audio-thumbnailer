use anyhow::anyhow;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub fn draw_waveform<P: AsRef<Path>>(
    samples: &[u8],
    out_path: &P,
    size: (u32, u32),
    color: &Rgb<u8>,
    bg_color: &Rgb<u8>,
) -> anyhow::Result<()> {
    if samples.is_empty() {
        return Err(anyhow!("Decoded audio data is empty."));
    }

    let (width, height) = size;
    if width < 1 || height < 1 {
        return Err(anyhow!("Thumbnail size cannot be 0."));
    }

    let mut img = ImageBuffer::from_fn(width, height, |_, _| *bg_color);
    let slice_size = samples.len() / width as usize;

    for slice_index in 0..width {
        let mut min = 0xff;
        let mut max = 0;

        for sample_index in 0..slice_size {
            let index = slice_index as usize * slice_size + sample_index;
            let sample = samples[index];

            min = min.min(sample);
            max = max.max(sample);
        }

        let min_y = ((min as f32 / 255.0) * (height - 1) as f32) as u32;
        let max_y = ((max as f32 / 255.0) * (height - 1) as f32) as u32;

        if min_y == max_y {
            img.put_pixel(slice_index, min_y, *color);
        } else {
            for y in min_y..max_y {
                img.put_pixel(slice_index, y, *color);
            }
        }
    }

    img.save(out_path)?;

    Ok(())
}
