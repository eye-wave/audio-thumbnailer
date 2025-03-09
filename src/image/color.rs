use image::Rgb;

pub fn parse_color(color: &str) -> anyhow::Result<Rgb<u8>> {
    let parsed = csscolorparser::parse(color)?;
    let rgba = parsed.to_rgba8();
    let rgb = Rgb([rgba[0], rgba[1], rgba[2]]);

    Ok(rgb)
}

pub struct Gradient {
    values: Vec<Rgb<u8>>,
}

impl Gradient {
    pub fn new(colors: &[String]) -> Self {
        let values: Vec<_> = colors
            .iter()
            .filter_map(|color| parse_color(color).ok())
            .collect();

        Self { values }
    }

    pub fn has_single_color(&self) -> bool {
        self.values.len() == 1
    }

    pub fn step(&self, step: f32) -> Rgb<u8> {
        if self.has_single_color() {
            return self.values[0];
        }

        let num_steps = self.values.len() - 1;
        let step_index = (step * num_steps as f32).min(num_steps as f32) as usize;
        let t = step * num_steps as f32 - step_index as f32;

        let start_color = &self.values[step_index];
        let end_color = &self.values[step_index + 1];

        self.lerp(start_color, end_color, t)
    }

    fn lerp(&self, start: &Rgb<u8>, end: &Rgb<u8>, t: f32) -> Rgb<u8> {
        let r = (start[0] as f32 + (end[0] as f32 - start[0] as f32) * t) as u8;
        let g = (start[1] as f32 + (end[1] as f32 - start[1] as f32) * t) as u8;
        let b = (start[2] as f32 + (end[2] as f32 - start[2] as f32) * t) as u8;

        Rgb::from([r, g, b])
    }
}
