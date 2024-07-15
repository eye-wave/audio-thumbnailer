mod symphonia;

pub struct AudioDecoder {
    current_file: Option<String>,
}

impl AudioDecoder {
    pub fn new() -> Self {
        Self { current_file: None }
    }
}
