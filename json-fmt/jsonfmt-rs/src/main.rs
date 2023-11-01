#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::{stdin, Read};
use std::path::PathBuf;
//use thiserror::Error;

use jsonfmt::fmt_json_string_pretty;

//TODO: 支持指定json-path输出
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the file for format
    #[arg(short = 'f', long)]
    file: Option<PathBuf>,

    #[arg()]
    values: Vec<String>,
}

// #[derive(Error, Debug)]
// enum GetJsonContentError {
//     #[error("can't get content from args")]
//     GetError,
// }

impl Args {
    fn get_json_content(&self) -> Result<Option<String>> {
        if let Some(file) = &self.file {
            //来自--file 指定的文件
            Ok(Some(fs::read_to_string(file)?))
        } else if !self.values.is_empty() {
            // 来自命令行参数值
            Ok(Some(self.values.join("")))
        } else {
            Ok(None)
            //Err(GetJsonContentError::GetError.into())
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = get_content(&args)?;
    let result = fmt_json_string_pretty(&content);

    match result {
        Ok(value) => println!("{value}"),
        Err(err) => eprintln!("ERROR: {err:?}"),
    }

    Ok(())
}

fn get_content(args: &Args) -> Result<String> {
    if let Some(value) = args.get_json_content()? {
        //从命令行参数获取
        Ok(value)
    } else {
        //从标准输入获取
        get_content_from_stdin()
    }
}

fn get_content_from_stdin() -> Result<String> {
    //on windows: ctrl + z get all input
    eprintln!("INFO: input the content for format(Press ctrl+z to finish typing on windows):\n");
    let mut buffer = String::new();
    let _ = stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}
