use std::collections::HashMap;
use std::error::Error;

use serde::Serialize;
use url::Url;

///Coder Result
pub type CoderResult = Result<String, Box<dyn Error>>;

///Coder
pub trait Coder {
    ///encode
    fn encode(&self, input: impl AsRef<[u8]>) -> CoderResult;
    ///decode
    fn decode(&self, input: impl AsRef<[u8]>) -> CoderResult;
}

///base64
pub mod base64 {
    use super::{Coder, CoderResult};
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};

    ///Base64
    pub struct Base64;

    impl Coder for Base64 {
        /// base64 encode.
        #[inline]
        fn encode(&self, input: impl AsRef<[u8]>) -> CoderResult {
            Ok(STANDARD_NO_PAD.encode(input))
        }

        /// base64 decode.
        #[inline]
        fn decode(&self, input: impl AsRef<[u8]>) -> CoderResult {
            let v = STANDARD_NO_PAD.decode(input)?;
            let v = std::str::from_utf8(&v)?;

            Ok(v.to_string())
        }
    }
}

///hex
pub mod hex {
    use super::{Coder, CoderResult};

    ///Hex
    pub struct Hex;

    impl Coder for Hex {
        ///encode
        #[inline]
        fn encode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            Ok(faster_hex::hex_string(src.as_ref()))
        }

        /// decode
        #[inline]
        fn decode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let bytes = src.as_ref();
            let mut dst = vec![0; bytes.len() / 2];
            faster_hex::hex_decode(bytes, &mut dst)?;
            let v = std::str::from_utf8(&dst)?;

            Ok(v.to_string())
        }
    }
}

///i to hex
pub mod i2hex {
    use super::{Coder, CoderResult};

    ///Hex
    pub struct I2Hex;

    impl Coder for I2Hex {
        ///encode
        #[inline]
        fn encode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let src = std::str::from_utf8(src.as_ref())?.to_string();
            let i: i64 = src.parse()?;
            Ok(format!("0x{:x}", i))
        }

        /// decode
        #[inline]
        fn decode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let src = std::str::from_utf8(src.as_ref())?;
            let src = src.strip_prefix("0x").unwrap_or(src);
            Ok(i64::from_str_radix(src, 16)?.to_string())
        }
    }
}


///i to binary
pub mod i2binary {
    use super::{Coder, CoderResult};

    ///Hex
    pub struct I2Binary;

    impl Coder for I2Binary {
        ///encode
        #[inline]
        fn encode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let src = std::str::from_utf8(src.as_ref())?.to_string();
            let i: i64 = src.parse()?;
            Ok(format!("0b{:b}", i))
        }

        /// decode
        #[inline]
        fn decode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let src = std::str::from_utf8(src.as_ref())?;
            let src = src.strip_prefix("0b").unwrap_or(src);
            Ok(i64::from_str_radix(src, 2)?.to_string())
        }
    }
}


///encode
pub fn md5(src: impl AsRef<[u8]>) -> String {
    let digest = md5::compute(src);
    format!("{:x}", digest)
}


///uuid
pub fn uuid(upcase: bool, no_underline: bool) -> String {
    let id = uuid::Uuid::new_v4();
    let mut ret = id.to_string();
    if upcase {
        ret = ret.to_uppercase();
    } else {
        ret = ret.to_lowercase();
    }

    if no_underline {
        ret = ret.replace('-', "");
    }

    ret
}

///random_str
pub fn random_str(length: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

///now
pub fn now(fmt: &str) -> String {
    use chrono::{Datelike, Weekday};
    let now = chrono::offset::Local::now();
    let week = match now.weekday() {
        Weekday::Mon => "星期一",
        Weekday::Tue => "星期二",
        Weekday::Wed => "星期三",
        Weekday::Thu => "星期四",
        Weekday::Fri => "星期五",
        Weekday::Sat => "星期六",
        Weekday::Sun => "星期天",
    };
    format!("{} {}", now.format(fmt), week)
}

///query mode
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum QueryMode {
    ///qsl
    Qsl,
    ///qs
    Qs,
    ///raw
    Raw,
}

///pretty print url

pub fn pretty_print_url(url: &str, query_mode: QueryMode) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(url)?;

    let scheme = url.scheme();
    let url_obj = UrlObj {
        scheme: scheme.to_string(),
        host: url.host().map(|h| h.to_string()),
        #[allow(clippy::unnecessary_lazy_evaluations)]
        port: url.port().or_else(|| match scheme {
            "https" => Some(443),
            "http" => Some(80),
            _ => None
        }),
        path: url.path().to_string(),
        query: url.query().map(|q| parse_query(q, query_mode)),
        fragment: url.fragment().map(|f| f.to_string()),
        username: Some(url.username().to_string()),
        password: url.password().map(|p| p.to_string()),
    };

    Ok(serde_json::to_string_pretty(&url_obj)?)
}

fn parse_query(query: &str, query_mode: QueryMode) -> Query {
    match query_mode {
        QueryMode::Qsl => {
            let mut q = HashMap::new();
            for (k, v) in url::form_urlencoded::parse(query.as_bytes()).into_owned() {
                q.insert(k, v);
            }
            Query::Qsl(q)
        }
        QueryMode::Qs => {
            let mut q = HashMap::new();
            for (k, v) in url::form_urlencoded::parse(query.as_bytes()).into_owned() {
                q.entry(k).or_insert(vec![]).push(v);
            }
            Query::Qs(q)
        }
        QueryMode::Raw => Query::Raw(query.to_string()),
    }
}

#[derive(Serialize, Debug)]
pub struct UrlObj {
    pub scheme: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<Query>,
    pub fragment: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Debug)]
//#[serde(tag = "__type")]
#[serde(untagged)]
pub enum Query {
    ///qsl
    Qsl(HashMap<String, String>),
    ///qs
    Qs(HashMap<String, Vec<String>>),
    ///raw
    Raw(String),
}
