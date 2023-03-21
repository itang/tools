#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! td lib

use std::fs;

use chrono::prelude::*;

/// generate a dir.
pub fn gen_dir_str() -> String {
    let now = Local::now();
    let s = format!("{}", now.format("%m%d"));
    if s.starts_with('0') {
        match s.strip_prefix('0') {
            Some(r) => r.to_string(),
            None => unreachable!(),
        }
        //s[1..].to_string()
    } else {
        s
    }
}

/// DirCreate
pub trait DirCreate {
    /// create dir from self
    fn create(&self) -> bool;
}

impl DirCreate for String {
    fn create(&self) -> bool {
        matches!(fs::create_dir(self), Ok(()))
    }
}
