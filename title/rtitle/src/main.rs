use chrono::*;
use std::env;

struct Item {
    url: String,
    title: String,
}

impl Item {
    fn new(url: String, title: String) -> Self {
        Item { url, title }
    }
}

const VERSION: &str = "0.3.1-20220203";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match &args[..] {
        [] => {
            println!("Please input the url.");
            print_help();
        }
        [head, ..] if head == "-h" || head == "--help" => print_help(),
        [head, ..] if head == "-v" || head == "--version" => print_version(),
        [url, ..] => rtitle(url),
    }
}

fn print_version() {
    println!("rtitle-v{VERSION}");
}

fn print_help() {
    println!("e.g.\n  rtitle <url>")
}

fn rtitle(url: &str) {
    let ret = title(url);
    match ret {
        Ok(Item { url, title }) => {
            let local: DateTime<Local> = Local::now();
            let now = local.format("%Y-%m-%d %H:%M");

            println!(
                "\nrs << Read.new \"{url}\",\n  title: \"{title}\",\n  created_at: \"{now}\"\n"
            );
        }
        Err(e) => println!("\t{}", e),
    }
}

/*
fn url_from_args() -> Result<String, String> {
    env::args()
        .nth(1)
        .ok_or_else(|| "Please input the url.".to_string())
}*/

fn title(url: &str) -> Result<Item, String> {
    fn http_get_as_string(url: &str) -> reqwest::Result<String> {
        reqwest::blocking::get(url)?.text()
    }

    fn extract_ret(mut content: String) -> Result<String, String> {
        content
            .find("<title>")
            .and_then(|p1| {
                content.drain(..p1);
                content.find("</title")
            })
            .map(|p2| {
                let (start, end) = ("<title>".len(), p2);
                let title: String = content.drain(start..end).collect();
                let title = title.trim().to_string();
                title
            })
            .ok_or_else(|| "无法解析html".to_string())
    }

    http_get_as_string(url)
        .map_err(|err| format!("error: {:?}", err))
        .and_then(extract_ret)
        .map(|title| Item::new(url.to_string(), title))
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_title() {
    assert_eq!(
        title("http://www.baidu.com").unwrap().title,
        "百度一下，你就知道".to_string()
    );
}
