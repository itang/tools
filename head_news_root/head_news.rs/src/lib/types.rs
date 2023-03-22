//! types module.
use async_trait::async_trait;
use tabled::Tabled;

/// News.
#[derive(Debug, Tabled)]
pub struct News {
    /// The title.
    pub title: String,
    /// The href.
    pub href: String,
}

/// Portal trait.
#[async_trait]
pub trait Portal {
    /// Returns news.
    async fn get_news() -> Result<Vec<News>, Box<dyn std::error::Error>>;
}
