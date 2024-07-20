use rtitle::ExtractError;

use self::handlers::*;

const VERSION: &str = "0.3.2-20240719.01";

/// The router
pub struct Router {
    args: Vec<String>,
}

impl Router {
    /// create a new router
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    /// run the router
    pub async fn run(&self) -> Result<(), ExtractError> {
        match &self.args[..] {
            [] => {
                println!("Please input the url.");
                print_help();
            },
            [head, ..] if head == "-h" || head == "--help" => print_help(),
            [head, ..] if head == "-v" || head == "--version" => print_version(),
            urls => {
                for url in urls {
                    print_title(url).await?;
                }
            },
        }

        Ok(())
    }
}

mod handlers {
    use chrono::{DateTime, Local};
    use rtitle::{ExtractError, Title};

    use super::VERSION;

    pub(crate) fn print_version() {
        println!("rtitle-v{VERSION}");
    }

    pub(crate) fn print_help() {
        println!("e.g.\n  rtitle <url>")
    }

    ///print title of the url
    pub(crate) async fn print_title(url: &str) -> Result<(), ExtractError> {
        let title = Title::extract_from_url(url).await?;

        let local: DateTime<Local> = Local::now();
        let now = local.format("%Y-%m-%d %H:%M");

        println!("\nrs << Read.new \"{url}\",\n  title: \"{}\",\n  created_at: \"{now}\"\n", title);

        Ok(())
    }
}
