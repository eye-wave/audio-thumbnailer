use std::str::FromStr;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum InterpolationType {
    Lanczos3,
    CatmullRom,
    Gaussian,
    Nearest,
    Triangle,
}

impl Default for InterpolationType {
    fn default() -> Self {
        Self::Triangle
    }
}

impl FromStr for InterpolationType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "lanczos3" => Ok(Self::Lanczos3),
            "catmullrom" => Ok(Self::CatmullRom),
            "gaussian" => Ok(Self::Gaussian),
            "nearest" => Ok(Self::Nearest),
            "triangle" => Ok(Self::Triangle),
            _ => Err(String::from("Invalid aspect ratio. Possible values are: \"lanczos3\", \"catmullrom\", \"gaussian\", \"nearest\", \"triangle\"")),
        }
    }
}
