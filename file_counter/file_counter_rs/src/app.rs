use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Extension name for match
    #[arg(short, long)]
    ext_name: Option<Vec<String>>,

    /// Contains for match
    #[arg(short, long)]
    contains: Option<Vec<String>>,

    ///The dir
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

    pub fn run(self) -> anyhow::Result<()> {
        println!("DEBUG: args: {:?}", self.args);
        handlers::do_files(self.args)
    }
}

mod handlers {
    use std::path::{Path, PathBuf};

    use super::Args;
    use ifile_counter::{build_predicate_contains_fn, build_predicate_ext_fn, PredicatePathFn};

    pub(super) fn do_files(args: Args) -> anyhow::Result<()> {
        let files = ifile_counter::files(args.dir.clone(), Box::new(build_predicate_fn(args.clone())))?;
        output_format(&files);

        Ok(())
    }

    fn build_predicate_fn(args: Args) -> impl Fn(&Path) -> bool {
        let mut ps: Vec<Box<PredicatePathFn>> = Vec::new();

        if let Some(ext_name) = args.ext_name {
            ps.push(Box::new(build_predicate_ext_fn(ext_name)));
        }

        if let Some(contains) = args.contains {
            ps.push(Box::new(build_predicate_contains_fn(contains)))
        }

        move |path| {
            for f in ps.iter() {
                if !f(path) {
                    return false;
                }
            }

            true
        }
    }

    fn output_format(files: &[PathBuf]) {
        if !files.is_empty() {
            println!("INFO: matched files");

            for (index, f) in files.iter().enumerate() {
                println!("{:4}: {}", index + 1, f.to_str().expect(""));
            }

            println!("\n");
        }

        println!("INFO: The total number of matched files is {}", files.len());
    }
}
