#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::Parser;

use jsonfmt::fmt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    values: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let value = args.values.join("");
    let ret = fmt(&value);
    match ret {
        Ok(value) => println!("{value}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
