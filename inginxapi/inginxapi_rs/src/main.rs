#![forbid(unsafe_code)]

mod args;
mod service;
mod types;

use crate::service::display_for_cli;
use clap::Parser;

use self::args::Args;
use self::service::stat_api;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("args:{:?}", args);
    println!("{}", "*".repeat(80));

    let result = stat_api(args.file, args.api_name, args.debug)?;
    display_for_cli(result);

    Ok(())
}
