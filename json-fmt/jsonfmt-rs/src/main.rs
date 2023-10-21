#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use clap::Parser;

use jsonfmt::fmt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    values: Vec<String>,
}

//TODO: 1. 支持--file 指定要格式化的文件， 2. 支持要格式的内容来自 std input
fn main() {
    let args = Args::parse();

    let value = args.values.join("");
    let ret = fmt(&value);
    match ret {
        Ok(value) => println!("{value}"),
        Err(err) => eprintln!("{err:?}"),
    }
}
