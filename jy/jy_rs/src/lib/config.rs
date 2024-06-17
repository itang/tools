//! config

use std::fs;
use std::path::Path;
use toml::Value;

/// get content from path
pub fn get_config<P>(config_path: P) -> anyhow::Result<Value>
where
    P: AsRef<Path>,
{
    let config_path: &Path = config_path.as_ref();
    println!("INFO: read from config file: {:?}", config_path);

    let result = fs::read_to_string(config_path)?.parse::<Value>();

    Ok(result?)
}

/// get urls from toml config
pub fn get_urls(config: Value) -> Vec<String> {
    let urls = config["urls"]
        .as_array()
        .into_iter()
        .flat_map(|x| x.iter().flat_map(|y| y.as_str().into_iter()))
        .map(|x| x.to_string());
    urls.collect()
}
