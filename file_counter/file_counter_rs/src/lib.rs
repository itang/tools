#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate ifile_counter.
//!
//!

use anyhow::Result;
use std::path::{Path, PathBuf};

/// get all files in some dir with predicate
pub fn files(root_dir: impl AsRef<Path>, predicate: Box<dyn Fn(&Path) -> bool>) -> Result<Vec<PathBuf>> {
    fn nest<S: AsRef<Path>>(p: S, pred: &dyn Fn(&Path) -> bool, res: &mut Vec<PathBuf>) -> Result<()> {
        let files = std::fs::read_dir(p)?;
        for entry in files {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if pred(path.as_path()) {
                    //处理文件
                    res.push(path);
                }
            } else if path.is_dir() {
                nest(path, pred, res)?;
            }
        }

        Ok(())
    }

    let mut res = Vec::new();
    nest(root_dir, predicate.as_ref(), &mut res)?;

    Ok(res)
}
