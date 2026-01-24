#![feature(const_trait_impl)]
#![feature(const_index)]

pub const ALL_FORMATS: &[&str] = &[
    "audio/aac",
    "audio/x-hx-aac-adts",
    "audio/flac",
    "audio/mp2",
    "audio/mp4",
    "audio/mpeg",
    "audio/x-aiff",
    "audio/x-caf",
    "audio/x-vorbis+ogg",
    "audio/x-wav",
    "audio/x-opus+ogg",
    "audio/ogg",
    "audio/x-wavpack",
    // Other
    "audio/x-mpegurl",
    "audio/midi",
];

pub const SYMPHONIA_FORMATS: &[&str] = &ALL_FORMATS[0..13];
pub const OTHER_FORMATS: &[&str] = &ALL_FORMATS[13..15];
