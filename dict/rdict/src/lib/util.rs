use std::error::Error;

pub(crate) async fn http_get_as_string(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url).await?.text().await?)
}
