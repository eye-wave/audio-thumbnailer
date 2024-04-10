use image::imageops;

pub fn parse_interpol_arg(input: &Option<String>) -> imageops::FilterType {
    match input {
        Some(input) => match input.trim() {
            "lanczos3" => imageops::Lanczos3,
            "gaussian" => imageops::Gaussian,
            "nearest" => imageops::Nearest,
            "triangle" => imageops::Triangle,
            _ => panic!(
                "Invalid interpolation.\nPossible values are: \"lanczos3\", \"gaussian\", \"nearest\", \"triangle\", "
            ),

        },
        None => imageops::Lanczos3
    }
}
