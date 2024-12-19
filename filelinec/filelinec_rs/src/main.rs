#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use filelinec::app::{App, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    debug_config(&config);
    println!("{}", "-".repeat(80));

    App::new(config).run()
}

fn debug_config(config: &Config) {
    println!("config: {config:?}");
}
