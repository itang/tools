use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The access log file path
    #[arg(short, long, default_value = "access.log-20240304")]
    pub file: PathBuf,

    /// The api pattern
    #[arg(short = 'a', long, default_value = "/getClient")]
    pub api_name: String,

    /// Debug mode
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}
