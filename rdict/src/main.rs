#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate ansi_term;

use std::error::Error;
use std::io;
use std::env;

use ansi_term::Colour;

mod util;

#[derive(Serialize, Deserialize, Debug)]
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

    match word_from_args() {
        Some(ref word) => process_word(word),
        None => {
            let mut count = 0;
            loop {
                count += 1;
                println!("[{}]Please input word:", count);
                match word_from_input() {
                    Some(ref v) if *v == ":quit" || *v == ":q" => break,
                    Some(ref v) => process_word(v),
                    None => println!("Please input word."),
                }
                println!("-------------------------------------");
            }
        }
    }
}

fn process_word(word: &str) {
    println!("{}:", Colour::Green.paint(word));

    match dict(word) {
        Ok(trans) => {
            println!("\t->: {}", Colour::Blue.paint(trans.as_ref()));

            println!("\ntry post to cloud...");
            match post_to_cloud(&TransResult {
                to: trans,
                from: word.to_string(),
            }) {
                Ok(resp) => println!("\t->: {}", resp),
                Err(err) => println!("error: {}", err),
            }
        }
        Err(err) => println!("\terror: {}", err),
    }
}

fn dict(word: &str) -> Result<String, Box<Error>> {
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

    util::http_get_as_string(&url).and_then(|content| {
        extract_result(content).ok_or(From::from("无法解析获取翻译内容"))
    })
}

const MAX_TO_CHARS: usize = 100;

fn post_to_cloud(tr: &TransResult) -> Result<String, Box<Error>> {
    if tr.to.len() > MAX_TO_CHARS {
        let msg = format!("Too large content({} chars), ignore to post!", tr.to.len());
        println!("INFO: {}", msg);
        return Err(From::from(msg));
    }

    util::http_post_as_string("http://dict.godocking.com/api/dict/logs", tr)
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_dict() {
    assert_eq!(dict("hello").unwrap(), "int. 喂；哈罗".to_string());
}
