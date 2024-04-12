use std::path::Path;

mod symphonia;

pub fn get_cover_art(path: &Path) -> Option<Vec<u8>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("opus") => unimplemented!(),
        Some("wv") => unimplemented!(),
        _ => symphonia::get_cover_art(path),
    }
}
