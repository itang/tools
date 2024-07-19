use std::fmt::Display;
use std::ops::Deref;

use reqwest::IntoUrl;
use thiserror::Error;

///The Title
#[derive(Debug)]
pub struct Title(String);

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Title {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

///The Extract Error
#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

impl Title {
    /// get url title info
    pub async fn extract_from_url<U: IntoUrl>(url: U) -> Result<Self, ExtractError> {
        async fn http_get_as_string<U: IntoUrl>(url: U) -> reqwest::Result<String> {
            reqwest::get(url).await?.text().await
        }

        fn extract(mut content: String) -> Result<String, ExtractError> {
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
                .ok_or_else(|| ExtractError::ParseError("无法解析html".into()))
        }

        let content = http_get_as_string(url).await?;

        Ok(Title(extract(content)?))
    }
}

/// ////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_title() {
        use super::Title;

        let title = Title::extract_from_url("http://www.baidu.com").await.expect("title");

        assert_eq!(&*title, "百度一下，你就知道");
        assert_eq!(title.as_ref(), "百度一下，你就知道");
        assert_eq!(title.to_string(), "百度一下，你就知道");
    }
}
