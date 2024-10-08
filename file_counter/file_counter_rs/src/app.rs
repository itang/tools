use std::path::PathBuf;

use clap::Parser;

mod handlers;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Extension name for match
    #[arg(short, long)]
    ext_name: Option<Vec<String>>,

    /// Glob match
    #[arg(long)]
    glob: Option<String>,

    /// Contains for match
    #[arg(short, long)]
    contains: Option<Vec<String>>,

    /// Contains for match
    #[arg(short = 'n', long)]
    not_contains: Option<Vec<String>>,

    /// Ignore directories
    #[arg(short, long)]
    ignore_dirs: Option<Vec<PathBuf>>,

    /// The source dir
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,

    /// Grouped by file name and show
    #[arg(short = 'g', long, default_value_t = false)]
    show_same_name_files: bool,

    /// Show the extension of matched files
    #[arg(long, default_value_t = false)]
    show_extensions: bool,

    /// Show all verbose info
    #[arg(long, default_value_t = false)]
    show_all: bool,

    /// The exported dir
    #[arg(short = 't', long)]
    exported_dir: Option<PathBuf>,

    /// The strip prefix before dir. e.g. /src
    #[arg(short = 's', long)]
    strip_prefix_before_dir: Option<PathBuf>,
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
