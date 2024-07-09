//! types module.
use std::future::Future;
//use async_trait::async_trait;

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
//#[async_trait]
pub trait Portal {
    //async fn get_news() -> Result<Vec<News>, Box<dyn std::error::Error>>;

    /// Returns news.
    ///TODO: https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html

    fn get_news() -> impl Future<Output = Result<Vec<News>, Box<dyn std::error::Error>>> + Send;
}
