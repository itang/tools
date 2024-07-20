#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

mod app;

use std::env;

use self::app::Router;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    Router::new(args).run().await?;

    Ok(())
}
