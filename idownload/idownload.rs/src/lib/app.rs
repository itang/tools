//! app
//!

use clap::{
    Parser,
    builder::styling::{AnsiColor, Style, Styles},
};

use crate::downloader::Downloader;

const HEADER: Style = AnsiColor::Green.on_default().bold();
const USAGE: Style = AnsiColor::Green.on_default().bold();
const LITERAL: Style = AnsiColor::Cyan.on_default().bold();
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const ERROR: Style = AnsiColor::Red.on_default().bold();
const VALID: Style = AnsiColor::Cyan.on_default().bold();
const INVALID: Style = AnsiColor::Yellow.on_default().bold();

const HELP_STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);

/// Config
#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "A simple file download tool", long_about = None, styles = HELP_STYLES)]
pub struct Config {
    /// URL to download
    pub url: String,

    /// Output file path (optional, defaults to current directory with original filename)
    #[arg(short, long)]
    pub output: Option<String>,
}

impl Config {
    /// Parse Config
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

/// App
#[derive(Debug, Clone)]
pub struct App {
    config: Config,
}

impl App {
    /// New App
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Run App
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let downloader = Downloader::new();
        downloader.download(&self.config.url, self.config.output.as_deref()).await
    }
}
