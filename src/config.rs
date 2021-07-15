use serde::Deserialize;
use typetest_themes::Theme;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub global_settings: GlobalSettings,
    pub random_generator_settings: RandomGeneratorSettings,
}

#[derive(Debug, Default, Deserialize)]
pub struct GlobalSettings {
    pub theme: Theme
}

#[derive(Debug, Default, Deserialize)]
pub struct RandomGeneratorSettings {
    pub time_length_seconds: u64,
}

// TODO: Points of failure:
// - Configuration directory/file doesn't exist
// - Missing configuration options

/// Loads the configuration file on native platforms.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_config() -> anyhow::Result<Config> {
    // let base_dirs = directories::BaseDirs::new().expect("Could not locate base directory for configuration file!");
    // let config_path = base_dirs.config_dir().join("config.toml");
    // let config_str = std::fs::read_to_string(config_path)?;
    // let config = toml::from_str(&config_str)?;

    let config = Config {
        global_settings: GlobalSettings {
            theme: Theme::DefaultLight
        },
        random_generator_settings: RandomGeneratorSettings {
            time_length_seconds: 30
        },
    };
    Ok(config)
}
