#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

mod app;

use crate::app::{Args, Router};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Router::new(Args::get()).run().map_err(|e| e.into())
}
