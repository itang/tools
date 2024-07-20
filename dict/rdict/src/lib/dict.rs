use std::error::Error;

use crate::util;

/// dict
pub async fn dict(word: &str) -> Result<String, Box<dyn Error>> {
    // content: owned move ?
    fn extract_result(mut content: String) -> Option<String> {
        if let Some(p1) = content.find("trans-container") {
            content.drain(..p1);
            if let Some(p2) = content.find("<li>") {
                content.drain(..p2);
                let (end, start) = (content.find("</li>").expect("find"), "</li>".len() - 1);
                return Some(content.drain(start..end).collect());
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
    assert!(dict("hello").await.expect("ok").contains("你好"));
}
