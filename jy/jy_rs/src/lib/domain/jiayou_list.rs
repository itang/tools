use std::path::PathBuf;
use toml::Value;

use crate::domain::gateway::{Browser, ConfigProvider};
use crate::domain::Url;

///jiayou list
#[derive(Debug)]
pub struct JiayouList {
    urls: Vec<Url>,
    _p: std::marker::PhantomData<()>,
}

impl JiayouList {
    ///new
    pub fn new(urls: Vec<Url>) -> Self {
        Self { urls, _p: std::marker::PhantomData }
    }

    /// parse
    pub fn parse(config_path: Option<PathBuf>, cp: impl ConfigProvider) -> anyhow::Result<Self> {
        let config = cp.get_config(config_path)?;
        let urls: Vec<String> = config.urls();
        let urls: Vec<Url> = urls.into_iter().map(Url).collect();

        Ok(JiayouList::new(urls))
    }

    ///browser the list
    pub fn browser(&self, dry_run: bool, browser: impl Browser) -> anyhow::Result<()> {
        for (index, url) in self.urls.iter().enumerate() {
            println!("{index:4}: open {url}", index = index + 1, url = url.0);
            if !dry_run {
                browser.browser_single_url(url)?;
            }
        }

        Ok(())
    }
}

///Urls
trait GetUrls {
    ///get urls
    fn urls(&self) -> Vec<String>;
}

impl GetUrls for Value {
    /// get urls from toml config
    fn urls(&self) -> Vec<String> {
        let urls = self["urls"]
            .as_array()
            .into_iter()
            .flat_map(|x| x.iter().flat_map(|y| y.as_str().into_iter()))
            .map(|x| x.to_string());
        urls.collect()
    }
}
