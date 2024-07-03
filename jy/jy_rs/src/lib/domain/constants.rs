//! constants

macro_rules! default_config_name {
    () => {
        "jiayou.toml"
    };
}

/// the default config file name
pub const DEFAULT_FILE_NAME: &str = default_config_name!();

/// the config path env key
pub const CONFIG_PATH_ENV_KEY: &str = "JY_CONFIG";

/// the default config content
pub const DEFAULT_CONFIG: &str = include_str!(default_config_name!());
