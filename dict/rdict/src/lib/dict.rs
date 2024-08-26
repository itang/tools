use crate::util;
use std::error::Error;
use std::fmt::{Display, Formatter};

///Dict Result
#[derive(Debug)]
pub struct DictResult {
    result: String,
    pronunciation: String,
}

impl DictResult {
    pub fn new(result: String, pronunciation: String) -> Self {
        Self { result, pronunciation }
    }
}

impl Display for DictResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.result, self.pronunciation)
    }
}

/// dict
pub async fn dict(word: &str) -> Result<DictResult, Box<dyn Error>> {
    // content: owned move ?
    //TODO: extract pronunciation
    fn extract_result(mut content: String) -> Option<DictResult> {
        if let Some(p1) = content.find("trans-container") {
            content.drain(..p1);
            if let Some(p2) = content.find("<li>") {
                content.drain(..p2);
                let (end, start) = (content.find("</li>").expect("find"), "</li>".len() - 1);
                return Some(DictResult::new(content.drain(start..end).collect(), "".to_string()));
            }
        }

        None
    }

    let url = format!("http://dict.youdao.com/search?q={}&keyfrom=dict.index" /*DICT_SERVICE_URL*/, word);

    util::http_get_as_string(&url)
        .await
        .and_then(|content| extract_result(content).ok_or_else(|| From::from("无法解析获取翻译内容")))
}

/// ////////////////////////////////////////////////////////////////////////////
#[tokio::test]
async fn test_dict() {
    assert!(dict("hello").await.expect("ok").result.contains("你好"));
}
