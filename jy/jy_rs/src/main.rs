//! jy cli.
//!
//! jy cli rust version.
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::fs;
use toml::{self, Value};

use anyhow::{Error, Result};
use clap::Parser;

use jy::{
    self, browser,
    opt::{self, IConfigPath, Opt},
};

fn main() -> Result<()> {
    let opt = Opt::parse();
    println!("{opt:?}");

    match opt {
        Opt { dry_run: true, .. } => handle_dry_run(&opt),
        Opt { show_info: true, .. } => handle_show_info(),
        _ => handle_jy(opt),
    }
}

fn handle_jy(opt: Opt) -> Result<()> {
    let content = match opt.get_config_path() {
        Ok(path) => {
            println!("INFO: 配置路径:{:?}", path.clone());
            match jy::get_content(path.clone()) {
                Ok(content) => content,
                Err(e) => panic!("WARN: 尝试从配置路径加载文件失败 {:?}, error: {}", path, e),
            }
        },
        Err(_) => {
            let home_config_dir = dirs::home_dir().expect("get home dir").join(".jy");
            println!("INFO: 未指定要加载配置文件, 尝试从{:?}加载", home_config_dir.clone());
            let home_config_path = home_config_dir.join(opt::DEFAULT_FILE_NAME);
            if home_config_path.exists() {
                println!("INFO: 默认配置文件存在 {:?}", home_config_path.clone());
                jy::get_content(home_config_path)?
            } else {
                fs::create_dir(home_config_dir).expect("create dir");
                println!("WARN: 默认配置文件不存在, 使用默认配置列表创建{:?}...", home_config_path.clone());
                fs::write(home_config_path, opt::DEFAULT_CONFIG).expect("write file");
                opt::DEFAULT_CONFIG.into()
            }
        },
    };

    let config = content.parse::<Value>()?;

    browser::browser_batch(config)
}

fn handle_show_info() -> Result<(), Error> {
    println!("CONFIG_PATH_ENV_KEY: {}", opt::CONFIG_PATH_ENV_KEY);
    println!("ENV {} VALUE: {:?}", opt::CONFIG_PATH_ENV_KEY, std::env::var(opt::CONFIG_PATH_ENV_KEY));
    println!("DEFAULT FILE NAME: {}", opt::DEFAULT_FILE_NAME);
    println!("DEFAULT CONFIG CONTENT:\n{}", opt::DEFAULT_CONFIG);

    Ok(())
}

fn handle_dry_run(opt: &Opt) -> Result<(), Error> {
    let content = jy::get_content(opt.get_config_path()?)?;
    println!("{content}");

    println!("{}", "-".repeat(80));
    println!("dry run. exit!");
    Ok(())
}
