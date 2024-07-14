#[cfg(feature = "config_file")]
pub mod config_file {
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use crate::config::Config;
    use crate::{Result,Error};
    use dirs::config_dir;

    fn get_config_path() -> Result<PathBuf> {
        if let Some(config_dir) = config_dir() {
            let config_path = config_dir
                .join(env!("CARGO_PKG_NAME"))
                .join("config.toml");
            
            return Ok(config_path)
        }

        Err(Error::Custom("Could not find a config dir".to_string()))
    }

    impl Config {
        pub fn load() -> Result<Self> {
            let config_path = get_config_path()?;
            if config_path.exists() {
                let contents = read_to_string(config_path)?;
                let config: Config = toml::from_str(&contents)?;

                Ok(config)
            }
            else {
                Ok(Self::default())
            }
        }
    }
}
