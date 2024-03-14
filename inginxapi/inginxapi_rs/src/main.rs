#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

mod args;
mod service;
mod types;

use clap::Parser;

use self::args::Args;
use self::service::{display_for_cli, stat_api, StatApiOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("args:{:?}", args);
    println!("{}", "*".repeat(80));

    let result = stat_api(
        args.access_log_file,
        args.api_name,
        StatApiOptions { debug: args.debug, ..StatApiOptions::default() },
    )?;

    display_for_cli(result);

    Ok(())
}
