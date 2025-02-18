use std::fs;

fn main() -> std::io::Result<()> {
    let supported_types = [
        "audio/aac",
        "audio/flac",
        "audio/midi",
        "audio/mp2",
        "audio/mp4",
        "audio/mpeg",
        "audio/x-aiff",
        "audio/x-caf",
        "audio/x-opus+ogg",
        "audio/x-vorbis+ogg",
        "audio/x-wav",
    ];

    let out_path = "audio.thumbnailer";
    let contents = format!(
        "[Thumbnailer Entry]
TryExec=/usr/bin/audio-thumbnailer
Exec=/usr/bin/audio-thumbnailer -i %i -o %o
MimeType={}
",
        supported_types.join(";")
    );

    fs::write(out_path, contents)?;

    Ok(())
}
