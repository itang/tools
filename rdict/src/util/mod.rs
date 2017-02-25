extern crate reqwest;

use std::error::Error;
use std::io::Read;
use serde::ser::Serialize;


pub fn http_get_as_string(url: &str) -> Result<String, Box<Error>> {
    let client = reqwest::Client::new()?;
    let mut res = client.get(url).send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}

pub fn http_post_as_string<S>(url: &str, obj: S) -> Result<String, Box<Error>>
    where S: Serialize
{
    let client = reqwest::Client::new()?;
    let mut res = client.post(url).form(&obj).send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}
