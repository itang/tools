use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The access log file path
    #[arg(short = 'f', long, default_value = "access.log-20240304")]
    pub access_log_file: PathBuf,

    /// The api pattern
    #[arg(short = 'a', long, default_value = "/getClient")]
    pub api_name: String,

    /// Debug mode
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Percent Value List
    #[arg(short = 'p', long, value_delimiter = ',', default_value = "0.995,0.99,0.98,0.97,0.96,0.95,0.90,0.75")]
    pub pv_list: Vec<f64>,
}
