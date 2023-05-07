//! jy_rs.
//!
//! jy lib.
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use anyhow::Result;
use std::path::PathBuf;

use std::fs;

pub mod browser;
pub mod opt;

/// get content from path
pub fn get_content(config_path: PathBuf) -> Result<String> {
    println!("Read config from {config_path:?}");

    Ok(fs::read_to_string(config_path)?)
}
