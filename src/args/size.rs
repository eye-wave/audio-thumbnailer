pub fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        panic!("Invalid dimensions: {}", input)
    }

    let width = parts[0].parse::<u32>();
    let height = parts[1].parse::<u32>();

    match (width, height) {
        (Ok(w), Ok(h)) => (w, h),
        err => panic!("{:?}", err),
    }
}
