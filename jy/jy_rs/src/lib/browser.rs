//! browser
//!
use anyhow::Result;

/// browser batch urls
pub fn browser_batch(urls: Vec<String>) -> Result<()> {
    for (index, url) in urls.iter().enumerate() {
        println!("{index:4}: open {url}", index = index + 1);
        browser_single_url(url)?;
    }

    Ok(())
}

#[inline(always)]
fn browser_single_url(url: &str) -> Result<()> {
    Ok(webbrowser::open(url)?)
}
