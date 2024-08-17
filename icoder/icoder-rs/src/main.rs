#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::Parser;
use std::error::Error;

use crate::cli::*;
use icoder::*;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let ret = match args.command {
        Command::Base64(options) => {
            if options.decode {
                Base64.decode(options.input.or_read_line())?
            } else {
                Base64.encode(options.input.or_read_line())?
            }
        }
        Command::Hex(options) => {
            if options.decode {
                Hex.decode(options.input.or_read_line())?
            } else {
                Hex.encode(options.input.or_read_line())?
            }
        }
        Command::I2hex(options) => {
            if options.decode {
                I2Hex.decode(options.input.or_read_line())?
            } else {
                I2Hex.encode(options.input.or_read_line())?
            }
        }
        Command::I2binary(options) => {
            if options.decode {
                I2Binary.decode(options.input.or_read_line())?
            } else {
                I2Binary.encode(options.input.or_read_line())?
            }
        }
        Command::Uuid(options) => uuid(options.upcase, options.no_underline),
        Command::Upcase(options) => options.input.or_read_line().to_uppercase(),
        Command::Lowcase(options) => options.input.or_read_line().to_lowercase(),
        Command::Random(options) => random_str(options.length.unwrap_or(8)),
        Command::Md5(options) => md5(options.input.or_read_line()),
        Command::Now => now("%Y-%m-%d %H:%M:%S"),
    };

    println!("{}", ret);

    Ok(())
}
