#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate httpin-rs.
//!
//! add doc here

use std::fmt::Display;
use clap::Parser;

/// httpin cli
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
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

    /// to links
    pub fn to_links(&self) -> String {
        let mut links = String::new();

        let host = self.host();
        if host == "0.0.0.0" {
            links.push_str(&("127.0.0.1", self.port).to_link());
            match local_ip_address::local_ip() {
                Ok(ip) => {
                    links.push_str(", ");
                    links.push_str(&(ip, self.port).to_link())
                }
                Err(err) => {
                    tracing::warn!("can't get local ip, error: {:?}", err);
                }
            }
        } else {
            links.push_str(&(host, self.port).to_link())
        }

        links
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

///As Url
pub trait ToLink {
    /// to link
    fn to_link(&self) -> String;
}

impl<T: Display> ToLink for (T, u16) {
    #[inline(always)]
    fn to_link(&self) -> String {
        format!("http://{}:{}", self.0, self.1)
    }
}
