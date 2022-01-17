use std::env;
use std::fs;
use std::path::Path;

use anyhow::Result;

const DATA_HOME: &'static str = r#"D:\ProgramData\bin\data\tip"#;
const VERSION: &'static str = "0.1-20220117";

fn main() -> Result<()> {
    println!("tip_rs-V{VERSION}");

    let names: Vec<String> = env::args().skip(1).collect();
    println!("{names:?}");

    for name in names {
        let path = Path::new(DATA_HOME).join(format!("{name}.md"));
        let content = fs::read_to_string(path)?;
        println!("{content}");
    }

    Ok(())
}
