#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use self::app::Router;
use self::args::CliArgs;

mod app;
mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Router::new(CliArgs::get()).run()
}
