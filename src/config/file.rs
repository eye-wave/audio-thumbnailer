#[cfg(feature = "config_file")]
pub mod config_file {
    use crate::config::{Config, ConfigDeserialize};
    use crate::Result;
    use dirs::config_dir;
    use std::fs::{create_dir_all, read_to_string, write};
    use std::path::PathBuf;

    fn get_config_path() -> PathBuf {
        config_dir()
            .unwrap()
            .join(env!("CARGO_PKG_NAME"))
            .join("config.toml")
    }

    impl ConfigDeserialize {
        pub fn load() -> Result<Config> {
            let config_path = get_config_path();
            if config_path.exists() {
                let contents = read_to_string(config_path)?;
                let config: ConfigDeserialize = toml::from_str(&contents)?;

                return Ok(Config {
                    cover_settings: config.cover_settings.unwrap_or_default(),
                    waveform_settings: config.waveform_settings.unwrap_or_default(),
                    thumbnail_settings: config.thumbnail_settings.unwrap_or_default(),
                    debug: config.debug.unwrap_or_default(),

                    #[cfg(feature = "colored_waveform")]
                    audio_analyzer_settings: config.audio_analyzer_settings.unwrap_or_default(),

                    #[cfg(feature = "3d_wavetables")]
                    wavetable_settings: config.wavetable_settings.unwrap_or_default(),
                });
            }

            Ok(Config::default())
        }

        pub fn create_file() -> Result<()> {
            let config_path = get_config_path();
            if !config_path.exists() {
                create_dir_all(config_path.parent().unwrap())?;
            }

            let config_content = include_str!("./example_config.toml");
            let debug_path = dirs::cache_dir().unwrap().join(env!("CARGO_PKG_NAME"));

            let config_content =
                config_content.replace("$DEBUG_PATH", debug_path.to_string_lossy().as_ref());

            write(&config_path, config_content)?;

            println!("Successfully created config file at {:?}", &config_path);

            Ok(())
        }
    }
}
