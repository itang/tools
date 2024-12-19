#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use filelinec::app::{Config, Router};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Router::new(Config::parse()).run()
}
