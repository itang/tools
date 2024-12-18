use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// file extension
    #[arg(short, long)]
    pub ext: Option<String>,

    /// the directory to search
    #[arg(default_value = ".")]
    dir: PathBuf,

    #[arg(short, long, default_value_t = false)]
    sort: bool,
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
        filelinec::list_and_count_file_lines(self.args.dir, self.args.ext, self.args.sort)
    }
}
