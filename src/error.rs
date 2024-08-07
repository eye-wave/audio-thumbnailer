pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Symphonia(#[from] symphonia::core::errors::Error),

    #[error(transparent)]
    Img(#[from] image::error::ImageError),

    #[error(transparent)]
    Color(#[from] csscolorparser::ParseColorError),

    #[error("{0}")]
    Custom(&'static str),

    #[cfg(feature = "config_file")]
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
