//! app
//!

use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::Error;

use crate::config::Urls;
use crate::constants::CONFIG_PATH_ENV_KEY;
use crate::{browser, config, constants};

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
            None => Ok(env::var(CONFIG_PATH_ENV_KEY).map(|c| Path::new(&c).to_path_buf())?),
        }
    }
}

/// run jy
pub fn run(options: RunOptions) -> anyhow::Result<()> {
    let config = match options.get_config_path() {
        Ok(path) => {
            println!("INFO: 配置路径:{:?}", path);
            match config::get_config(&path) {
                Ok(content) => content,
                Err(e) => panic!("WARN: 尝试从配置路径加载文件失败 {:?}, error: {}", path, e),
            }
        },
        Err(_) => {
            let home_config_dir = dirs::home_dir().expect("get home dir").join(".jy");
            println!("INFO: 未指定要加载配置文件, 尝试从{:?}加载", home_config_dir);

            let home_config_path = home_config_dir.join(constants::DEFAULT_FILE_NAME);
            if home_config_path.exists() {
                println!("INFO: 默认配置文件存在 {:?}", home_config_path);
                config::get_config(home_config_path)?
            } else {
                fs::create_dir(home_config_dir).expect("create dir");
                println!("WARN: 默认配置文件不存在, 使用默认配置列表创建{:?}...", home_config_path);
                fs::write(home_config_path, constants::DEFAULT_CONFIG).expect("write file");
                constants::DEFAULT_CONFIG.into()
            }
        },
    };

    let urls = config.urls();

    browser::browser_batch(urls, options.dry_run)
}

///show info
pub fn show_info() -> anyhow::Result<(), Error> {
    println!("CONFIG_PATH_ENV_KEY: {}", constants::CONFIG_PATH_ENV_KEY);
    println!("ENV {} VALUE: {:?}", constants::CONFIG_PATH_ENV_KEY, std::env::var(constants::CONFIG_PATH_ENV_KEY));
    println!("DEFAULT FILE NAME: {}", constants::DEFAULT_FILE_NAME);
    println!("DEFAULT CONFIG CONTENT:\n{}", constants::DEFAULT_CONFIG);

    Ok(())
}
