use async_trait::async_trait;

#[derive(Debug)]
pub struct News {
    pub title: String,
    pub href: String,
}

#[async_trait]
pub trait Portal {
    async fn get_news() -> Result<Vec<News>, Box<dyn std::error::Error>>;
}
