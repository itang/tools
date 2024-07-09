//! args
//!
use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Dry run mode
    #[arg(long)]
    pub dry_run: bool,

    /// Show info
    #[arg(long)]
    pub show_info: bool,
}
