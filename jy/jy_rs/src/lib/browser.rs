//! browser
use anyhow::Result;
use toml::Value;

/// browser batch urls
pub fn browser_batch(config: Value) -> Result<()> {
    let urls = config["urls"].as_array().into_iter().flat_map(|x| x.iter().flat_map(|y| y.as_str().into_iter()));
    for (index, url) in urls.enumerate() {
        println!("{index:4}: open {url}", index = index + 1);
        browser_single_url(url)?;
    }

    Ok(())
}

fn browser_single_url(url: &str) -> Result<()> {
    Ok(webbrowser::open(url)?)
}
