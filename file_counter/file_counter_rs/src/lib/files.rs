use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::types::PredicatePathFn;

/// get all files in some dir with predicate
pub fn files<P>(root_dir: P, predicate: Box<PredicatePathFn>) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    fn _rec_files<S: AsRef<Path>>(p: S, pred: &PredicatePathFn, res: &mut Vec<PathBuf>) -> Result<()> {
        let files = fs::read_dir(p)?;
        for entry in files {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if pred(path.as_path()) {
                    res.push(path);
                }
            } else if path.is_dir() {
                _rec_files(path, pred, res)?;
            }
        }

        Ok(())
    }

    let mut res = Vec::new();
    _rec_files(root_dir, predicate.as_ref(), &mut res)?;

    Ok(res)
}
