#![feature(drain)]

extern crate hyper;

use std::io::Read;
use std::env;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    if let Some(word) = get_word() {
        println!("{}:", word);
        match dict(word) {
            Some(ret) => println!("\t->: {}", ret),
            _ => println!("\tUnknown!"),
        }
    } else {
        println!("Please input the word.");
    }
}

fn get_word() -> Option<String> {
    env::args().nth(1)
}

fn dict(word: String) -> Option<String> {
    let url = format!("http://dict.youdao.com/search?q={}&keyfrom=dict.index",
                      word);
    let content = http_get_as_string(url);
    extract_title(content)
}

fn extract_title(mut content: String) -> Option<String> {
    if let Some(p1) = content.find("trans-container") {
        content.drain(..p1);
        if let Some(p2) = content.find("<li>") {
            content.drain(..p2);
            let (end, start) = (content.find("</li>").unwrap(), "</li>".len() - 1);
            return Some(content.drain(start..end).collect());
        }
    }
    return None;
}

fn http_get_as_string(url: String) -> String {
    let client = Client::new();

    // Creating an outgoing request.
    // &String can automatically coerce to a &str.
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send()
                        .unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_dict() {
    assert_eq!(dict("hello".to_string()),
               Some("int. 喂；哈罗".to_string()));
}
