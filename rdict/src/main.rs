#![feature(drain)]

extern crate hyper;

use std::io::Read;
use std::env;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    let word = get_word();
    if word.is_empty() {
        println!("Please input the word.");
        return;
    }

    println!("{}:", word);
    let ret = dict(word);
    println!("\t->: {}", ret);
}

fn get_word() -> String {
    if let Some(arg1) = env::args().nth(1) {
        arg1
    }else{
        "".to_string()
    }
}

fn dict(word: String) -> String {
    let url = format!("http://dict.youdao.com/search?q={}&keyfrom=dict.index",
                      word);
    let mut content = http_get_as_string(url);
    if !content.is_empty() {
        match content.find("trans-container") {
            Some(offset) => {
                content.drain(..offset);
                match content.find("<li>") {
                    Some(value1) => {
                        content.drain(..value1);
                        let end = content.find("</li>").unwrap();
                        let start = "</li>".len() - 1;
                        return content.drain(start..end).collect();
                    },
                    _ => (),
                };
            },
            _ => (),
        }
    }
    return "".to_string();
}

fn http_get_as_string(url: String) -> String {
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send()
                        .unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

#[test]
fn test_dict() {
    assert_eq!(dict("hello".to_string()), "int. 喂；哈罗".to_string());
}
