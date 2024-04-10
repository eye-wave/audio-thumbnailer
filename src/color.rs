use hex_color::HexColor;
use plotters::style::RGBAColor;

pub fn parse_hex_color(input: &str) -> Option<RGBAColor> {
    if let Ok(color) = HexColor::parse(input) {
        return Some(RGBAColor(color.r, color.b, color.g, color.a as f64));
    }

    None
}
