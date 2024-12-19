//! app entry
use std::path::PathBuf;

use clap::Parser;

use crate::FileLineCount;

/// Config
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// file extension
    #[arg(short, long)]
    ext: Option<String>,

    /// sort by number of lines (ASC)
    #[arg(short, long, default_value_t = false)]
    sort: bool,

    /// the directory to count
    #[arg(default_value = ".")]
    dir: PathBuf,
}

impl Config {
    /// parse
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

/// Router
#[derive(Debug, Clone)]
pub struct Router {
    config: Config,
}

impl Router {
    /// new
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// run
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let Config { ext, dir, sort } = self.config;

        FileLineCount::from_dir(dir, ext, sort)?.pretty_print();

        Ok(())
    }
}
