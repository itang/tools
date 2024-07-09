use tabled::{settings::Style, Table};
use tokio::runtime;

use head_news::{Portal, Sina};

// https://stackoverflow.com/questions/66960947/what-is-the-smallest-feature-set-to-enable-polling-a-future-with-tokio
//#[tokio::main(flavor = "current_thread")]
/*async*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_list = runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .expect("build tokio runtime")
        .block_on(Sina::get_news())?;
    // let news_list = Sina::get_news().await?;

    let mut table = Table::new(news_list);
    table.with(Style::psql());

    println!("{}", table);

    Ok(())
}
