#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use crate::app::{Args, Router};

use anyhow::Result;

mod app;

fn main() -> Result<()> {
    Router::new(Args::get()).run()
}
