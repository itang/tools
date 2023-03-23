use tabled::{Style, TableIteratorExt};

use head_news::*;

// https://stackoverflow.com/questions/66960947/what-is-the-smallest-feature-set-to-enable-polling-a-future-with-tokio
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_list = Sina::get_news().await?;

    let mut table = news_list.table();
    table.with(Style::psql());

    println!("{}", table);

    Ok(())
}
