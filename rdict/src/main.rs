// #![feature(drain)]

// extern crate url;
extern crate rustc_serialize;

#[macro_use]
extern crate hyper;

extern crate ansi_term;

use std::io;
use std::io::Read;
use std::env;

use ansi_term::Colour;
use rustc_serialize::json;
use hyper::Client;
use hyper::header::{Connection, Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::client::Body::BufBody;


header! { (Auth, "Auth") => [String] }

#[derive(RustcDecodable, RustcEncodable)]
struct TransResult {
    to: String,
    from: String,
}

fn main() {
    fn word_from_args() -> Option<String> {
        env::args().nth(1)
    }

    fn word_from_input() -> Option<String> {
        let mut word = String::new();
        let ret = io::stdin().read_line(&mut word);
        if ret.is_ok() && word.trim().len() > 0 {
            return Some(word.trim().to_string());
        }
        return None;
    }

    if env::args().len() > 1 {
        process_word(word_from_args());
    } else {
        let mut count = 0;
        loop {
            count += 1;
            println!("[{}]Please input word:", count);
            match word_from_input() {
                Some(ref v) if *v == ":quit" || *v == ":q" => break,
                it => process_word(it),
            }
            println!("-------------------------------------");
        }
    }
}

fn process_word(input: Option<String>) {
    if let Some(word_input) = input {
        let word = word_input.trim().to_string();
        println!("{}:", word);
        match dict(&word) {
            Some(trans) => {
                println!("\t->: {}", Colour::Blue.paint(trans.as_ref()));

                println!("\ntry post to cloud...");
                let resp = post_to_cloud(&TransResult {
                    to: trans,
                    from: word,
                });
                println!("\t->: {}", resp);
            }
            _ => println!("\tUnknown!"),
        }
    } else {
        println!("Please input word.")
    }
}

fn dict(word: &str) -> Option<String> {
    fn http_get_as_string(url: &str) -> String {
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

    // content: owned move ?
    fn extract_result(mut content: String) -> Option<String> {
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

    extract_result(content)
}

fn post_to_cloud(tr: &TransResult) -> String {
    let http_post_as_string = |url: &str| -> String {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set(Auth("test;test2015".to_owned()));
        headers.set(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));

        let body_str = json::encode(tr).unwrap();
        let bytes = body_str.as_bytes();
        let length = bytes.len();
        let mut res = client.post(url)
            .body(BufBody(bytes, length))
            .headers(headers)
            .send()
            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    };

    let ret = http_post_as_string("http://www.haoshuju.net:9800/dict/logs");

    ret
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_dict() {
    assert_eq!(dict("hello"), Some("int. 喂；哈罗".to_string()));
}
