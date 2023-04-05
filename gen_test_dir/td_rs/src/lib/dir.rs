//! dir module.
use std::{fs, io};

use chrono::prelude::*;
use thiserror::Error;

/// The prefix string.
const PREFIX_STR: &str = "t";

/// generate a dir.
pub fn gen_dir_str() -> String {
    let now = Local::now();
    let s = format!("{}", now.format("%m%d"));
    let s = if s.starts_with('0') {
        match s.strip_prefix('0') {
            Some(r) => r.to_string(),
            None => unreachable!(),
        }
        //s[1..].to_string()
    } else {
        s
    };
    format!("{PREFIX_STR}{s}")
}

/// DirCreate
pub trait DirCreate {
    /// create dir from self
    fn create(&self) -> Result<(), DirCreateError>;
}

impl DirCreate for String {
    fn create(&self) -> Result<(), DirCreateError> {
        fs::create_dir(self).map_err(|e| DirCreateError::Io { dir: self.clone(), source: e })
    }
}

/// A directory create error type.
#[derive(Error, Debug)]
pub enum DirCreateError {
    /// Io error.
    #[error("create dir '{dir}' error, caused by '{source}'")]
    Io {
        /// dir
        dir: String,
        /// source.
        source: io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::{gen_dir_str, PREFIX_STR};

    #[test]
    fn test_gen_dir_str() {
        let d = gen_dir_str();
        assert!(d.starts_with(PREFIX_STR));
    }
}
