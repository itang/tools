use std::str::FromStr;
use std::time::Duration;

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
    ///Pow
    Pow(PowOptions),
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

#[derive(Args, Debug, Clone)]
pub struct NumOptions {
    pub input: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct PowOptions {
    pub args: Vec<u32>,
}

///pows
#[derive(Args, Debug, Clone)]
pub struct PowsOptions {}
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
                let num = UNum::from_str(&raw_s)?;

                num.to_pretty_string()
            },
            Command::Nums(_options) => {
                let content = include_str!("nums.txt");

                content
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|raw_s| {
                        let raw_s = unwrap(raw_s);

                        let num = UNum::from_str(raw_s).expect("failed to parse num");
                        num.to_pretty_string()
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            },
            Command::Pow(options) => {
                let (e, exp) = match options.args.as_slice() {
                    [e, exp] => (*e, *exp),
                    [exp] => (2, *exp),
                    _ => Default::default(),
                };

                pow_s(e as u64, exp)
            },
            Command::Pows(_options) => (0..20).map(|i| pow_s(2, i)).collect::<Vec<String>>().join("\n"),
        };

        println!("{}", ret);

        Ok(())
    }
}

fn pow_s(e: u64, exp: u32) -> String {
    let s = format!("pow({}, {})", e, exp);
    let value = u64::pow(e, exp);
    format!("{:<16} = {:<16} 0x{:<16x} 0o{:<16o} 0b{:b}", s, value, value, value, value)
}

fn unwrap(raw_s: &str) -> &str {
    if raw_s.starts_with("\"") || raw_s.starts_with("'") {
        &raw_s[1..raw_s.len() - 1]
    } else {
        raw_s
    }
}
