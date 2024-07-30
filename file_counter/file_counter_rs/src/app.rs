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

    ///The source dir
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,

    #[arg(long)]
    exported_dir: Option<PathBuf>,
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
        handlers::main(self.args)
    }
}

mod handlers {
    use super::Args;
    use anyhow::anyhow;
    use ifile_counter::{build_predicate_contains_fn, build_predicate_ext_fn, PredicatePathFn};
    use std::path::{Path, PathBuf};

    pub(super) fn main(args: Args) -> anyhow::Result<()> {
        let files = ifile_counter::files(args.dir.clone(), Box::new(build_predicate_fn(args.clone())))?;

        output_format(&files);

        if let Some(exported_dir) = &args.exported_dir {
            println!("INFO: export matched files to directory {:?}", exported_dir);
            export(&files, exported_dir)?
        }

        Ok(())
    }

    fn export(files: &[PathBuf], exported_dir: &PathBuf) -> anyhow::Result<()> {
        if exported_dir.exists() {
            return Err(anyhow!("{:?} already exists", exported_dir));
        } else {
            std::fs::create_dir_all(exported_dir)?;
        }

        for file in files {
            let mut new_file = PathBuf::from(exported_dir);
            let relative_path = get_relative_path(exported_dir, file)?;

            new_file.push(relative_path);

            if let Some(p) = new_file.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }

            std::fs::copy(file, &new_file)?;
            println!("DEBUG: copy {:?}", &new_file);
        }

        Ok(())
    }

    fn get_relative_path(_dir: &Path, file: &Path) -> anyhow::Result<PathBuf> {
        Ok(file.to_path_buf())
        //Ok(file.strip_prefix(std::env::current_dir()?)?.to_path_buf())
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
