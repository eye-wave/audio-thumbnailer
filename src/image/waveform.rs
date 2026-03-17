use std::path::Path;

use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

use crate::image::{color::Gradient, DrawError};

pub fn draw_waveform<P: AsRef<Path>>(
    samples: &[u8],
    out_path: &P,
    size: (u32, u32),
    gradient: &Gradient,
    bg_color: &Rgb<u8>,
) -> Result<(), DrawError> {
    if samples.is_empty() {
        return Err(DrawError::DecoderEmpty);
    }

    let (width, height) = size;
    if width < 1 || height < 1 {
        return Err(DrawError::ThumbnailZero);
    }

    let mut img = ImageBuffer::from_fn(width, height, |_, _| *bg_color);
    let slice_size = samples.len() / width as usize;

    if slice_size == 0 {
        return Err(DrawError::NotEnoughSamples);
    }

    let column_data: Vec<(u32, u32)> = (0..width)
        .into_par_iter()
        .map(|slice_index| {
            let mut min = u8::MAX;
            let mut max = u8::MIN;

            let start_idx = slice_index as usize * slice_size;
            let end_idx = (start_idx + slice_size).min(samples.len());

            for sample in &samples[start_idx..end_idx] {
                min = min.min(*sample);
                max = max.max(*sample);
            }

            let min_y = (min as u32 * (height - 1)) / 255;
            let max_y = (max as u32 * (height - 1)) / 255;

            (min_y, max_y)
        })
        .collect();

    for (slice_index, &(min_y, max_y)) in column_data.iter().enumerate() {
        for y in min_y..=max_y {
            let t = y as f32 / (height - 1) as f32;
            let color = gradient.step(t);
            img.put_pixel(slice_index as u32, y, color);
        }
    }

    img.save(out_path)?;

    Ok(())
}
