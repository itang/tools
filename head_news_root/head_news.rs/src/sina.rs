use scraper::Html;
use scraper::Selector;

#[derive(Debug)]
pub struct News {
    pub title: String,
    pub href: String,
}

pub async fn get_news() -> Result<Vec<News>, Box<dyn std::error::Error>> {
    let content = reqwest::get("https://news.sina.com.cn/")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&content);

    let selector = Selector::parse(r#"h1[data-client="headline"] a"#)?;

    Ok(document
        .select(&selector)
        .map(|element| News {
            title: element.text().collect::<String>(),
            href: element.value().attr("href").unwrap().to_string(),
        })
        .collect())
}
