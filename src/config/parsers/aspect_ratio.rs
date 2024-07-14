use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "config_file", derive(serde_derive::Deserialize))]
pub enum AspectRatio {
    Auto,
    Crop,
    Stretch,
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::Crop
    }
}

impl FromStr for AspectRatio {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "auto" => Ok(Self::Auto),
            "crop" => Ok(Self::Crop),
            "stretch" => Ok(Self::Stretch),
            _ => Err(String::from(
                "Invalid aspect ratio. Possible values are: \"auto\", \"crop\", \"stretch\"",
            )),
        }
    }
}
