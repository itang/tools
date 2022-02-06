use std::{fs, path::PathBuf};
//use std::process::{Command, Stdio};

use anyhow::Result;
use structopt::StructOpt;
use toml::{self, Value};

use opt::{IConfigPath, Opt};

mod opt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{opt:?}");

    if opt.dry_run {
        let content = get_content(opt.get_config_path()?)?;
        println!("{content}");

        println!("{}", "-".repeat(80));
        println!("dry run. exit!");

        Ok(())
    } else {
        let content = get_content(opt.get_config_path()?)?;
        let config = content.parse::<Value>()?;

        browser_batch(config)
    }
}

fn get_content(config_path: PathBuf) -> Result<String> {
    println!("Read config from {config_path:?}");

    Ok(fs::read_to_string(config_path)?)
}

fn browser_batch(config: Value) -> Result<()> {
    let urls = config["urls"]
        .as_array()
        .into_iter()
        .flat_map(|x| x.iter().flat_map(|y| y.as_str().into_iter()));
    for (index, url) in urls.enumerate() {
        println!("{index:4}: open {url}", index = index + 1);
        let _ = browser_single_url(url)?;
    }

    Ok(())
}

fn browser_single_url(url: &str) -> Result<std::process::Output> {
    Ok(webbrowser::open(url)?)
}
