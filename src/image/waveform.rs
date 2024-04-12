use plotters::prelude::*;
use std::path::Path;

pub fn draw_waveform(samples: &[f32], out_path: &Path, size: &(u32, u32), color: &RGBAColor) {
    if samples.is_empty() {
        panic!("samples cannot be empty");
    }

    let mut quality = samples.len() / (size.0 * 100) as usize;
    let root = BitMapBackend::new(out_path, *size).into_drawing_area();
    root.fill(&TRANSPARENT).unwrap();

    if quality < 1 {
        quality = 1
    }

    let range_x = 0.0..(samples.len() / quality) as f64;
    let range_y = -1.0..1.0;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(range_x, range_y)
        .unwrap();

    chart.configure_mesh().disable_x_mesh().draw().unwrap();

    let data_to_draw: Vec<(f64, f64)> = samples
        .iter()
        .step_by(quality)
        .enumerate()
        .map(|(x, &y)| (x as f64, y as f64))
        .collect();

    let mut new_color = *color;
    new_color.3 = 0.2;

    let series = LineSeries::new(data_to_draw, color);
    chart.draw_series(series).unwrap();
}
