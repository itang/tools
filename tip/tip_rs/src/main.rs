use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

const VERSION: &'static str = "0.1.1-20220117";

const DATA_ROOT_ENV_NAME: &'static str = "TIP_DATA_ROOT";
const HOME_ENV_NAME: &str = "HOME";

fn main() -> Result<()> {
    println!("tip_rs-V{VERSION}");

    let names: Vec<String> = env::args().skip(1).collect();
    println!("tips for {names:?}");
    println!("{}", "-".repeat(80));

    for name in names {
        let content = get_content(name)?;
        println!("{content}");
    }

    Ok(())
}

fn get_content(name: String) -> Result<String> {
    let path = get_markdown_path(name)?;
    Ok(fs::read_to_string(path)?)
}

fn get_markdown_path(name: String) -> Result<PathBuf> {
    let root = env::var(DATA_ROOT_ENV_NAME)
        .map(|x| Path::new(&x).to_path_buf())
        .or_else(|_| {
            env::var(HOME_ENV_NAME).map(|x| Path::new(&x).join("bin").join("data").join("tip"))
        });

    Ok(root.map(|x| x.join(format!("{name}.md")))?)
}
