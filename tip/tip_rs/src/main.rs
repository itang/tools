use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

const DATA_HOME: &'static str = r#"D:\ProgramData\bin\data\tip"#;
const VERSION: &'static str = "0.1-20220117";

fn main() -> Result<()> {
    println!("tip_rs-V{VERSION}");

    let names: Vec<String> = args().skip(1).collect();
    println!("{names:?}");

    for name in names {
        let content = read_to_string(Path::new(DATA_HOME).join(format!("{name}.md")))?;
        println!("{content}");
    }

    Ok(())
}
