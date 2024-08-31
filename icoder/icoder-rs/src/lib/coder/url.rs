//! coder url
//!
use std::collections::HashMap;
use std::error::Error;

use serde::Serialize;
use url::Url;

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

fn parse_query(query: &str, query_mode: QueryMode) -> QueryObj {
    match query_mode {
        QueryMode::Qsl => {
            let mut q = HashMap::new();
            for (k, v) in url::form_urlencoded::parse(query.as_bytes()).into_owned() {
                q.insert(k, v);
            }
            QueryObj::Qsl(q)
        }
        QueryMode::Qs => {
            let mut q = HashMap::new();
            for (k, v) in url::form_urlencoded::parse(query.as_bytes()).into_owned() {
                q.entry(k).or_insert(vec![]).push(v);
            }
            QueryObj::Qs(q)
        }
        QueryMode::Raw => QueryObj::Raw(query.to_string()),
    }
}

///Url Object
#[derive(Serialize, Debug)]
struct UrlObj {
    ///scheme
    pub scheme: String,
    ///host
    pub host: Option<String>,
    ///port
    pub port: Option<u16>,
    ///path
    pub path: String,
    ///query
    pub query: Option<QueryObj>,
    ///fragment
    pub fragment: Option<String>,
    ///username
    pub username: Option<String>,
    ///password
    pub password: Option<String>,
}

///Query
#[derive(Serialize, Debug)]
//#[serde(tag = "__type")]
#[serde(untagged)]
enum QueryObj {
    ///qsl
    Qsl(HashMap<String, String>),
    ///qs
    Qs(HashMap<String, Vec<String>>),
    ///raw
    Raw(String),
}