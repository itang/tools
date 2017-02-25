extern crate reqwest;


use std::io::Read;
use serde::ser::Serialize;


pub fn http_get_as_string(url: &str) -> Result<String, String> {
    let client = reqwest::Client::new().map_err(|x| format!("{}", x))?;
    let mut res = client.get(url).send().map_err(|x| format!("{}", x))?;

    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|x| format!("{}", x)).map(|_| body)
}

pub fn http_post_as_string<S>(url: &str, obj: S) -> Result<String, String>
    where S: Serialize
{
    let client = reqwest::Client::new().map_err(|x| format!("{}", x))?;
    let mut res = client.post(url).form(&obj).send().map_err(|x| format!("{}", x))?;

    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|x| format!("{}", x)).map(|_| body)
}
