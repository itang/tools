use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
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
