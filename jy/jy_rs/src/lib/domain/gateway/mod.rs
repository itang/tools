use crate::domain::Url;
use std::path::PathBuf;
use toml::Value;

/// Config Provider
pub trait ConfigProvider {
    ///get config
    fn get_config(&self, path: Option<PathBuf>) -> anyhow::Result<Value>;
}

///Browser
pub trait Browser {
    /// browser single url
    fn browser_single_url(&self, url: &Url) -> anyhow::Result<()>;
}
