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
    Base64(CoderOptions),
    Hex(CoderOptions),
    I2hex(CoderOptions),
    Upcase(UpcaseOptions),
    Lowcase(LowcaseOptions),
    Uuid,
    Random(RandomOptions),
}

///Coder Options
#[derive(Args, Debug, Clone)]
struct CoderOptions {
    ///input
    input: Option<String>,
    ///decode
    #[arg(short, long)]
    decode: bool,

}

///upcase
#[derive(Args, Debug, Clone)]
struct UpcaseOptions {
    ///input
    input: Option<String>,
}

///lowcase
#[derive(Args, Debug, Clone)]
struct LowcaseOptions {
    ///input
    input: Option<String>,
}

///random
#[derive(Args, Debug, Clone)]
struct RandomOptions {
    ///length
    length: Option<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.actions {
        Actions::Base64(options) => {
            if options.decode {
                println!("{}", Base64.decode(input(options.input))?)
            } else {
                println!("{}", Base64.encode(input(options.input))?)
            }
        }
        Actions::Hex(options) => {
            if options.decode {
                println!("{}", Hex.decode(input(options.input))?)
            } else {
                println!("{}", Hex.encode(input(options.input))?)
            }
        }
        Actions::I2hex(options) => {
            if options.decode {
                println!("{}", I2Hex.decode(input(options.input))?)
            } else {
                println!("{}", I2Hex.encode(input(options.input))?)
            }
        }
        Actions::Uuid => println!("{}", uuid()),
        Actions::Upcase(options) => println!("{}", input(options.input).to_uppercase()),
        Actions::Lowcase(options) => println!("{}", input(options.input).to_lowercase()),
        Actions::Random(options) => println!("{}", random_str(options.length.unwrap_or(8)))
    }

    Ok(())
}

fn input(s: Option<String>) -> String {
    s.unwrap_or_else(|| {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("read line");
        buf
    })
}