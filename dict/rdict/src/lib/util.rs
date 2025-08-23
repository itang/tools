use std::error::Error;

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36";

pub(crate) async fn http_get_as_string(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

    let resp = client.get(url).send().await?;

    let body = resp.text().await?;

    Ok(body)
}
