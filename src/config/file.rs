#[cfg(feature = "config_file")]
pub mod config_file {
    use crate::config::Config;
    use crate::{Error, Result};
    use dirs::config_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    fn get_config_path() -> Result<PathBuf> {
        if let Some(config_dir) = config_dir() {
            let config_path = config_dir.join(env!("CARGO_PKG_NAME")).join("config.toml");

            return Ok(config_path);
        }

        Err(Error::Custom("Could not find a config dir".to_string()))
    }

    macro_rules! init_if_none {
        ($config:expr, $($field:ident),*) => {
            $(
                if $config.$field.is_none() {
                    $config.$field = Some(Default::default());
                }
            )*
        };
    }

    impl Config {
        pub fn load() -> Result<Self> {
            let config_path = get_config_path()?;
            if config_path.exists() {
                let contents = read_to_string(config_path)?;
                let mut config: Config = toml::from_str(&contents)?;

                init_if_none!(
                    config,
                    cover_settings,
                    audio_analyzer_settings,
                    waveform_settings,
                    thumbnail_settings,
                    debug
                );

                Ok(config)
            } else {
                Ok(Self::default())
            }
        }
    }
}
