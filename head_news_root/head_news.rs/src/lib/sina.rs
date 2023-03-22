//! sina module.
use async_trait::async_trait;
use scraper::Html;
use scraper::Selector;

use crate::types::*;

/// Sina.
pub struct Sina;

#[async_trait]
impl Portal for Sina {
    async fn get_news() -> Result<Vec<News>, Box<dyn std::error::Error>> {
        let content = reqwest::get("https://news.sina.com.cn/")
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&content);

        let selector = Selector::parse(r#"h1[data-client="headline"] a"#)?;

        let news_list = document
            .select(&selector)
            .map(|element| News {
                title: element.text().collect::<String>().trim().to_string(),
                href: element.value().attr("href").expect("get href").trim().to_string(),
            })
            .collect();

        Ok(news_list)
    }
}
