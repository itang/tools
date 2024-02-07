#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::Parser;
use std::error::Error;

use icoder_rs::*;
use cli::*;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let ret = match args.action {
        Action::Base64(options) => {
            if options.decode {
                Base64.decode(input(options.input))?
            } else {
                Base64.encode(input(options.input))?
            }
        }
        Action::Hex(options) => {
            if options.decode {
                Hex.decode(input(options.input))?
            } else {
                Hex.encode(input(options.input))?
            }
        }
        Action::I2hex(options) => {
            if options.decode {
                I2Hex.decode(input(options.input))?
            } else {
                I2Hex.encode(input(options.input))?
            }
        }
        Action::Uuid => uuid(),
        Action::Upcase(options) => input(options.input).to_uppercase(),
        Action::Lowcase(options) => input(options.input).to_lowercase(),
        Action::Random(options) => random_str(options.length.unwrap_or(8)),
    };

    println!("{}", ret);

    Ok(())
}

fn input(s: Option<String>) -> String {
    s.unwrap_or_else(|| {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("read line");
        buf.trim().to_string()
    })
}
