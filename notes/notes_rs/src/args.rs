use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    ///Account
    Account(AccountOptions),
}

///Command1 Options
#[derive(Args, Debug, Clone)]
pub struct AccountOptions {}
