use std::fs;

use audio_formats::ALL_FORMATS;

fn main() -> std::io::Result<()> {
    let out_path = "audio.thumbnailer";
    let contents = format!(
        "[Thumbnailer Entry]
TryExec=/usr/bin/audio-thumbnailer
Exec=/usr/bin/audio-thumbnailer -i %i -o %o
MimeType={}
",
        ALL_FORMATS.join(";")
    );

    fs::write(out_path, contents)?;

    Ok(())
}
