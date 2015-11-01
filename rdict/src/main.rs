#![feature(drain)]

// extern crate url;
extern crate rustc_serialize;

#[macro_use]
extern crate hyper;

use std::io;
use std::io::Read;
use std::env;

use rustc_serialize::json;
// use url::form_urlencoded;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::Headers;
use hyper::client::Body::BufBody;

header! { (Auth, "Auth") => [String] }

#[derive(RustcDecodable, RustcEncodable)]
struct TransResult {
    to: String,
    from: String,
}

fn main() {
    if env::args().len() > 1 {
        process_word(env::args().nth(1));
    } else {
        loop {
            println!("Please input word:");
            let mut word = String::new();
            let ret = io::stdin().read_line(&mut word).ok();
            process_word(if ret.is_some() {
                Some(word)
            } else {
                None
            });
            println!("-------------------------------------");
        }
    }
}

fn process_word(input: Option<String>) {
    if let Some(word_input) = input {
        let word = word_input.trim().to_string();
        println!("{}:", word);
        let word_clone = word.clone();
        match dict(word) {
            Some(trans) => {
                println!("\t->: {}", trans);

                println!("\ntry post to cloud...");
                let resp = post_to_cloud(&TransResult {
                    to: trans,
                    from: word_clone,
                });
                println!("\t->: {}", resp);
            }
            _ => println!("\tUnknown!"),
        }
    } else {
        println!("Please input word.")
    }
}

fn dict(word: String) -> Option<String> {
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

    let url = format!("http://dict.youdao.com/search?q={}&keyfrom=dict.index",
                      word);
    let content = http_get_as_string(&url);

    extract_ret(content)
}


fn post_to_cloud(tr: &TransResult) -> String {
    let http_post_as_string = |url: String| -> String {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set(Auth("test;test2015".to_owned()));

        // let body_str = form_urlencoded::serialize(vec!(("from", from), ("to", to)));
        let body_str = json::encode(tr).unwrap();
        let bytes = body_str.as_bytes();
        let length = bytes.len();
        let mut res = client.post(&url)
                            .body(BufBody(bytes, length))
                            .headers(headers)
                            .send()
                            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    };

    let ret = http_post_as_string("http://www.haoshuju.net:8000/api/dict".to_string());
    // let ret = http_post_as_string("http://localhost:8000/api/dict".to_string());
    ret
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_dict() {
    assert_eq!(dict("hello".to_string()),
               Some("int. 喂；哈罗".to_string()));
}
