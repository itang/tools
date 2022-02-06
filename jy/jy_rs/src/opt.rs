use std::env;
use std::path::{Path, PathBuf};

use anyhow::Result;
use structopt::StructOpt;

pub trait IConfigPath {
    fn get_config_path(&self) -> Result<PathBuf>;
}

#[derive(StructOpt, Debug)]
#[structopt(name = "jy_rs")]
pub struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,
    // The number of occurrences of the `v/verbose` flag
    //// Verbose mode (-v, -vv, -vvv, etc.)
    //#[structopt(short, long, parse(from_occurrences))]
    //verbose: u8,
    #[structopt(short, long)]
    pub dry_run: bool,
}

pub static DEFAULT_FILE_NAME: &str = "jiayou.toml";

impl IConfigPath for Opt {
    fn get_config_path(&self) -> Result<PathBuf> {
        match &self.config {
            Some(p) => Ok(p.clone()),
            None => env::var("JY_CONFIG")
                .map(|c| Path::new(&c).to_path_buf())
                .or_else(|_| {
                    Ok(env::var("HOME")
                        .map(|x| Path::new(&x).join("bin").join(DEFAULT_FILE_NAME))?)
                }),
        }
    }
}
