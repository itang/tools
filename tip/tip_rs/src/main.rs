use std::env;
use std::fs::{self, DirEntry};
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
    name: Option<String>,
}

enum Command {
    Display { name: String },
    List,
}
fn main() -> Result<()> {
    let args = Args::parse();

    match args.name {
        Some(name) => {
            println!("Tips for {name}");
            println!("{}", "-".repeat(80));
            handle(Command::Display { name })
        }
        None => {
            println!("Please input tip name!");
            handle(Command::List)
        }
    }
}

fn handle(command: Command) -> Result<()> {
    match command {
        Command::Display { name } => {
            let content = get_markdown_path(name)?.read_to_string()?;
            println!("{content}");
        }
        Command::List => {
            let root_dir = get_tip_data_root_dir()?;
            fn visit_dirs(i: u32, dir: &Path, cb: &dyn Fn(u32, &DirEntry)) -> io::Result<()> {
                if dir.is_dir() {
                    let mut v = i;
                    for entry in fs::read_dir(dir)? {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_dir() {
                            visit_dirs(i, &path, cb)?;
                        } else {
                            cb(v, &entry);
                            v = v + 1;
                        }
                    }
                }
                Ok(())
            }

            visit_dirs(1, root_dir.as_path(), &|i, x| {
                let name = Path::new(&x.file_name())
                    .with_extension("")
                    .into_os_string()
                    .into_string()
                    .unwrap();
                print!("{name:16}");
                if i % 6 == 0 {
                    println!("");
                }
            })?;
        }
    }

    Ok(())
}

fn get_tip_data_root_dir() -> Result<PathBuf> {
    let path = env::var(DATA_ROOT_ENV_NAME)
        .map(|x| Path::new(&x).to_path_buf())
        .or_else(|_| {
            env::var(HOME_ENV_NAME).map(|x| Path::new(&x).join("bin").join("data").join("tip"))
        })?;
    Ok(path)
}

fn get_markdown_path(name: String) -> Result<PathBuf> {
    let root = get_tip_data_root_dir();

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
