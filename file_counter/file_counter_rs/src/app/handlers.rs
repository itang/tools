use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;

use ifile_counter::{
    and, build_glob_match_fn, build_predicate_contains_fn, build_predicate_ext_fn,
    build_predicate_not_contains_fn, not, PredicatePathFn,
};

use super::Args;

pub(crate) fn main(args: Args) -> anyhow::Result<()> {
    let files =
        ifile_counter::files(args.dir.clone(), args.ignore_dirs.clone(), Box::new(build_predicate_fn(args.clone())))?;

    output_format(&files);

    if args.show_all || args.show_same_name_files {
        output_same_name_files(&files)?;
    }

    if args.show_all || args.show_extensions {
        output_extensions(&files)?;
    }

    if let Some(exported_dir) = &args.exported_dir {
        println!("INFO : export matched files to directory {:?}", exported_dir);
        export(&files, exported_dir, &args.strip_prefix_before_dir)?
    }

    Ok(())
}

fn export(files: &[PathBuf], exported_dir: &PathBuf, strip_prefix_before_dir: &Option<PathBuf>) -> anyhow::Result<()> {
    if exported_dir.exists() {
        return Err(anyhow!("{:?} already exists", exported_dir));
    } else {
        std::fs::create_dir_all(exported_dir)?;
    }

    for file in files {
        let mut new_file = PathBuf::from(exported_dir);
        let relative_path = get_relative_path(exported_dir, strip_prefix_before_dir, file)?;
        dbg!(&relative_path);

        if let Some(relative_path) = relative_path {
            new_file.push(relative_path);

            if let Some(p) = new_file.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }

            std::fs::copy(file, &new_file)?;
            println!("DEBUG: copy {:?}", &new_file);
        } else {
            println!("DEBUG: ignore {:?}", file);
        }
    }

    Ok(())
}

//TODO: Review
fn get_relative_path(
    _dir: &Path, strip_prefix_before_dir: &Option<PathBuf>, file: &Path,
) -> anyhow::Result<Option<PathBuf>> {
    match strip_prefix_before_dir {
        Some(dir) => {
            let buf = file.to_path_buf();

            let components = buf.components();

            let mut ret = PathBuf::new();

            let mut push = false;
            for comp in components {
                //dbg!(&comp);
                let x = comp.as_os_str();
                if x == dir.as_os_str() {
                    push = true;
                }

                if push {
                    ret.push(comp);
                }
            }
            if push {
                Ok(Some(ret))
            } else {
                Ok(None)
            }
        },
        None => Ok(Some(file.to_path_buf())),
    }
}

fn build_predicate_fn(args: Args) -> impl Fn(&Path) -> bool {
    let mut ps: Vec<Box<PredicatePathFn>> = Vec::new();

    if let Some(ext_name) = args.ext_name {
        ps.push(Box::new(build_predicate_ext_fn(ext_name)));
    }

    if let Some(glob) = args.glob {
        ps.push(Box::new(build_glob_match_fn(glob)))
    }

    if let Some(contains) = args.contains {
        ps.push(Box::new(build_predicate_contains_fn(contains)))
    }

    if let Some(not_contains) = args.not_contains {
        ps.push(Box::new(build_predicate_not_contains_fn(not_contains)))
    }

    and(ps)
}

fn output_format(files: &[PathBuf]) {
    if !files.is_empty() {
        println!("\nINFO : matched files");

        for (index, f) in files.iter().enumerate() {
            println!("{:4}: {}", index + 1, f.display());
        }

        println!("\n");
    }

    println!("INFO : The total number of matched files is {}", files.len());
}

fn output_same_name_files(files: &[PathBuf]) -> anyhow::Result<()> {
    let files: Vec<(&PathBuf, &str)> =
        files.iter().map(|f| (f, f.file_name().expect("").to_str().expect(""))).collect();

    let mut map = HashMap::new();
    for item in files.iter() {
        map.entry(item.1).or_insert_with(Vec::new).push(item);
    }

    let duplicate: HashMap<&str, &Vec<&(&PathBuf, &str)>> =
        map.iter().filter(|(_, value)| value.len() > 1).map(|(&key, value)| (key, value)).collect();

    println!("\nINFO : List of files with duplicate filenames");

    for (index, (&key, &value)) in duplicate.iter().enumerate() {
        println!("{:4}: {:}", index + 1, key);
        for (index2, it) in value.iter().enumerate() {
            println!("\t{:3}: {}", index2 + 1, it.0.as_path().display());
        }
    }

    println!("INFO : The total number of files with duplicate filenames is {}", duplicate.len());

    println!("\n");

    Ok(())
}

fn output_extensions(files: &[PathBuf]) -> anyhow::Result<()> {
    let mut extensions: Vec<&str> =
        files.iter().flat_map(|file| file.extension().map(|it| it.to_str().expect("to_str"))).collect();

    extensions.sort_unstable();
    extensions.dedup();

    println!("\nINFO : List of the extensions");

    for (index, value) in extensions.into_iter().enumerate() {
        println!("{:4}: .{}", index + 1, value);
    }

    Ok(())
}
