use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::util;
use scraper::{Html, Selector};

//Tran Result
#[derive(Debug)]
pub struct TranResult {
    pub part_of_speech: String,
    pub explanation: String,
}

impl Display for TranResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.part_of_speech, self.explanation)
    }
}

//Pronunciation Result
#[derive(Debug)]
pub struct PronResult {
    pub alias: String,
    pub pronunciation: String,
}

impl Display for PronResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.alias, self.pronunciation)
    }
}

///Dict Result
#[derive(Debug)]
pub struct DictResult {
    pub result: Vec<TranResult>,
    pub pronunciation: Vec<PronResult>,
}

/// dict
pub async fn dict(word: &str) -> Result<DictResult, Box<dyn Error>> {
    //TODO: 优化抽取结果
    fn extract_result(content: &str) -> Option<DictResult> {
        let document = Html::parse_document(content);

        let span_trans_selector = Selector::parse("div.trans-container li.word-exp").expect("selector parse");
        let result: Vec<TranResult> = document
            .select(&span_trans_selector)
            .map(|e| {
                let s1 = Selector::parse("span.pos").expect("selector parse");
                let part_of_speech = e.select(&s1).map(|it| it.inner_html()).collect::<Vec<String>>().join("");
                let s2 = Selector::parse("span.trans").expect("selector parse");
                let explanation = e.select(&s2).map(|it| it.inner_html()).collect::<Vec<String>>().join("");
                TranResult { part_of_speech, explanation }
            })
            .collect();

        let phone_selector = Selector::parse("div.trans-container div.per-phone").expect("selector parse");
        let pronunciation: Vec<PronResult> = document
            .select(&phone_selector)
            .map(|e| {
                let s1 = Selector::parse(":first-child").expect("selector parse");
                let alias = e.select(&s1).map(|it| it.inner_html()).collect::<Vec<String>>().join("");
                let s2 = Selector::parse("span.phonetic").expect("selector parse");
                let pronunciation = e.select(&s2).map(|it| it.inner_html()).collect::<Vec<String>>().join("");
                PronResult { alias, pronunciation }
            })
            .collect();

        Some(DictResult { result, pronunciation })
    }

    let url = format!("https://dict.youdao.com/result?word={}&lang=en" /*DICT_SERVICE_URL*/, word);

    util::http_get_as_string(&url)
        .await
        .and_then(|content| extract_result(&content).ok_or_else(|| From::from("无法解析获取翻译内容")))
}

/// ////////////////////////////////////////////////////////////////////////////
#[tokio::test]
async fn test_dict() {
    assert!(dict("hello").await.expect("ok").result.iter().any(|it| it.explanation.contains("你好")));
}
