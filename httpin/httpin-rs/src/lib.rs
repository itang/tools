#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate httpin-rs.
//!
//! add doc here

use clap::Parser;

/// Args
#[derive(Debug, Parser)]
pub struct Args {
    ///host
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value_t = 3000)]
    ///port
    port: u16,
}

impl Args {
    ///from parse
    pub fn from_parse() -> Self {
        Args::parse()
    }

    ///address
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    ///as url
    pub fn as_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}
