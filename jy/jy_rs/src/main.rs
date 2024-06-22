#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! jy cli.
//!
//! jy cli rust version.
//!
mod args;

use clap::Parser;

use jy::{self, RunOptions};

use self::args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    match args {
        Args { show_info: true, .. } => jy::show_info(),
        Args { config, dry_run, .. } => jy::run(RunOptions { config, dry_run }),
    }
}
