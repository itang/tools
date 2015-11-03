#![feature(drain)]

extern crate chrono;

#[macro_use]
extern crate hyper;

use std::io::Read;
use std::env;

use chrono::*;

use hyper::Client;
use hyper::header::Connection;


struct Item {
    url: String,
    title: String,
}

impl Item {
    fn new(url: String, title: String) -> Self {
        Item {
            url: url,
            title: title,
        }
    }
}


fn main() {
    match url_from_args().and_then(|ref url| title(url)) {
        Ok(Item{url, title}) => {
            let local: DateTime<Local> = Local::now();
            let now = local.format("%Y-%m-%d %H:%M");

            println!("\nrs << Read.new \"{}\",\n  title: \"{}\",\n  created_at: \"{}\"\n",
                     url,
                     title,
                     now);
        }
        Err(e) => println!("\t{}", e),
    }
}

fn url_from_args() -> Result<String, String> {
    env::args().nth(1).ok_or("Please input the url.".to_owned())
}

fn title(url: &str) -> Result<Item, String> {
    fn http_get_as_string(url: &str) -> Result<String, String> {
        let client = Client::new();
        let mut res = try!(client.get(url)
                                 .header(Connection::close())
                                 .send()
                                 .map_err(|e| e.to_string()));

        let mut body = String::new();
        try!(res.read_to_string(&mut body).map_err(|e| e.to_string()));

        Ok(body)
    }

    fn extract_ret(mut content: String) -> Result<String, String> {
        content.find("<title>")
               .and_then(|p1| {
                   content.drain(..p1);
                   content.find("</title")
               })
               .and_then(|p2| {
                   let (start, end) = ("<title>".len(), p2);
                   // @TODO: the type of this value must be known in this context
                   let title: String = content.drain(start..end).collect();
                   let title = title.trim().to_string();
                   Some(title)
               })
               .ok_or("无法解析html".to_string())
    }

    http_get_as_string(url)
        .and_then(extract_ret)
        .and_then(|title| Ok(Item::new(url.to_string(), title)))
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_title() {
    assert_eq!(title("http://www.baidu.com").unwrap().title,
               "百度一下，你就知道".to_string());
}
