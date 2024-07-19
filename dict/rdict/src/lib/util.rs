use std::error::Error;

// use serde::ser::Serialize;

pub(crate) async fn http_get_as_string(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url).await?.text().await?)
}

// pub async fn http_post_as_string<S>(url: &str, obj: &S) -> Result<String, Box<dyn Error>>
// where
//     S: Serialize,
// {
//     let client = reqwest::Client::new();
//     Ok(client.post(url).json(obj).send().await?.text().await?)
// }
