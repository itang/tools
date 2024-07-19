use chrono::{DateTime, Local};

#[derive(Debug)]
struct Item {
    url: String,
    title: String,
}

impl Item {
    fn new(url: String, title: String) -> Self {
        Self { url, title }
    }
}

///print title of the url
pub async fn print_title(url: &str) {
    let ret = title(url).await;
    match ret {
        Ok(Item { url, title }) => {
            let local: DateTime<Local> = Local::now();
            let now = local.format("%Y-%m-%d %H:%M");

            println!("\nrs << Read.new \"{url}\",\n  title: \"{title}\",\n  created_at: \"{now}\"\n");
        }
        Err(e) => println!("\t{}", e),
    }
}

async fn title(url: &str) -> Result<Item, String> {
    async fn http_get_as_string(url: &str) -> reqwest::Result<String> {
        reqwest::get(url).await?.text().await
    }

    fn extract_ret(mut content: String) -> Result<String, String> {
        content
            .find("<title>")
            .and_then(|p1| {
                content.drain(..p1);
                content.find("</title")
            })
            .map(|p2| {
                let (start, end) = ("<title>".len(), p2);
                let title: String = content.drain(start..end).collect();
                let title = title.trim().to_string();
                title
            })
            .ok_or_else(|| "无法解析html".to_string())
    }

    http_get_as_string(url)
        .await
        .map_err(|err| format!("error: {:?}", err))
        .and_then(extract_ret)
        .map(|title| Item::new(url.to_string(), title))
}

/// ////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_title() {
        use super::title;

        let r = title("http://www.baidu.com").await.expect("title");
        assert_eq!(r.title, "百度一下，你就知道");
    }
}
