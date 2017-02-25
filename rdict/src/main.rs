#[macro_use]
extern crate serde_derive;
//extern crate serde_json;
extern crate reqwest;
extern crate ansi_term;

use std::io;
use std::io::Read;
use std::env;

use ansi_term::Colour;


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
            Ok(trans) => {
                println!("\t->: {}", Colour::Blue.paint(trans.as_ref()));

                println!("\ntry post to cloud...");
                match post_to_cloud(&TransResult {
                    to: trans,
                    from: word,
                }) {
                    Ok(resp) => println!("\t->: {}", resp),
                    Err(err) => println!("error: {}", err),
                }
            }
            Err(err) => println!("\terror: {}", err),
        }
    } else {
        println!("Please input word.")
    }
}

fn dict(word: &str) -> Result<String, String> {
    fn http_get_as_string(url: &str) -> Result<String, String> {
        let client = reqwest::Client::new().unwrap();
        let mut res = client.get(url).send().map_err(|x| format!("{}", x))?;

        // Read the Response.
        let mut body = String::new();

        res.read_to_string(&mut body).map_err(|x| format!("{}", x))?;

        Ok(body)
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
    let content = http_get_as_string(&url)?;

    extract_result(content).ok_or("无法解析获取翻译内容".to_string())
}

const MAX_TO_CHARS: usize = 100;

fn post_to_cloud(tr: &TransResult) -> Result<String, String> {
    if tr.to.len() > MAX_TO_CHARS {
        let msg = format!("Too large content({} chars), ignore to post!", tr.to.len());
        println!("INFO: {}", msg);
        return Err(msg);
    }

    let http_post_as_string = |url: &str| -> Result<String, String> {
        let client = reqwest::Client::new().unwrap();
        let mut res = client.post(url)
            .form(&tr)
            .send()
            .map_err(|x| format!("{}", x))?;

        let mut body = String::new();

        res.read_to_string(&mut body).map_err(|x| format!("{}", x))?;

        Ok(body)
    };

    http_post_as_string("http://dict.godocking.com/api/dict/logs")
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_dict() {
    assert_eq!(dict("hello"), Some("int. 喂；哈罗".to_string()));
}
