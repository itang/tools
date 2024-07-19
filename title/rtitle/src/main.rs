#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use std::env;

mod app;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match &args[..] {
        [] => {
            println!("Please input the url.");
            app::print_help();
        },
        [head, ..] if head == "-h" || head == "--help" => app::print_help(),
        [head, ..] if head == "-v" || head == "--version" => app::print_version(),
        urls => {
            for url in urls {
                app::print_title(url).await?;
            }
        },
    }

    Ok(())
}
