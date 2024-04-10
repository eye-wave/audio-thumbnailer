// #[derive(Debug, Clone, Copy)]
pub enum AspectRatio {
    Auto,
    Crop,
    Stretch,
}

pub fn parse_aspect_ratio_arg(input: &str) -> AspectRatio {
    match input.trim() {
        "auto" => AspectRatio::Auto,
        "crop" => AspectRatio::Crop,
        "stretch" => AspectRatio::Stretch,
        _ => panic!("Invalid aspect ratio.\nPossible values are: \"auto\", \"crop\", \"stretch\""),
    }
}
