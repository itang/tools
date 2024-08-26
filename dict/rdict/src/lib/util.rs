use std::error::Error;

pub(crate) async fn http_get_as_string(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("")
        .build()?;
    Ok(client.get(url).send().await?.text().await?)
}
