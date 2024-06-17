#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! jy cli.
//!
//! jy cli rust version.
//!

use std::fs;

use anyhow::{Error, Result};

use jy::{
    self, browser,
    opt::{self, IConfigPath, Opt},
};

fn main() -> Result<()> {
    let opt = Opt::get();
    println!("{opt:?}");

    match opt {
        Opt { show_info: true, .. } => handle_show_info(),
        _ => handle_jy(opt),
    }
}

fn handle_jy(opt: Opt) -> Result<()> {
    let config = match opt.get_config_path() {
        Ok(path) => {
            println!("INFO: 配置路径:{:?}", path);
            match jy::get_config(&path) {
                Ok(content) => content,
                Err(e) => panic!("WARN: 尝试从配置路径加载文件失败 {:?}, error: {}", path, e),
            }
        },
        Err(_) => {
            let home_config_dir = dirs::home_dir().expect("get home dir").join(".jy");
            println!("INFO: 未指定要加载配置文件, 尝试从{:?}加载", home_config_dir);

            let home_config_path = home_config_dir.join(opt::DEFAULT_FILE_NAME);
            if home_config_path.exists() {
                println!("INFO: 默认配置文件存在 {:?}", home_config_path);
                jy::get_config(home_config_path)?
            } else {
                fs::create_dir(home_config_dir).expect("create dir");
                println!("WARN: 默认配置文件不存在, 使用默认配置列表创建{:?}...", home_config_path);
                fs::write(home_config_path, opt::DEFAULT_CONFIG).expect("write file");
                opt::DEFAULT_CONFIG.into()
            }
        },
    };

    let urls = jy::urls(config);

    browser::browser_batch(urls, opt.dry_run)
}

fn handle_show_info() -> Result<(), Error> {
    println!("CONFIG_PATH_ENV_KEY: {}", opt::CONFIG_PATH_ENV_KEY);
    println!("ENV {} VALUE: {:?}", opt::CONFIG_PATH_ENV_KEY, std::env::var(opt::CONFIG_PATH_ENV_KEY));
    println!("DEFAULT FILE NAME: {}", opt::DEFAULT_FILE_NAME);
    println!("DEFAULT CONFIG CONTENT:\n{}", opt::DEFAULT_CONFIG);

    Ok(())
}
