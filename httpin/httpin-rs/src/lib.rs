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
        let args = Args::parse();
        if args.listen_all() && !args.host.is_empty() {
            tracing::info!("ignore the `host` args");
        }

        args
    }

    ///address
    pub fn address(&self) -> String {
        format!("{}:{}", self.host(), self.port)
    }

    ///as url
    pub fn as_url(&self) -> String {
        let mut url = String::new();

        let host = self.host();
        if host == "0.0.0.0" {
            url.push_str(&format!("http://127.0.0.1:{}", self.port));
            match local_ip_address::local_ip() {
                Ok(ip) => {
                    url.push_str(", ");
                    url.push_str(&format!("http://{}:{}", ip, self.port))
                },
                Err(err) => {
                    tracing::warn!("can't get local ip, error: {:?}", err);
                },
            }
        } else {
            url.push_str(&format!("http://{}:{}", host, self.port))
        }

        url
    }

    fn listen_all(&self) -> bool {
        matches!(self.listen_all, Some(None) | Some(Some(true)))
    }

    fn host(&self) -> &str {
        if self.listen_all() {
            "0.0.0.0"
        } else {
            &self.host
        }
    }
}
