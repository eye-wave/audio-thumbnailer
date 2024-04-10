use plotters::prelude::*;
use std::path::Path;

pub fn is_wavetable(chunk: &[u8]) -> bool {
    let chunk = String::from_utf8_lossy(chunk);
    chunk.contains("wavetable")
}

pub fn draw_wavetable(samples: &[f32], out_path: &Path, size: &(u32, u32), color: &RGBAColor) {
    let window_size = 2048;
    let frames = if samples.len() / window_size > 256 {
        256
    } else {
        samples.len() / window_size
    };

    let is_single_frame = frames < 2;

    let root = BitMapBackend::new(out_path, *size).into_drawing_area();
    root.fill(&RGBAColor(0, 0, 0, 0.0)).unwrap();

    let range_x = 0.0..window_size as f64;
    let range_y = -2.0..2.0;
    let range_z = 0.0..(frames + is_single_frame as usize) as f64;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_3d(range_x, range_y, range_z)
        .unwrap();

    chart.with_projection(|mut p| {
        p.pitch = 0.42;
        p.yaw = 0.51;
        p.scale = 1.2;

        p.into_matrix()
    });

    let x_axis = 0..window_size;
    let z_axis = 0..frames + is_single_frame as usize;

    let mut fill = color.clone();
    fill.3 = 0.01;

    let series = SurfaceSeries::xoz(
        x_axis.map(|x| x as f64),
        z_axis.map(|z| z as f64),
        |x, mut z| {
            z = if is_single_frame {
                0.0
            } else {
                frames as f64 - z
            };

            let i = (window_size as f64 * z + x) as usize;
            let y = samples.get(i).unwrap_or(&0.0);

            *y as f64
        },
    )
    .style(fill);

    chart.draw_series(series).unwrap();
}
