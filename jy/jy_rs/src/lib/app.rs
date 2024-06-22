//! app
//!

use std::env;
use std::path::{Path, PathBuf};

use anyhow::Error;

use crate::constants;
use crate::domain::JiayouList;
use crate::infrastructure::{BrowserImpl, ConfigProviderImpl};

/// RunOptions
pub struct RunOptions {
    /// The config file
    pub config: Option<PathBuf>,

    /// Dry run mode
    pub dry_run: bool,
}

impl RunOptions {
    fn get_config_path(&self) -> anyhow::Result<PathBuf> {
        match &self.config {
            Some(p) => Ok(p.clone()),
            None => Ok(env::var(constants::CONFIG_PATH_ENV_KEY).map(|c| Path::new(&c).to_path_buf())?),
        }
    }
}

/// run jy
pub fn run(options: RunOptions) -> anyhow::Result<()> {
    let jiayou_list = JiayouList::parse(options.get_config_path(), ConfigProviderImpl {})?;
    jiayou_list.browser(options.dry_run, BrowserImpl {})
}

///show info
pub fn show_info() -> anyhow::Result<(), Error> {
    println!("CONFIG_PATH_ENV_KEY: {}", constants::CONFIG_PATH_ENV_KEY);
    println!("ENV {} VALUE: {:?}", constants::CONFIG_PATH_ENV_KEY, std::env::var(constants::CONFIG_PATH_ENV_KEY));
    println!("DEFAULT FILE NAME: {}", constants::DEFAULT_FILE_NAME);
    println!("DEFAULT CONFIG CONTENT:\n{}", constants::DEFAULT_CONFIG);

    Ok(())
}
