mod sina;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_list = sina::get_news().await?;

    for news in news_list {
        println!("{:30} {}", news.title, news.href);
    }

    Ok(())
}
