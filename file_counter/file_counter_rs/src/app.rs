use std::path::PathBuf;

use clap::Parser;

mod handlers;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Extension name for match
    #[arg(short, long)]
    ext_name: Option<Vec<String>>,

    /// Contains for match
    #[arg(short, long)]
    contains: Option<Vec<String>>,

    ///The source dir
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,

    #[arg(long)]
    exported_dir: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    show_same_name_files: bool,
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

    pub fn run(self) -> anyhow::Result<()> {
        println!("DEBUG: args: {:?}", self.args);
        handlers::main(self.args)
    }
}
