use std::path::PathBuf;

use clap::Parser;
use filelinec::FileLineCount;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
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

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    args: Args,
}

impl Router {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let Args { ext, dir, sort } = self.args;
        
        FileLineCount::from_dir(dir, ext, sort)?.pretty_print();

        Ok(())
    }
}
