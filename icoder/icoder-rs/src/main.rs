#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

mod app;

use std::error::Error;

use app::{CliArgs, IRouter, Router};

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::get();

    let router = Router::default();

    router.dispatch(args)?;

    Ok(())
}
