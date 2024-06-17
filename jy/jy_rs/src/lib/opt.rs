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
    #[arg(long)]
    pub dry_run: bool,

    /// Show info
    #[arg(long)]
    pub show_info: bool,
}

impl Opt {
    /// parse from args
    pub fn get() -> Self {
        Self::parse()
    }
}

impl IConfigPath for Opt {
    fn get_config_path(&self) -> Result<PathBuf> {
        match &self.config {
            Some(p) => Ok(p.clone()),
            None => Ok(env::var(CONFIG_PATH_ENV_KEY).map(|c| Path::new(&c).to_path_buf())?),
        }
    }
}

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
