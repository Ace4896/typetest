use log::info;
use serde::{Deserialize, Serialize};
use typetest_themes::Theme;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub global: GlobalSettings,
    pub random_generator: RandomGeneratorSettings,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub theme: Theme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomGeneratorSettings {
    pub time_length_seconds: u64,
}

impl Default for RandomGeneratorSettings {
    fn default() -> Self {
        Self {
            time_length_seconds: 60,
        }
    }
}

// TODO: Points of failure:
// - Missing configuration options

/// Loads the configuration file on native platforms.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_config() -> anyhow::Result<Config> {
    use directories::BaseDirs;
    use std::{
        fs::{self, OpenOptions},
        io::Write,
    };

    let base_dirs =
        BaseDirs::new().expect("Could not locate base directory for configuration file!");
    let config_path = base_dirs.config_dir().join("typetest").join("config.toml");
    dbg!(&config_path);
    if config_path.exists() {
        info!("Config path exists");
        let mut merged_config = config::Config::new();
        merged_config.merge(config::File::from(config_path))?;
        let config = merged_config.try_into::<Config>()?;
        Ok(config)
    } else {
        info!("No configuration exists, creating new one");
        let parent_folder = config_path
            .parent()
            .expect("Could not retrieve parent directory for configuration path!");
        fs::create_dir_all(parent_folder)?;

        let mut config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config_path)?;

        let default_config = Config::default();
        let default_config_str = toml::to_string_pretty(&default_config)?;
        config_file.write_all(default_config_str.as_bytes())?;

        Ok(default_config)
    }
}
