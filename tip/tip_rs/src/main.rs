use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

const DATA_ROOT_ENV_NAME: &str = "TIP_DATA_ROOT";
const HOME_ENV_NAME: &str = "HOME";

//ref: https://github.com/clap-rs/clap/tree/v3.0.9/examples/tutorial_derive
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let name = args.name;
    println!("tips for {name}");
    println!("{}", "-".repeat(80));

    let content = get_markdown_path(name)?.read_to_string()?;
    println!("{content}");

    Ok(())
}

fn get_markdown_path(name: String) -> Result<PathBuf> {
    let root = env::var(DATA_ROOT_ENV_NAME)
        .map(|x| Path::new(&x).to_path_buf())
        .or_else(|_| {
            env::var(HOME_ENV_NAME).map(|x| Path::new(&x).join("bin").join("data").join("tip"))
        });

    Ok(root.map(|x| x.join(format!("{name}.md")))?)
}

trait IReadString {
    fn read_to_string(&self) -> io::Result<String>;
}

impl IReadString for PathBuf {
    fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self)
    }
}
