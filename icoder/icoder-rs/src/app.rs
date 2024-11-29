use clap::{Args, Parser, Subcommand};
use icoder::*;
use std::time::Duration;

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
    // ///i2hex
    // I2hex(CoderOptions),
    // ///i2binary
    // I2binary(CoderOptions),
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
    ///split
    Split(SplitOptions),
    ///delay
    Delay(DelayOptions),
    ///Num
    Num(NumOptions),
    ///Nums
    Nums(NumsOptions),
    ///Pows
    Pows(PowsOptions),
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

/// split options
#[derive(Args, Debug, Clone)]
pub struct SplitOptions {
    ///input
    pub input: Option<String>,
    ///whitespace
    #[arg(short = 'w', long, default_value_t = true)]
    pub whitespace: bool,
}

///delay
#[derive(Args, Debug, Clone)]
pub struct DelayOptions {
    ///input
    pub input: Option<u32>,
}
///nums
#[derive(Args, Debug, Clone)]
pub struct NumsOptions {}

///pows
#[derive(Args, Debug, Clone)]
pub struct PowsOptions {}

#[derive(Args, Debug, Clone)]
pub struct NumOptions {
    pub input: Option<String>,
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
            },
            Command::Hex(options) => {
                if options.decode {
                    Hex.decode(options.input.or_read_line())?
                } else {
                    Hex.encode(options.input.or_read_line())?
                }
            },
            // Command::I2hex(options) => {
            //     if options.decode {
            //         I2Hex.decode(options.input.or_read_line())?
            //     } else {
            //         I2Hex.encode(options.input.or_read_line())?
            //     }
            // },
            // Command::I2binary(options) => {
            //     if options.decode {
            //         I2Binary.decode(options.input.or_read_line())?
            //     } else {
            //         I2Binary.encode(options.input.or_read_line())?
            //     }
            // },
            Command::Uuid(options) => uuid(options.upcase, options.no_underline),
            Command::Upcase(options) => options.input.or_read_line().to_uppercase(),
            Command::Lowcase(options) => options.input.or_read_line().to_lowercase(),
            Command::Random(options) => random_str(options.length.unwrap_or(8)),
            Command::Md5(options) => md5(options.input.or_read_line()),
            Command::Now => now("%Y-%m-%d %H:%M:%S"),
            Command::Url(options) => {
                url::parse_url(&options.input.or_read_line(), options.query_mode)?.to_pretty_string()?
            },
            Command::Split(options) => {
                let s = options.input.or_read_line();
                if options.whitespace {
                    split_string_whitespace(&s)
                } else {
                    s
                }
            },
            Command::Delay(options) => {
                let value: String = options.input.map(|it| it.to_string()).or_read_line();
                let value: u64 = value.parse()?;
                std::thread::sleep(Duration::from_millis(value));

                String::default()
            },
            Command::Num(options) => {
                let raw_s = options.input.or_read_line();
                let value = num_from_string(&raw_s)?;

                to_num_string(&raw_s, value)
            },
            Command::Nums(_options) => {
                let ds = [
                    "0xFFFF",
                    "0x8000",
                    "0xFFF",
                    "0xFF",
                    "0xF",
                    "0x1111",
                    "0x1000",
                    "0x111",
                    "0x11",
                    "0x1",
                    "0b1",
                    "0b11",
                    "0b111",
                    "0b1111",
                    "0b11111",
                    "0b1111111",
                    "0b11111111",
                    "0b111111111",
                ];
                ds.into_iter()
                    .map(|raw_s| {
                        let value = num_from_string(raw_s).expect("failed to parse num");
                        to_num_string(raw_s, value)
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            },
            Command::Pows(_options) => (0..20)
                .map(|i| {
                    let value = u64::pow(2, i);
                    let s = format!("pow(2, {})", i);
                    to_num_string(&s, value)
                })
                .collect::<Vec<String>>()
                .join("\n"),
        };

        println!("{}", ret);

        Ok(())
    }
}

fn num_from_string(raw_s: &str) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(match raw_s {
        hex_string if hex_string.starts_with("0x") || hex_string.starts_with("0X") => {
            let trimmed = &hex_string[2..];
            u64::from_str_radix(trimmed, 16)?
        },
        binary_string if binary_string.starts_with("0o") || binary_string.starts_with("0o") => {
            let trimmed = &binary_string[2..];
            u64::from_str_radix(trimmed, 8)?
        },
        binary_string if binary_string.starts_with("0b") || binary_string.starts_with("0b") => {
            let trimmed = &binary_string[2..];
            u64::from_str_radix(trimmed, 2)?
        },
        _ => {
            let trimmed = raw_s;
            u64::from_str_radix(trimmed, 10)?
        },
    })
}

fn to_num_string(raw_s: &str, value: u64) -> String {
    format!("{:<12} = {:<8} 0x{:<8x} 0o{:<10o} 0b{:b}", raw_s, value, value, value, value)
}
