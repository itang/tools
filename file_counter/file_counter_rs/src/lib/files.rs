use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::types::PredicatePathFn;

/// get all files in some dir with predicate
pub fn files<P>(
    root_dir: P, ignore_dirs: Option<Vec<PathBuf>>, file_predicate: Box<PredicatePathFn>,
) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    fn _rec_files<S: AsRef<Path>>(
        p: S, ignore_dirs: &Option<Vec<PathBuf>>, pred: &PredicatePathFn, res: &mut Vec<PathBuf>,
    ) -> Result<()> {
        let files = fs::read_dir(p)?;
        for entry in files {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if pred(path.as_path()) {
                    res.push(path);
                }
            } else if path.is_dir() && !is_ignore_dir(&path, ignore_dirs) {
                _rec_files(path, ignore_dirs, pred, res)?;
            }
        }

        Ok(())
    }
    fn is_ignore_dir(dir: &PathBuf, ignore_dirs: &Option<Vec<PathBuf>>) -> bool {
        match ignore_dirs {
            Some(ignore_dirs) => ignore_dirs.iter().any(|p| p == dir),
            None => false,
        }
    }

    let mut res = Vec::new();
    _rec_files(root_dir, &ignore_dirs, file_predicate.as_ref(), &mut res)?;

    Ok(res)
}
