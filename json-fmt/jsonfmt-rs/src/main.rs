#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::{stdin, Read};
use std::path::PathBuf;

use jsonfmt::fmt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the file for format
    #[arg(short = 'f')]
    file: Option<PathBuf>,

    #[arg()]
    values: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = get_content(&args)?;

    let ret = fmt(&content);
    match ret {
        Ok(value) => println!("{value}"),
        Err(err) => eprintln!("{err:?}"),
    }

    Ok(())
}

fn get_content(args: &Args) -> Result<String> {
    if let Some(file) = &args.file {
        //来自--file 指定的文件
        Ok(fs::read_to_string(file)?)
    } else if !args.values.is_empty() {
        // 来自命令行参数值
        Ok(args.values.join(""))
    } else {
        //从标准输入获取
        let mut buffer = String::new();
        let _ = stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
