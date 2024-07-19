// ///Translate Result
// #[derive(Serialize, Deserialize, Debug)]
// pub struct TransResult {
//     ///to
//     pub to: String,
//     ///from
//     pub from: String,
// }

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

//const MAX_TO_CHARS: usize = 100;

// pub fn post_to_cloud(upstream_url: &str, tr: &TransResult) -> Result<String, Box<dyn Error>> {
//     if tr.to.len() > MAX_TO_CHARS {
//         let msg = format!("Too large content({} chars), ignore to post!", tr.to.len());
//         println!("INFO: {}", msg);
//         Err(From::from(msg))
//     } else {
//         util::http_post_as_string(upstream_url, tr)
//     }
// }

/// ////////////////////////////////////////////////////////////////////////////
#[tokio::test]
async fn test_dict() {
    assert!(dict("hello").await.expect("ok").contains("你好"));
}
