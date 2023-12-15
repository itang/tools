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
    ///listen host
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    ///listen on all interfaces
    #[arg(short = 'A', long)]
    listen_all: Option<Option<bool>>,

    #[arg(short, long, default_value_t = 3000)]
    ///listen port
    port: u16,
}

impl Args {
    ///from parse
    pub fn from_parse() -> Self {
        Args::parse()
    }

    ///address
    pub fn address(&self) -> String {
        format!("{}:{}", self.host(), self.port)
    }

    ///as url
    pub fn as_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    fn host(&self) -> &str {
        match self.listen_all {
            Some(None) | Some(Some(true)) => "0.0.0.0",
            _ => &self.host,
        }
    }
}
