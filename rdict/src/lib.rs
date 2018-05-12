#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::error::Error;

mod util;

//const DICT_SERVICE_URL: &'static str = "http://dict.youdao.com/search?q={}&keyfrom=dict.index";

#[derive(Serialize, Deserialize, Debug)]
pub struct TransResult {
    pub to: String,
    pub from: String,
}

pub fn dict(word: &str) -> Result<String, Box<Error>> {
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

    let url = format!(
        "http://dict.youdao.com/search?q={}&keyfrom=dict.index" /*DICT_SERVICE_URL*/,
        word
    );

    util::http_get_as_string(&url).and_then(|content| {
        extract_result(content).ok_or(From::from("无法解析获取翻译内容"))
    })
}

const MAX_TO_CHARS: usize = 100;

pub fn post_to_cloud(upstream_url: &str, tr: &TransResult) -> Result<String, Box<Error>> {
    if tr.to.len() > MAX_TO_CHARS {
        let msg = format!("Too large content({} chars), ignore to post!", tr.to.len());
        println!("INFO: {}", msg);
        return Err(From::from(msg));
    }

    util::http_post_as_string(upstream_url, tr)
}

/// ////////////////////////////////////////////////////////////////////////////
#[test]
fn test_dict() {
    assert_eq!(dict("hello").unwrap(), "int. 喂；哈罗".to_string());
}
