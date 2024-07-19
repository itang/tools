#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use std::env;

use rtitle::print_title;

const VERSION: &str = "0.3.2-20240719.01";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match &args[..] {
        [] => {
            println!("Please input the url.");
            print_help();
        }
        [head, ..] if head == "-h" || head == "--help" => print_help(),
        [head, ..] if head == "-v" || head == "--version" => print_version(),
        urls => {
            for url in urls {
                print_title(url).await;
            }
        }
    }
}

fn print_version() {
    println!("rtitle-v{VERSION}");
}

fn print_help() {
    println!("e.g.\n  rtitle <url>")
}
