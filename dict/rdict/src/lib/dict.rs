use std::error::Error;
use std::fmt::{Display, Formatter};

use scraper::{Html, Selector};

use crate::util;

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
    fn extract_result(content: &str) -> Option<DictResult> {
        // let document = Html::parse_document(r#"<html><div class="trans-container"><span class="trans">你好</span></div></html>"#);
        // dbg!(document.html());

        let document = Html::parse_document(content);
        // let html = document.html();
        // let document = Html::parse_document(&html);

        let span_trans_selector = Selector::parse("div.trans-container span.trans").expect("selector parse");

        for element in document.select(&span_trans_selector) {
            dbg!(&element.value().name());
        }

        let values: Vec<String> = document
            .select(&span_trans_selector)
            .map(|e| {
                dbg!(&e);
                e.inner_html()
            })
            .collect();
        let r = values.join(", ");
        dbg!(&r);

        let phonetic_selector = Selector::parse("span.phonetic").expect("selector parse");
        let values: Vec<String> = document.select(&phonetic_selector).map(|e| e.inner_html()).collect();
        let p = values.join(", ");
        dbg!(&p);

        Some(DictResult::new(r, p))
    }

    let url = format!("http://dict.youdao.com/search?q={}&keyfrom=dict.index" /*DICT_SERVICE_URL*/, word);

    util::http_get_as_string(&url)
        .await
        .and_then(|content| extract_result(&content).ok_or_else(|| From::from("无法解析获取翻译内容")))
}

/// ////////////////////////////////////////////////////////////////////////////
#[tokio::test]
async fn test_dict() {
    assert!(dict("hello").await.expect("ok").result.contains("你好"));
}
