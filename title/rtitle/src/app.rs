use chrono::{DateTime, Local};

use rtitle::{ExtractError, Title};

const VERSION: &str = "0.3.2-20240719.01";

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
