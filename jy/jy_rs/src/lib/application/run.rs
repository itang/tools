use std::env;
use std::path::{Path, PathBuf};

use crate::domain::{constants, JiayouList};
use crate::infrastructure::BrowserImpl;
use crate::infrastructure::ConfigProviderImpl;

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
            None => Ok(env::var(constants::CONFIG_PATH_ENV_KEY).map(|p| Path::new(&p).to_path_buf())?),
        }
    }
}

/// run jy
pub fn run(options: RunOptions) -> anyhow::Result<()> {
    let jiayou_list = JiayouList::parse(options.get_config_path().ok(), ConfigProviderImpl::new())?;
    jiayou_list.browser(options.dry_run, BrowserImpl {})
}
