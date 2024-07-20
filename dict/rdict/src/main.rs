#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

mod app;

use std::error::Error;

use self::app::{Args, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    Router::new(Args::get()).run().await
}
