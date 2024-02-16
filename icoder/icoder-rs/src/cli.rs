use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
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
    ///now
    Now,
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
