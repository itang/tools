use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    ///command1
    Account(AccountOptions),
    ///command2
    Command2(Command2Options),
}

///Command1 Options
#[derive(Args, Debug, Clone)]
pub struct AccountOptions {}

///Command2 Options
#[derive(Args, Debug, Clone)]
pub struct Command2Options {
    ///input
    pub input: Option<String>,
}
