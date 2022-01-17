use std::env;
use std::fs;
use std::io;
use std::path::Path;

use anyhow::Result;

const DATA_HOME: &'static str = r#"D:\ProgramData\bin\data\tip"#;
const VERSION: &'static str = "0.1-20220117";

fn main() -> Result<()> {
    println!("tip_rs-V{VERSION}");

    let names: Vec<String> = env::args().skip(1).collect();
    println!("{names:?}");

    for name in names {
        let content = get_content(name)?;
        println!("{content}");
    }

    Ok(())
}

fn get_content(name: String) -> io::Result<String> {
    let path = Path::new(DATA_HOME).join(format!("{name}.md"));
    fs::read_to_string(path)
}
