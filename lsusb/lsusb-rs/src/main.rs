#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

mod app;
mod router;

use self::router::{Args, Router};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Router::new(Args::get()).run()?;

    Ok(())
}
