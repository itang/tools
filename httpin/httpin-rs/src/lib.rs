#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate httpin-rs.
//!
//! add doc here

use clap::Parser;

///Args
#[derive(Debug, Parser)]
pub struct Args {
    ///host
    #[arg(short = 'H', long)]
    host: Option<String>,

    #[arg(short, long)]
    ///port
    port: Option<u16>,
}

impl Args {
    /// port
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }

    ///host
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("0.0.0.0")
    }

    ///address
    pub fn address(&self) -> String {
        format!("{}:{}", self.host(), self.port())
    }

    /// as url
    pub fn as_url(&self) -> String {
        format!("http://localhost:{}", self.port())
    }

    /// from parse
    pub fn from_parse() -> Self {
        Args::parse()
    }
}
