use serde::ser::Serialize;
use std::error::Error;

pub fn http_get_as_string(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

pub fn http_post_as_string<S>(url: &str, obj: &S) -> Result<String, Box<dyn Error>>
where
    S: Serialize,
{
    let client = reqwest::blocking::Client::new();
    Ok(client.post(url).json(obj).send()?.text()?)
}
