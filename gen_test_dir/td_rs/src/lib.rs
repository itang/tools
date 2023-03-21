#![deny(clippy::unwrap_used)]
//#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![feature(provide_any)]
#![feature(error_generic_member_access)]

//! td lib

use std::{fs, io};

use chrono::prelude::*;
use thiserror::Error;

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
    fn create(&self) -> Result<(), DirCreateError>;
}

impl DirCreate for String {
    fn create(&self) -> Result<(), DirCreateError> {
        fs::create_dir(self).map_err(|e| DirCreateError::Io {
            dir: self.clone(),
            source: e,
        })
    }
}

/// A directory create error type.
#[derive(Error, Debug)]
pub enum DirCreateError {
    /// Io error.
    #[error("create dir '{dir}' error, caused by {source}")]
    Io {
        /// dir
        dir: String,
        /// source.
        source: io::Error,
    },
}
