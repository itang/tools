#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use idownload::app::{App, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new(Config::parse()).run().await
}
