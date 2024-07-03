//!infrastructure

mod browser;

mod config_provider;

pub(crate) use browser::BrowserImpl;

pub(crate) use config_provider::ConfigProviderImpl;
