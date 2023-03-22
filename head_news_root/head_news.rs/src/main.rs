use head_news::*;
use tabled::{Style, TableIteratorExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_list = Sina::get_news().await?;

    let mut table = news_list.table();
    table.with(Style::psql());

    println!("{}", table);

    Ok(())
}
