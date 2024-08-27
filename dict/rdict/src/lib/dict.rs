use std::error::Error;

use scraper::{ElementRef, Html, Selector};

use crate::util;

///Tran Result
#[derive(Debug)]
pub struct TranResult {
    ///part of speech
    pub part_of_speech: String,
    ///explanation
    pub explanation: String,
}

///Pronunciation Result
#[derive(Debug)]
pub struct PronResult {
    ///alias
    pub alias: String,
    ///pronunciation
    pub pronunciation: String,
}

///Dict Result
#[derive(Debug)]
pub struct DictResult {
    ///result
    pub result: Vec<TranResult>,
    ///pronunciation
    pub pronunciation: Vec<PronResult>,
}

/// dict
pub async fn dict(word: &str) -> Result<DictResult, Box<dyn Error>> {
    fn extract_result(content: &str) -> Option<DictResult> {
        let document = Html::parse_document(content);

        let result = parse_result(&document);

        let pronunciation = parse_pronunciation(&document);

        Some(DictResult { result, pronunciation })
    }

    let url = format!("https://dict.youdao.com/result?word={}&lang=en" /*DICT_SERVICE_URL*/, word);

    util::http_get_as_string(&url)
        .await
        .and_then(|content| extract_result(&content).ok_or_else(|| From::from("无法解析获取翻译内容")))
}

fn parse_to_string(element_ref: ElementRef, selector: &Selector) -> String {
    element_ref.select(selector).map(|it| it.inner_html()).collect::<Vec<String>>().join("")
}

fn parse_and_wrap(element_ref: ElementRef, selector: &Selector) -> String {
    parse_to_string(element_ref, selector).replace("&lt;", "<").replace("&gt;", ">")
}

fn parse_result(document: &Html) -> Vec<TranResult> {
    let span_trans_selector = Selector::parse("div.trans-container li.word-exp").expect("selector parse");

    let mut result: Vec<TranResult> = document
        .select(&span_trans_selector)
        .map(|e| {
            let s1 = Selector::parse("span.pos").expect("selector parse");
            let part_of_speech = parse_to_string(e, &s1);
            let s2 = Selector::parse("span.trans").expect("selector parse");
            let explanation = parse_and_wrap(e, &s2);

            TranResult { part_of_speech, explanation }
        })
        .collect();

    if result.is_empty() {
        let span_trans_selector = Selector::parse("div.trans-container li.word-exp-ce").expect("selector parse");
        result = document
            .select(&span_trans_selector)
            .map(|e| {
                let s1 = Selector::parse("span.index").expect("selector parse");
                let index = parse_to_string(e, &s1);
                let s2 = Selector::parse("div.trans-ce a.point").expect("selector parse");
                let word = parse_and_wrap(e, &s2);

                let s3 = Selector::parse(".word-exp_tran").expect("selector parse");
                let explanation = parse_and_wrap(e, &s3);
                TranResult { part_of_speech: index + " " + &word, explanation }
            })
            .collect();
    }

    result
}

fn parse_pronunciation(document: &Html) -> Vec<PronResult> {
    let phone_selector = Selector::parse("div.trans-container div.per-phone").expect("selector parse");
    let pronunciation: Vec<PronResult> = document
        .select(&phone_selector)
        .map(|e| {
            let s1 = Selector::parse(":first-child").expect("selector parse");
            let alias = parse_to_string(e, &s1);
            let s2 = Selector::parse("span.phonetic").expect("selector parse");
            let pronunciation = parse_and_wrap(e, &s2);
            PronResult { alias, pronunciation }
        })
        .collect();
    pronunciation
}

#[cfg(test)]
mod tests {
    use super::dict;

    #[tokio::test]
    async fn test_dict() {
        assert!(dict("hello").await.expect("ok").result.iter().any(|it| it.explanation.contains("你好")));
    }
}
