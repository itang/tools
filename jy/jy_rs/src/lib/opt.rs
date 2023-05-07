//! opt
use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;

/// The get config path trait
pub trait IConfigPath {
    /// get config path
    fn get_config_path(&self) -> Result<PathBuf>;
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    /// The config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Dry run mode
    #[arg(short, long)]
    pub dry_run: bool,

    /// Show info
    #[arg(short, long)]
    pub show_info: bool,
}

macro_rules! default_config_name {
    () => {
        "jiayou.toml"
    };
}

/// the default config file name
pub static DEFAULT_FILE_NAME: &str = default_config_name!();

/// the config path env key
pub static CONFIG_PATH_ENV_KEY: &str = "JY_CONFIG";

/// the default config content
pub static DEFAULT_CONFIG: &str = include_str!(default_config_name!());

impl IConfigPath for Opt {
    fn get_config_path(&self) -> Result<PathBuf> {
        match &self.config {
            Some(p) => Ok(p.clone()),
            None => Ok(env::var(CONFIG_PATH_ENV_KEY).map(|c| Path::new(&c).to_path_buf())?),
        }
    }
}
