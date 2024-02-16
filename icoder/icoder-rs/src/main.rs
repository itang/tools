#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::Parser;
use std::error::Error;

use crate::cli::*;
use icoder::*;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let ret = match args.action {
        Action::Base64(options) => {
            if options.decode {
                Base64.decode(options.input.or_read_line())?
            } else {
                Base64.encode(options.input.or_read_line())?
            }
        }
        Action::Hex(options) => {
            if options.decode {
                Hex.decode(options.input.or_read_line())?
            } else {
                Hex.encode(options.input.or_read_line())?
            }
        }
        Action::I2hex(options) => {
            if options.decode {
                I2Hex.decode(options.input.or_read_line())?
            } else {
                I2Hex.encode(options.input.or_read_line())?
            }
        }
        Action::I2binary(options) => {
            if options.decode {
                I2Binary.decode(options.input.or_read_line())?
            } else {
                I2Binary.encode(options.input.or_read_line())?
            }
        }
        Action::Uuid(options) => uuid(options.upcase, options.no_underline),

        Action::Upcase(options) => options.input.or_read_line().to_uppercase(),
        Action::Lowcase(options) => options.input.or_read_line().to_lowercase(),
        Action::Random(options) => random_str(options.length.unwrap_or(8)),
        Action::Now => now("%Y-%m-%d %H:%M:%S"),
    };

    println!("{}", ret);

    Ok(())
}
