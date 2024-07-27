use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Days
    #[arg(short, long)]
    ext_name: Option<String>,

    #[arg(short, long, default_value = ".")]
    dir: PathBuf,
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
        println!("DEBUG: args: {:?}", self.args);
        handlers::do_files(self.args)
    }
}

mod handlers {
    use super::Args;
    use anyhow::Result;
    use std::path::{Path, PathBuf};

    pub(super) fn do_files(args: Args) -> Result<()> {
        let files = ifile_counter::files(args.dir.clone(), Box::new(build_predicate_fn(args.ext_name.clone())))?;

        output_format(args, files);

        Ok(())
    }

    fn output_format(args: Args, files: Vec<PathBuf>) {
        if !files.is_empty() {
            println!("INFO: matched files");

            for (index, f) in files.iter().enumerate() {
                println!("{:4}: {}", index + 1, f.to_str().expect(""));
            }

            println!("\n");
        }

        println!(
            "INFO: file total number that matches extension '{}': {}",
            args.ext_name.unwrap_or_default(),
            files.len()
        );
    }

    fn trim(ext_name: &str) -> &str {
        ext_name.strip_prefix(".").unwrap_or(ext_name)
    }

    fn build_predicate_fn(ext_name: Option<String>) -> impl Fn(&Path) -> bool {
        move |p| match &ext_name {
            Some(ext_name) => p.extension().is_some_and(|ext| ext.to_str().expect("to_str") == trim(ext_name)),
            None => true,
        }
    }
}
