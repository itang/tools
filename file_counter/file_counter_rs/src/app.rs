use clap::Parser;
use std::path::Path;

use anyhow::Result;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Days
    #[arg(short, long, default_value = "java")]
    ext_name: String,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    args: Args,
}

impl Router {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn run(self) -> Result<()> {
        do_files(self.args.ext_name)
    }
}

fn do_files(ext_name: String) -> Result<()> {
    fn build_predicate_fn(ext_name: String) -> impl Fn(&Path) -> bool {
        move |p| {
            if let Some(ext) = p.extension() {
                ext.to_str().expect("") == ext_name
            } else {
                false
            }
        }
    }

    let files = ifile_counter::files(".", Box::new(build_predicate_fn(ext_name.clone())))?;

    for (index, f) in files.iter().enumerate() {
        println!("{:4}: {}", index + 1, f.to_str().expect(""));
    }

    if !files.is_empty() {
        println!("\n");
    }

    println!("INFO: file total number that matches extension '{}': {}", ext_name, files.len());

    Ok(())
}
