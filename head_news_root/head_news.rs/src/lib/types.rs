use async_trait::async_trait;

use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct News {
    pub title: String,
    pub href: String,
}

#[async_trait]
pub trait Portal {
    async fn get_news() -> Result<Vec<News>, Box<dyn std::error::Error>>;
}
