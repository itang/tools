#![feature(drain)]

// extern crate url;
extern crate chrono;

#[macro_use]
extern crate hyper;

use std::io::Read;
use std::env;

use chrono::*;

// use url::form_urlencoded;
use hyper::Client;
use hyper::header::Connection;

fn main() {
    if let Some(url) = get_url() {
        println!("{} ->:", url);
        let url_clone = url.clone();
        match title(url) {
            Some(ret) => {
                let local: DateTime<Local> = Local::now();
                let now = local.format("%Y-%m-%d %H:%M").to_string();

                println!("\nrs << Read.new \"{}\",\n  title: \"{}\",\n  created_at: \"{}\"",
                         url_clone,
                         ret.trim(),
                         now);
            }
            _ => println!("\tUnknown!"),
        }
    } else {
        println!("Please input the url.");
    }
}

fn get_url() -> Option<String> {
    env::args().nth(1)
}

fn title(url: String) -> Option<String> {
    fn http_get_as_string(url: &String) -> String {
        let client = Client::new();

        // Creating an outgoing request.
        // &String can automatically coerce to a &str.
        let mut res = client.get(url)
                            .header(Connection::close())
                            .send()
                            .unwrap();

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn extract_ret(mut content: String) -> Option<String> {
        if let Some(p1) = content.find("<title>") {
            content.drain(..p1);
            if let Some(p2) = content.find("</title>") {
                let (start, end) = ("<title>".len(), p2);
                return Some(content.drain(start..end).collect());
            }
        }
        return None;
    }

    let content = http_get_as_string(&url);

    extract_ret(content)
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_title() {
    assert_eq!(title("http://www.baidu.com".to_string()),
               Some("百度一下，你就知道".to_string()));
}
