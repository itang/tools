#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use anyhow::Result;

use crate::app::{Args, Router};

mod app;

fn main() -> Result<()> {
    Router::new(Args::get()).run()
}
