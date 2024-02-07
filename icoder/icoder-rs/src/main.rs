#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::{Args, Parser, Subcommand};
use std::error::Error;

use icoder_rs::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    actions: Actions,
}

#[derive(Subcommand, Debug, Clone)]
enum Actions {
    Base64(Base64Options),
    Hex(HexOptions),
    Uuid,
    I2hex(I2hexOptions),
    Upcase(UpcaseOptions),
    Lowcase(LowcaseOptions),
}

///base64
#[derive(Args, Debug, Clone)]
struct Base64Options {
    ///input
    input: String,
    ///decode
    #[arg(short, long)]
    decode: bool,

}

///hex
#[derive(Args, Debug, Clone)]
struct HexOptions {
    ///input
    input: String,
    ///decode
    #[arg(short, long)]
    decode: bool,
}

///I2Hex
#[derive(Args, Debug, Clone)]
struct I2hexOptions {
    ///input
    input: String,
    ///decode
    #[arg(short, long)]
    decode: bool,
}

///upcase
#[derive(Args, Debug, Clone)]
struct UpcaseOptions {
    ///input
    input: String,
}

///lowcase
#[derive(Args, Debug, Clone)]
struct LowcaseOptions {
    ///input
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.actions {
        Actions::Base64(options) => {
            if options.decode {
                println!("{}", Base64.decode(options.input)?)
            } else {
                println!("{}", Base64.encode(options.input)?)
            }
        }
        Actions::Hex(options) => {
            if options.decode {
                println!("{}", Hex.decode(options.input)?)
            } else {
                println!("{}", Hex.encode(options.input)?)
            }
        }
        Actions::I2hex(options) => {
            if options.decode {
                println!("{}", I2Hex.decode(options.input)?)
            } else {
                println!("{}", I2Hex.encode(options.input)?)
            }
        }
        Actions::Uuid => println!("{}", uuid()),
        Actions::Upcase(options) => println!("{}", options.input.to_uppercase()),
        Actions::Lowcase(options) => println!("{}", options.input.to_lowercase()),
    }

    Ok(())
}
