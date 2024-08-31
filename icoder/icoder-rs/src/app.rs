use clap::{Args, Parser, Subcommand};

use icoder::*;


#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

impl CliArgs {
    ///get 
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    ///base64
    Base64(CoderOptions),
    ///hex
    Hex(CoderOptions),
    ///i2hex
    I2hex(CoderOptions),
    ///i2binary
    I2binary(CoderOptions),
    ///upcase
    Upcase(UpcaseOptions),
    ///lowcase
    Lowcase(LowcaseOptions),
    ///uuid
    Uuid(UuidOptions),
    ///random
    Random(RandomOptions),
    ///md5
    Md5(Md5Options),
    ///now
    Now,
    ///url
    Url(UrlOptions),
}

///i2binary
#[derive(Args, Debug, Clone)]
pub struct CoderOptions {
    ///input
    pub input: Option<String>,
    ///decode
    #[arg(short, long)]
    pub decode: bool,
}

///upcase
#[derive(Args, Debug, Clone)]
pub struct UpcaseOptions {
    ///input
    pub input: Option<String>,
}

///lowcase
#[derive(Args, Debug, Clone)]
pub struct LowcaseOptions {
    ///input
    pub input: Option<String>,
}

///random
#[derive(Args, Debug, Clone)]
pub struct RandomOptions {
    ///length
    pub length: Option<usize>,
}

///uuid options
#[derive(Args, Debug, Clone)]
pub struct UuidOptions {
    ///upcase
    #[arg(short, long)]
    pub upcase: bool,
    #[arg(short, long)]
    pub no_underline: bool,
}

///i2binary
#[derive(Args, Debug, Clone)]
pub struct Md5Options {
    ///input
    pub input: Option<String>,
}


/// url options
#[derive(Args, Debug, Clone)]
pub struct UrlOptions {
    ///input
    pub input: Option<String>,
    ///query mode
    #[arg(short='m', long, value_enum, default_value_t=url::QueryMode::Qsl)]
    pub query_mode: url::QueryMode,
}

///IRouter
pub trait IRouter {
    type ARGS;
    fn dispatch(&self, args: Self::ARGS) -> Result<(), Box<dyn std::error::Error>>;
}

///Router
#[derive(Debug, Clone, Default)]
pub struct Router {}


impl IRouter for Router {
    type ARGS = CliArgs;

    /// run
    fn dispatch(&self, args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
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
            Command::Url(options) => url::parse_url(&options.input.or_read_line(), options.query_mode)?.to_pretty_string()?
        };

        println!("{}", ret);

        Ok(())
    }
}
