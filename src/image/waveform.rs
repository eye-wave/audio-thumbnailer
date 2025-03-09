use super::Gradient;
use anyhow::anyhow;
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use std::path::Path;
use std::sync::Mutex;

pub fn draw_waveform<P: AsRef<Path>>(
    samples: &[f32],
    out_path: &P,
    size: (u32, u32),
    gradient: &Gradient,
    bg_color: &Rgb<u8>,
) -> anyhow::Result<()> {
    if samples.is_empty() {
        return Err(anyhow!("Decoded audio data is empty."));
    }

    let (width, height) = size;
    if width < 1 || height < 1 {
        return Err(anyhow!("Thumbnail size cannot be 0."));
    }

    let window_size = 2048;

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(window_size);

    let fft_slice_count = samples.len() / window_size;
    let dominant_frequencies: Vec<u8> = if gradient.has_single_color() {
        vec![]
    } else {
        (0..fft_slice_count)
            .into_par_iter()
            .map(|i| {
                let start = i * window_size;
                let end = start + window_size;

                let mut window: Vec<Complex<f32>> = samples[start..end]
                    .iter()
                    .map(|&sample| Complex::new(sample, 0.0))
                    .collect();

                fft.process(&mut window);

                window
                    .iter()
                    .map(|c| (c.re * c.re + c.im * c.im).sqrt())
                    .fold(0.0, f32::max) as u8
            })
            .collect()
    };

    let img = Mutex::new(ImageBuffer::from_fn(width, height, |_, _| *bg_color));
    let slice_size = samples.len() / width as usize;

    (0..width).into_par_iter().for_each(|slice_index| {
        let mut min: f32 = 1.0;
        let mut max: f32 = -1.0;

        for sample_index in 0..slice_size {
            let index = slice_index as usize * slice_size + sample_index;
            let sample = samples[index];

            min = min.min(sample);
            max = max.max(sample);
        }

        let min_y = (((min + 1.0) / 2.0) * (height - 1) as f32) as u32;
        let max_y = (((max + 1.0) / 2.0) * (height - 1) as f32) as u32;

        let color = if gradient.has_single_color() {
            gradient.step(0.0)
        } else {
            let step = slice_index as f32 / width as f32;
            let i = (step * fft_slice_count as f32) as usize;
            let freq = dominant_frequencies[i];

            gradient.step(freq as f32 / 255.0)
        };

        let mut img = img.lock().unwrap();

        if min_y == max_y {
            img.put_pixel(slice_index, min_y, color);
        } else {
            for y in min_y..max_y {
                img.put_pixel(slice_index, y, color);
            }
        }
    });

    let img = img.into_inner().unwrap();
    img.save(out_path)?;

    Ok(())
}
